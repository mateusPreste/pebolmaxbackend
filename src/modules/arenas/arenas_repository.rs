use chrono::NaiveTime;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use tokio_postgres::{Client, Error};

use super::arenas_model::{Estabelecimento, Horario, Quadra};

pub async fn create_estabelecimento(
    client: &mut Client,
    mut est: Estabelecimento,
) -> Result<Estabelecimento, String> {
    // Start a transaction.
    let transaction = client.transaction().await.map_err(|e| e.to_string())?;

    // Inserir estabelecimento
    let est_stmt = "INSERT INTO estabelecimentos (nome, tax_id, tipo, pais) VALUES ($1, $2, $3, $4) RETURNING id";
    let row = transaction
        .query_one(est_stmt, &[&est.nome, &est.tax_id, &est.tipo, &est.pais])
        .await
        .map_err(|e| e.to_string())?;
    let est_id: i32 = row.get("id");
    est.id = Some(est_id);

    // Verificar se há ao menos um local
    if est.locais.is_empty() {
        return Err("Pelo menos um local deve ser informado".into());
    }

    // Inserir cada local vinculado ao estabelecimento
    let local_stmt = "INSERT INTO locais (nome, rua, numero, complemento, bairro, cidade, estado, codigo_postal, country, latitude, longitude, estabelecimento_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id";

    for local in est.locais.iter_mut() {
        let latitude =
            Decimal::from_f64(local.latitude).ok_or("Failed to convert latitude to Decimal")?;
        let longitude =
            Decimal::from_f64(local.longitude).ok_or("Failed to convert longitude to Decimal")?;

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
                ],
            )
            .await
            .map_err(|e| e.to_string())?;
        local.id = Some(row.get("id"));
    }

    // Commit the transaction; if commit fails, whole transaction is rolled back.
    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(est)
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
    horarios: Vec<Horario>,
) -> Result<Quadra, Error> {
    // Inicia uma transação
    let transaction = client.transaction().await?;

    // Insere a quadra e retorna o ID gerado.
    // Ajuste a consulta conforme as suas colunas e requisitos.
    let quadra_stmt =
        "INSERT INTO quadras (nome, local_id, photo_url) VALUES ($1, $2, $3) RETURNING id";
    let row = transaction
        .query_one(
            quadra_stmt,
            &[&quadra.nome, &quadra.local_id, &quadra.photo_url],
        )
        .await?;
    let quadra_id: i32 = row.get("id");

    // Insere cada horário vinculado à quadra.
    let horario_stmt = "\
        INSERT INTO quadras_horarios (quadra_id, dia_semana, horario_inicio, horario_fim) \
        VALUES ($1, $2, $3, $4)";
    for h in &horarios {
        transaction
            .execute(
                horario_stmt,
                &[&quadra_id, &h.dia_semana, &h.horario_inicio, &h.horario_fim],
            )
            .await?;
    }

    // Commit da transação.
    transaction.commit().await?;

    Ok(quadra.clone())
}
