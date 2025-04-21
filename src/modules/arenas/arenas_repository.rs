use chrono::{ Datelike, NaiveDate, NaiveDateTime, NaiveTime };
use rust_decimal::{ prelude::{ FromPrimitive, ToPrimitive }, Decimal };
use tokio_postgres::{ Client, Error, Row };

// Removido: use crate::modules::arenas::arenas_model::Local;
// Adicionado AsyncTryFromRow
use super::arenas_model::{ AsyncTryFromRow, Estabelecimento, Horario, Local, Quadra };



// Helper: Returns the day of week string in lowercase as expected (e.g., "segunda", "terca", etc.)
fn day_of_week_str(date: NaiveDate) -> String {
    // Adjust this mapping as needed to match your quadras_horarios "dia_semana" values.
    // For example, chrono returns: Monday, Tuesday, etc.
    match date.weekday().num_days_from_sunday() {
        0 => "domingo".to_string(),
        1 => "segunda".to_string(),
        2 => "terca".to_string(),
        3 => "quarta".to_string(),
        4 => "quinta".to_string(),
        5 => "sexta".to_string(),
        6 => "sabado".to_string(),
        _ => "domingo".to_string(),
    }
}

// This function returns free times as a vector of (NaiveTime, NaiveTime) tuples.
pub async fn list_free_times(
    client: &mut Client,
    quadra_id: i32,
    date: NaiveDate
) -> Result<Vec<(NaiveTime, NaiveTime)>, String> {
    // Determine the day-of-week string
    let dia_semana = day_of_week_str(date);

    // Query operating hours for the given quadra and day.
    let operating_query =
        r#"
        SELECT horario_inicio, horario_fim 
        FROM quadras_horarios 
        WHERE quadra_id = $1 AND LOWER(dia_semana) = $2
        ORDER BY horario_inicio
    "#;
    let op_rows = client
        .query(operating_query, &[&quadra_id, &dia_semana]).await
        .map_err(|e| format!("Error querying operating hours: {}", e))?;

    if op_rows.is_empty() {
        return Err(format!("No operating hours found for quadra {} on {}", quadra_id, dia_semana));
    }

    // Build a vector of operating intervals as full timestamps.
    let mut operating_intervals: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::new();
    for row in op_rows {
        let op_start: NaiveTime = row.get("horario_inicio");
        let op_end: NaiveTime = row.get("horario_fim");
        let start_dt = NaiveDateTime::new(date, op_start);
        // If op_end is less than or equal to op_start, assume the interval spans midnight.
        let end_dt = if op_end <= op_start {
            // Create end timestamp on day + 1.
            NaiveDateTime::new(date.succ_opt().ok_or("Failed to get next day")?, op_end)
        } else {
            NaiveDateTime::new(date, op_end)
        };
        operating_intervals.push((start_dt, end_dt));
    }

    // Now query all reservations (with status 'Reservado') for the given quadra
    // that overlap any operating interval on the given day.
    // We use the min operating start and max operating end as bounds.
    let global_start = operating_intervals
        .iter()
        .map(|(s, _)| *s)
        .min()
        .unwrap();
    let global_end = operating_intervals
        .iter()
        .map(|(_, e)| *e)
        .max()
        .unwrap();
    let reserved_query =
        r#"
        SELECT inicio, fim 
        FROM reservas
        WHERE quadra_id = $1
          AND status_id = 'Reservado'
          AND inicio < $2 AND fim > $3
        ORDER BY inicio
    "#;
    let res_rows = client
        .query(reserved_query, &[&quadra_id, &global_end, &global_start]).await
        .map_err(|e| format!("Error querying reservations: {}", e))?;

    let mut reserved_intervals: Vec<(NaiveDateTime, NaiveDateTime)> = res_rows
        .iter()
        .map(|row| {
            let inicio: NaiveDateTime = row.get("inicio");
            let fim: NaiveDateTime = row.get("fim");
            (inicio, fim)
        })
        .collect();

    // Sort and merge overlapping reserved intervals.
    reserved_intervals.sort_by_key(|(s, _)| *s);
    let mut merged_reservations: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::new();
    for interval in reserved_intervals {
        if let Some(last) = merged_reservations.last_mut() {
            if interval.0 <= last.1 {
                if interval.1 > last.1 {
                    last.1 = interval.1;
                }
            } else {
                merged_reservations.push(interval);
            }
        } else {
            merged_reservations.push(interval);
        }
    }

    // For each operating interval, subtract the reserved intervals that overlap and produce free intervals.
    let mut free_intervals = Vec::new();
    for (op_start, op_end) in operating_intervals {
        let mut current = op_start;
        for (res_start, res_end) in merged_reservations.iter() {
            // Only consider reservations that overlap this operating interval.
            if res_end <= &op_start || res_start >= &op_end {
                continue;
            }

            let interval_start = if *res_start < current { current } else { *res_start };
            if interval_start > current {
                free_intervals.push((current.time(), interval_start.time()));
            }
            if *res_end > current {
                current = *res_end;
            }
            if current >= op_end {
                break;
            }
        }
        if current < op_end {
            free_intervals.push((current.time(), op_end.time()));
        }
    }

    Ok(free_intervals)
}
