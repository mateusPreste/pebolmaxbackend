use chrono::{ Datelike, NaiveDate, NaiveDateTime, NaiveTime };
use rust_decimal::{ prelude::{ FromPrimitive, ToPrimitive }, Decimal };
use tokio_postgres::{ Client, Error };

use crate::modules::arenas::arenas_model::Local;

use super::arenas_model::{ Estabelecimento, Horario, Quadra };

pub async fn create_estabelecimento(
    client: &mut Client,
    mut est: Estabelecimento
) -> Result<Estabelecimento, String> {
    // Start a transaction.
    let transaction = client.transaction().await.map_err(|e| e.to_string())?;

    // Inserir estabelecimento
    let est_stmt =
        "INSERT INTO estabelecimentos (nome, tax_id, tipo, pais) VALUES ($1, $2, $3, $4) RETURNING id";
    let row = transaction
        .query_one(est_stmt, &[&est.nome, &est.tax_id, &est.tipo, &est.pais]).await
        .map_err(|e| e.to_string())?;
    let est_id: i32 = row.get("id");
    est.id = Some(est_id);

    // Verificar se há ao menos um local
    if est.locais.is_empty() {
        return Err("Pelo menos um local deve ser informado".into());
    }

    // Inserir cada local vinculado ao estabelecimento
    let local_stmt =
        "INSERT INTO locais (nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude, estabelecimento_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id";

    for local in est.locais.iter_mut() {
        let latitude = Decimal::from_f64(local.latitude).ok_or(
            "Failed to convert latitude to Decimal"
        )?;
        let longitude = Decimal::from_f64(local.longitude).ok_or(
            "Failed to convert longitude to Decimal"
        )?;

        let row = transaction
            .query_one(
                local_stmt,
                &[
                    &local.nome,
                    &local.rua,
                    &local.numero,
                    &local.complemento,
                    &local.bairro,
                    &local.cidade,
                    &local.estado,
                    &local.codigo_postal,
                    &local.country,
                    &latitude,
                    &longitude,
                    &est_id,
                ]
            ).await
            .map_err(|e| e.to_string())?;
        local.id = Some(row.get("id"));
    }

    // Commit the transaction; if commit fails, whole transaction is rolled back.
    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(est)
}

//search in the db the database where the id was request
pub async fn find_estabelecimento_by_id(
    client: &tokio_postgres::Client,
    id: i32
) -> Result<Option<Estabelecimento>, String> {
    println!("Buscando estabelecimento com ID: {}", id);

    // Query para buscar o estabelecimento pelo ID
    let query =
        "
        SELECT id, nome, tax_id, tipo, pais 
        FROM estabelecimentos 
        WHERE id = $1
    ";

    match client.query_opt(query, &[&id]).await {
        Ok(Some(row)) => {
            let estabelecimento_id: i32 = row.get("id");

            // Buscar locais associados ao estabelecimento
            let locais_query =
                "
                SELECT id, nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude 
                FROM locais 
                WHERE estabelecimento_id = $1
            ";
            let locais_rows = client
                .query(locais_query, &[&estabelecimento_id]).await
                .map_err(|e| format!("Erro ao buscar locais: {}", e))?;

            let locais = locais_rows
                .into_iter()
                .map(|local_row| Local {
                    id: Some(local_row.get("id")),
                    nome: local_row.get("nome"),
                    rua: local_row.get("rua"),
                    numero: local_row.get("numero"),
                    complemento: local_row.get("complemento"),
                    bairro: local_row.get("bairro"),
                    cidade: local_row.get("cidade"),
                    estado: local_row.get("estado"),
                    codigo_postal: local_row.get("codigo_postal"),
                    country: local_row.get("country"),
                    latitude: local_row.get::<_, Decimal>("latitude").to_f64().unwrap_or(0.0),
                    longitude: local_row.get::<_, Decimal>("longitude").to_f64().unwrap_or(0.0),
                })
                .collect();

            let estabelecimento = Estabelecimento {
                id: Some(estabelecimento_id),
                nome: row.get("nome"),
                tax_id: row.get("tax_id"),
                tipo: row.get("tipo"),
                pais: row.get("pais"),
                locais,
            };

            Ok(Some(estabelecimento))
        }
        Ok(None) => {
            println!("Nenhum estabelecimento encontrado para o ID: {}", id);
            Ok(None)
        }
        Err(err) => {
            println!("Erro ao buscar estabelecimento: {}", err);
            Err(format!("Erro ao buscar estabelecimento: {}", err))
        }
    }
}

