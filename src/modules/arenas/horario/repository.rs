use tokio_postgres::{ Client, Error };

use crate::modules::arenas::arenas_model::Horario;

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

/// Atualiza (substitui) todos os horários de uma quadra específica.
/// Operação realizada dentro de uma transação.
pub async fn update_horarios_for_quadra(
    client: &mut Client,
    quadra_id: i32,
    horarios: &Vec<Horario>
) -> Result<(), Error> {
    let transaction = client.transaction().await?;

    let delete_stmt = "DELETE FROM quadras_horarios WHERE quadra_id = $1";
    transaction.execute(delete_stmt, &[&quadra_id]).await?;

    // 2. Inserir os novos horários
    let insert_stmt =
        "INSERT INTO quadras_horarios (quadra_id, dia_semana, horario_inicio, horario_fim) VALUES ($1, $2, $3, $4)";
    for h in horarios {
        transaction.execute(
            insert_stmt,
            &[&quadra_id, &h.dia_semana, &h.horario_inicio, &h.horario_fim]
        ).await?;
    }

    transaction.commit().await?;

    Ok(())
}

/// Atualiza o horário de um dia específico para uma quadra.
pub async fn update_single_horario(
    client: &Client,
    quadra_id: i32,
    horario: &Horario
) -> Result<u64, Error> {
    let sql =
        "UPDATE quadras_horarios SET horario_inicio = $1, horario_fim = $2 WHERE quadra_id = $3 AND dia_semana = $4";
    client.execute(
        sql,
        &[&horario.horario_inicio, &horario.horario_fim, &quadra_id, &horario.dia_semana]
    ).await
}

/// Deleta o horário de um dia específico para uma quadra.
pub async fn delete_single_horario(
    client: &Client,
    quadra_id: i32,
    dia_semana: &str
) -> Result<u64, Error> {
    let sql = "DELETE FROM quadras_horarios WHERE quadra_id = $1 AND dia_semana = $2";
    client.execute(sql, &[&quadra_id, &dia_semana]).await
}

/// Deleta TODOS os horários de funcionamento de uma quadra específica.
pub async fn delete_all_horarios_for_quadra(client: &Client, quadra_id: i32) -> Result<u64, Error> {
    let sql = "DELETE FROM quadras_horarios WHERE quadra_id = $1";
    client.execute(sql, &[&quadra_id]).await
}
