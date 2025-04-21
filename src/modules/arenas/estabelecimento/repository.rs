use tokio_postgres::Client;

use crate::modules::arenas::arenas_model::{ AsyncTryFromRow, Estabelecimento, Local }; // Corrigido

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
