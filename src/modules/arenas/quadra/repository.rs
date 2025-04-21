use tokio_postgres::{ Client, Error };
use crate::modules::arenas::arenas_model::{ Quadra, Horario };

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

//TODO retornar todas as quadras pelo id de um local

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

/// Deleta uma quadra pelo ID.
pub async fn delete_quadra(client: &Client, quadra_id: i32) -> Result<String, String> {
    let sql = "
    DELETE from quadras 
    WHERE id = $1
    ";

    match client.execute(sql, &[&quadra_id]).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                Err(format!("Nenhuma quadra encontrada para o ID: {}", quadra_id))
            } else {
                Ok(format!("Quadra com ID {} deletada com sucesso!", quadra_id))
            }
        }
        Err(err) => Err(format!("Erro ao deletar quadra: {}", err)),
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