pub async fn find_all_estabelecimentos(client: &Client) -> Result<Vec<Estabelecimento>, String> {
    let estabelecimentos_query = "SELECT id, nome, tax_id, tipo, pais FROM estabelecimentos";

    let estabelecimentos_rows = client
        .query(estabelecimentos_query, &[]).await
        .map_err(|e| format!("Erro ao buscar estabelecimentos: {}", e))?;

    let mut estabelecimentos = Vec::new();

    for row in estabelecimentos_rows {
        let estabelecimento_id: i32 = row.get("id");

        // Buscar locais associados ao estabelecimento
        let locais_query =
            "SELECT id, nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude FROM locais WHERE estabelecimento_id = $1";
        let locais_rows = client
            .query(locais_query, &[&estabelecimento_id]).await
            .map_err(|e| format!("Erro ao buscar locais: {}", e))?;

        let locais = locais_rows
            .into_iter()
            .map(|local_row| Local {
                id: Some(local_row.get("id")),
                nome: local_row.get("nome"),
                rua: local_row.get("rua"),
                numero: local_row.get("numero"),
                complemento: local_row.get("complemento"),
                bairro: local_row.get("bairro"),
                cidade: local_row.get("cidade"),
                estado: local_row.get("estado"),
                codigo_postal: local_row.get("codigo_postal"),
                country: local_row.get("country"),
                latitude: local_row.get::<_, Decimal>("latitude").to_f64().unwrap_or(0.0), // Conversão explícita para f64
                longitude: local_row.get::<_, Decimal>("longitude").to_f64().unwrap_or(0.0), // Conversão explícita para f64
            })
            .collect();

        estabelecimentos.push(Estabelecimento {
            id: Some(estabelecimento_id),
            nome: row.get("nome"),
            tax_id: row.get("tax_id"),
            tipo: row.get("tipo"),
            pais: row.get("pais"),
            locais, // Preenche o campo locais
        });
    }

    Ok(estabelecimentos)
}

pub async fn delete_estabelecimento_by_id(client: &Client, id: i32) -> Result<(), String> {
    println!("Deletando estabelecimento com ID: {}", id);

    let query = "DELETE FROM estabelecimentos WHERE id = $1";

    match client.execute(query, &[&id]).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("Nenhum estabelecimento encontrado para o ID: {}", id);
                Err(format!("Nenhum estabelecimento encontrado para o ID: {}", id))
            } else {
                println!("Estabelecimento com ID {} deletado com sucesso.", id);
                Ok(())
            }
        }
        Err(_err) => {
            println!("Erro ao deletar estabelecimento: {}", _err);
            Err(format!("Erro ao deletar estabelecimento: {}", _err))
        }
    }
}

pub async fn update_estabelecimento(
    client: &Client,
    id: i32,
    estabelecimento: Estabelecimento
) -> Result<Estabelecimento, String> {
    println!("Atualizando estabelecimento com ID: {}", id);

    let query =
        "
        UPDATE estabelecimentos
        SET nome = $1, tax_id = $2, tipo = $3, pais = $4
        WHERE id = $5
    ";

    match client.execute(
        query,
        &[
            &estabelecimento.nome,
            &estabelecimento.tax_id,
            &estabelecimento.tipo,
            &estabelecimento.pais,
            &id,
        ]
    ).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("Nenhum estabelecimento encontrado para o ID: {}", id);
                Err(format!("Nenhum estabelecimento encontrado para o ID: {}", id))
            } else {
                println!("Estabelecimento com ID {} atualizado com sucesso.", id);
                Ok(estabelecimento)
            }
        }
        Err(err) => {
            println!("Erro ao atualizar estabelecimento: {}", err);
            Err(format!("Erro ao atualizar estabelecimento: {}", err))
        }
    }
}

/// Registra uma nova quadra e insere seus horários em uma única transação.
/// Se qualquer inserção falhar, toda a operação é revertida.
///
/// Parâmetros:
/// - client: conexão mutável com o banco (tokio_postgres::Client)
/// - quadra: quadra a ser inserida (campo id será preenchido)
/// - horarios: vetor de Horário a ser inserido para a quadra
///
/// Retorna a quadra inserida (com id preenchido) ou um erro em forma de String.
pub async fn create_quadra(
    client: &mut Client,
    quadra: &Quadra,
    horarios: Vec<Horario>
) -> Result<Quadra, Error> {
    // Inicia uma transação
    let transaction = client.transaction().await?;

    // Insere a quadra e retorna o ID gerado.
    // Ajuste a consulta conforme as suas colunas e requisitos.
    let quadra_stmt =
        "INSERT INTO quadras (nome, local_id, photo_url) VALUES ($1, $2, $3) RETURNING id";
    let row = transaction.query_one(
        quadra_stmt,
        &[&quadra.nome, &quadra.local_id, &quadra.photo_url]
    ).await?;
    let quadra_id: i32 = row.get("id");

    // Insere cada horário vinculado à quadra.
    let horario_stmt =
        "\
        INSERT INTO quadras_horarios (quadra_id, dia_semana, horario_inicio, horario_fim) \
        VALUES ($1, $2, $3, $4)";
    for h in &horarios {
        transaction.execute(
            horario_stmt,
            &[&quadra_id, &h.dia_semana, &h.horario_inicio, &h.horario_fim]
        ).await?;
    }

    // Commit da transação.
    transaction.commit().await?;

    Ok(quadra.clone())
}

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
