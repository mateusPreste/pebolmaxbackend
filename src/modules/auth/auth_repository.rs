use tokio_postgres::{Client, Error};

use super::auth_model::{Credenciais, Usuario, NivelNome};

pub async fn find_user_and_credentials_by_oauth(
    client: &Client,
    oauth_provider_id: &str,
) -> Result<Option<(Usuario, Credenciais)>, String> {
    let query = r#"
        SELECT u.*, c.*
        FROM credenciais c
        JOIN usuarios u ON c.usuario_id = u.id
        WHERE c.oauth_provider_id = $1
    "#;

    let rows = client
        .query(query, &[&oauth_provider_id])
        .await
        .map_err(|e| e.to_string())?;

    if rows.is_empty() {
        Ok(None)
    } else {
        let row = &rows[0];
        let usuario = Usuario::from_row(row);
        let credenciais = Credenciais::from_row(row);
        println!("usuario: {:?}", usuario);
        println!("credenciais: {:?}", credenciais);
        Ok(Some((usuario, credenciais)))
    }
}

pub async fn create_new_user_and_credentials(
    client: &Client,
    nome: &str,
    cpf: &str,
    apelido: &str,
    foto: Option<&str>,
    reputacao: i32,
    email: &str,
    phone_number: Option<&str>,
    oauth_provider: Option<&str>,
    oauth_provider_id: Option<&str>,
    password_hash: Option<&str>,
) -> Result<(Usuario, Credenciais), String> {
    // print all the parameters
    println!("nome: {}", nome);
    println!("cpf: {}", cpf);
    println!("apelido: {}", apelido);
    println!("foto: {:?}", foto);
    println!("reputacao: {}", reputacao);
    println!("email: {}", email);
    println!("phone_number: {:?}", phone_number);
    println!("oauth_provider: {:?}", oauth_provider);
    println!("oauth_provider_id: {:?}", oauth_provider_id);
    println!("password_hash: {:?}", password_hash);

    // If needed, also include created_at/updated_at if you plan to override defaults.
    let query = r#"WITH new_usuario AS (
  INSERT INTO usuarios (nome, cpf, apelido, foto, reputacao)
  VALUES ($1, $2, $3, $4, $5)
  RETURNING id, nome, cpf, apelido, foto, reputacao
)
INSERT INTO credenciais (
  usuario_id,
  email,
  email_verified,
  phone_number,
  phone_verified,
  oauth_provider,
  oauth_provider_id,
  password_hash,
  created_at,
  updated_at
)
SELECT 
  new_usuario.id,
  $6,           -- email
  FALSE,        -- email_verified
  $7,           -- phone_number
  FALSE,        -- phone_verified
  $8,           -- oauth_provider
  $9,           -- oauth_provider_id
  $10,          -- password_hash
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
FROM new_usuario;
  "#;

    client
        .execute(
            query,
            &[
                &nome,
                &cpf,
                &apelido,
                &foto,
                &reputacao,
                &email,
                &phone_number,
                &oauth_provider,
                &oauth_provider_id,
                &password_hash,
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

    let a = match oauth_provider_id {
        Some(id) => match find_user_and_credentials_by_oauth(client, id).await {
            Ok(Some(query)) => Ok(Some(query)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        },
        None => Ok(None), // Handle the case where there's no OAuth provider ID
    };

    Ok(a.unwrap().unwrap())
}

pub async fn add_user_role(
    client: &Client,
    usuario_id: i32,
    nivel_id: i32,
) -> Result<(), String> {
    let query = r#"
        INSERT INTO usuario_niveis (usuario_id, niveis_id)
        VALUES ($1, $2)
    "#;

    client
        .execute(query, &[&usuario_id, &nivel_id])
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_nivel_id_by_name(
    client: &Client,
    nome: NivelNome,
) -> Result<i32, String> {
    let query = r#"
        SELECT id FROM niveis WHERE nome = $1
    "#;

    let nome_str = nome.to_string();

    let row = client
        .query_one(query, &[&nome_str])
        .await
        .map_err(|e| e.to_string())?;

    Ok(row.get(0))
}
