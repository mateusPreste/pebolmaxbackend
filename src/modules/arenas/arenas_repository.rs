use chrono::{ Datelike, NaiveDate, NaiveDateTime, NaiveTime };
use rust_decimal::{ prelude::{ FromPrimitive, ToPrimitive }, Decimal };
use tokio_postgres::{ Client, Error, Row };

// Removido: use crate::modules::arenas::arenas_model::Local;
// Adicionado AsyncTryFromRow
use super::arenas_model::{ AsyncTryFromRow, Estabelecimento, Horario, Local, Quadra };

/// Busca quadras associadas a um local_id.
// Esta função agora é pública pois é usada pela implementação da trait em arenas_model.rs
pub async fn find_quadras_by_local_id(
    client: &tokio_postgres::Client,
    local_id: i32
) -> Result<Vec<Quadra>, String> {
    let quadras_query = "SELECT id, nome, local_id, photo_url FROM quadras WHERE local_id = $1";
    let quadras_rows = client
        .query(quadras_query, &[&local_id]).await
        .map_err(|e| format!("Erro ao buscar quadras para o local {}: {}", local_id, e))?;

    let quadras = quadras_rows
        .into_iter()
        .map(|quadra_row| Quadra {
            id: Some(quadra_row.get("id")),
            nome: quadra_row.get("nome"),
            local_id: quadra_row.get("local_id"),
            photo_url: quadra_row.get("photo_url"),
        })
        .collect();
    Ok(quadras)
}

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
        let latitude = local.latitude;
        let longitude = local.longitude;

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

            // Mapeia cada linha de local para a struct Local usando a trait
            let mut locais = Vec::new(); // vetor para colocar os locais

            for local_row in locais_rows {
                // Usa a trait AsyncTryFromRow
                locais.push(Local::try_from_row(client, local_row).await?);
            }

            let estabelecimento = Estabelecimento {
                id: Some(estabelecimento_id),
                nome: row.get("nome"),
                tax_id: row.get("tax_id"),
                tipo: row.get("tipo"),
                pais: row.get("pais"),
                locais, // Agora contém locais com suas quadras
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

    //vector to show the estabelecimentos we fiiind
    let mut estabelecimentos = Vec::new();

    for row in estabelecimentos_rows {
        let estabelecimento_id: i32 = row.get("id");

        // Buscar locais associados ao estabelecimento
        let locais_query =
            "SELECT id, nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude FROM locais WHERE estabelecimento_id = $1";
        let locais_rows = client
            .query(locais_query, &[&estabelecimento_id]).await
            .map_err(|e| format!("Erro ao buscar locais: {}", e))?;

        // Mapeia cada linha de local para a struct Local usando a trait
        let mut locais = Vec::new();
        for local_row in locais_rows {
            // Usa a trait AsyncTryFromRow
            locais.push(Local::try_from_row(client, local_row).await?);
        }

        // coloca
        estabelecimentos.push(Estabelecimento {
            id: Some(estabelecimento_id),
            nome: row.get("nome"),
            tax_id: row.get("tax_id"),
            tipo: row.get("tipo"),
            pais: row.get("pais"),
            locais, // Agora contém locais com suas quadras
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

pub async fn delete_local_by_id(client: &Client, id: i32) -> Result<(), String> {
    println!("Deletando local com ID: {}", id);

    let query = "DELETE FROM locais WHERE id = $1";

    match client.execute(query, &[&id]).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("Nenhum locais encontrado para o ID: {}", id);
                Err(format!("Nenhum locais encontrado para o ID: {}", id))
            } else {
                println!("Local com ID {} deletado com sucesso.", id);
                Ok(())
            }
        }
        Err(_err) => {
            println!("Erro ao deletar local: {}", _err);
            Err(format!("Erro ao deletar local: {}", _err))
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

    match
        client.execute(
            query,
            &[
                &estabelecimento.nome,
                &estabelecimento.tax_id,
                &estabelecimento.tipo,
                &estabelecimento.pais,
                &id,
            ]
        ).await
    {
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

pub async fn find_locais_by_estabelecimento_id(
    client: &tokio_postgres::Client,
    estabelecimento_id: i32
) -> Result<Vec<Local>, String> {
    println!("Buscando locais para o estabelecimento com ID: {}", estabelecimento_id);

    let query =
        "
        SELECT id, nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude
        FROM locais
        WHERE estabelecimento_id = $1
    ";

    match client.query(query, &[&estabelecimento_id]).await {
        Ok(rows) => {
            // Mapeia cada linha de local para a struct Local usando a trait
            let mut locais = Vec::new();
            for row in rows {
                // Usa a trait AsyncTryFromRow
                locais.push(Local::try_from_row(client, row).await?);
            }
            Ok(locais)
        }
        Err(err) => {
            println!("Erro ao buscar locais: {}", err);
            Err(format!("Erro ao buscar locais: {}", err))
        }
    }
}

pub async fn find_local_by_id(client: &Client, local_id: i32) -> Result<Option<Local>, String> {
    println!("Buscando local com ID: {}", local_id);

    let query =
        "
        SELECT id, nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude
        FROM locais
        WHERE id = $1
    ";

    match client.query_opt(query, &[&local_id]).await {
        Ok(Some(row)) => {
            // Converte a linha para Local usando a trait
            let local = Local::try_from_row(client, row).await?;
            Ok(Some(local))
        }
        Ok(None) => {
            println!("Nenhum local encontrado com o ID: {}", local_id);
            Ok(None)
        }
        Err(err) => {
            println!("Erro ao buscar local: {}", err);
            Err(format!("Erro ao buscar local: {}", err))
        }
    }
}

pub async fn update_local(client: &Client, local_id: i32, local: Local) -> Result<Local, String> {
    println!("Atualizando local com ID: {}", local_id);

    // A query de atualização foca apenas nos campos da tabela 'locais'.
    // O campo 'quadras' da struct 'local' recebida não é usado aqui.
    let query =
        "
    UPDATE locais
    SET nome = $1, rua = $2, numero = $3, complemento = $4, bairro = $5, cidade = $6, estado = $7, codigo_postal = $8, country = $9, latitude = $10, longitude = $11
    WHERE id = $12
";

    match
        client.execute(
            query,
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
                &local.latitude, // Mantém Decimal
                &local.longitude, // Mantém Decimal
                &local_id,
            ]
        ).await
    {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("Nenhum local encontrado para o ID: {}", local_id);
                Err(format!("Nenhum local encontrado para o ID: {}", local_id))
            } else {
                println!("Local com ID {} atualizado com sucesso.", local_id);
                // Busca o local atualizado (incluindo as quadras) para retornar
                match find_local_by_id(client, local_id).await? {
                    Some(updated_local) => Ok(updated_local),
                    None => Err(format!("Local with ID {} not found after update", local_id)),
                }
            }
        }
        Err(err) => {
            println!("Erro ao atualizar local: {}", err);
            Err(format!("Erro ao atualizar local: {}", err))
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



/// Busca uma quadra pelo ID.
pub async fn find_quadra_by_id(client: &Client, quadra_id: i32) -> Result<Option<Quadra>, String> {
    let query =
        "
        SELECT id, nome, local_id, photo_url
        FROM quadras
        WHERE id = $1
    ";

    match client.query_opt(query, &[&quadra_id]).await {
        Ok(Some(row)) => {
            let quadra = Quadra {
                id: Some(row.get("id")),
                nome: row.get("nome"),
                local_id: row.get("local_id"),
                photo_url: row.get("photo_url"),
            };
            Ok(Some(quadra))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(format!("Erro ao buscar quadra: {}", err)),
    }
}

/// Atualiza uma quadra pelo ID.
pub async fn update_quadra(
    client: &Client,
    quadra_id: i32,
    quadra: Quadra
) -> Result<Quadra, String> {
    let query =
        "
        UPDATE quadras
        SET nome = $1, local_id = $2, photo_url = $3
        WHERE id = $4
    ";

    match
        client.execute(
            query,
            &[&quadra.nome, &quadra.local_id, &quadra.photo_url, &quadra_id]
        ).await
    {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                Err(format!("Nenhuma quadra encontrada para o ID: {}", quadra_id))
            } else {
                // Retorna a quadra atualizada buscando-a novamente
                find_quadra_by_id(client, quadra_id).await?.ok_or_else(|| {
                    format!("Quadra com ID {} não encontrada após atualização", quadra_id)
                })
            }
        }
        Err(err) => Err(format!("Erro ao atualizar quadra: {}", err)),
    }
}

/// Busca horários associados a uma quadra pelo ID.
pub async fn find_horarios_by_quadra_id(
    client: &Client,
    quadra_id: i32
) -> Result<Vec<Horario>, String> {
    let query =
        "
        SELECT id, quadra_id, dia_semana, horario_inicio, horario_fim
        FROM quadras_horarios
        WHERE quadra_id = $1
    ";

    match client.query(query, &[&quadra_id]).await {
        Ok(rows) => {
            let horarios = rows
                .into_iter()
                .map(|row| Horario {
                    dia_semana: row.get("dia_semana"),
                    horario_inicio: row.get("horario_inicio"),
                    horario_fim: row.get("horario_fim"),
                })
                .collect();
            Ok(horarios)
        }
        Err(err) => Err(format!("Erro ao buscar horários: {}", err)),
    }
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
