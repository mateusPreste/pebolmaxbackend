use crate::modules::arenas::arenas_model::{AsyncTryFromRow, Local, Quadra};
use tokio_postgres::Client;

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
