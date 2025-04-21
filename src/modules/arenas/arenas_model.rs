use async_trait::async_trait;
use chrono::NaiveTime;
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use tokio_postgres::{ Client, Row };

use crate::InputValidation;

use super::{local::repository::find_quadras_by_local_id, validator::{
        estabelecimento_validator::validate_establishment,
        local_validator::validate_local_code,
    }};

/// Trait para conversão assíncrona de uma linha de banco de dados.
#[async_trait]
pub trait AsyncTryFromRow: Sized {
    /// Tenta converter uma linha de banco de dados para Self de forma assíncrona.
    /// Requer uma conexão com o banco para buscar dados relacionados, se necessário.
    async fn try_from_row(client: &Client, row: Row) -> Result<Self, String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Local {
    pub id: Option<i32>,
    pub nome: String,
    pub rua: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub codigo_postal: String,
    pub country: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub quadras: Vec<Quadra>,
}

impl InputValidation for Local {
    fn validate(&mut self) -> Result<(), String> {
        validate_local_code(self)
    }
}

#[async_trait]
impl AsyncTryFromRow for Local {
    async fn try_from_row(client: &Client, row: Row) -> Result<Self, String> {
        let local_id: i32 = row.get("id");
        let quadras = find_quadras_by_local_id(client, local_id).await?;
        Ok(Local {
            id: Some(local_id),
            nome: row.get("nome"),
            rua: row.get("rua"),
            numero: row.get("numero"),
            complemento: row.get("complemento"),
            bairro: row.get("bairro"),
            cidade: row.get("cidade"),
            estado: row.get("estado"),
            codigo_postal: row.get("codigo_postal"),
            country: row.get("country"),
            latitude: row.get::<_, Decimal>("latitude"),
            longitude: row.get::<_, Decimal>("longitude"),
            quadras, // Preenche com as quadras buscadas
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Estabelecimento {
    pub id: Option<i32>,
    pub nome: String,
    pub tax_id: String,
    pub tipo: String, // deve ser "publico" ou "privado"
    pub pais: String,
    pub locais: Vec<Local>,
}

impl InputValidation for Estabelecimento {
    fn validate(&mut self) -> Result<(), String> {
        if self.tipo != "publico" && self.tipo != "privado" {
            return Err("Tipo de estabelecimento inválido".into());
        }

        validate_establishment(self)?;

        for local in self.locais.iter_mut() {
            local.validate()?;
        }
        Ok(())
    }
}

// Exemplo de struct para Quadra.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quadra {
    pub id: Option<i32>,
    pub nome: String,
    // Supondo que cada quadra esteja associada a um local.
    pub photo_url: Option<String>,
    pub local_id: i32,
}

impl InputValidation for Quadra {
    fn validate(&mut self) -> Result<(), String> {
        if self.nome.is_empty() {
            return Err("Nome da quadra não pode ser vazio".into());
        }
        Ok(())
    }
}

static DIAS_SEMANA: &[&str] = &[
    "domingo",
    "segunda",
    "terca",
    "quarta",
    "quinta",
    "sexta",
    "sabado",
];

// Exemplo de struct para Horário de Funcionamento.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Horario {
    pub dia_semana: String,
    pub horario_inicio: NaiveTime, // No formato "HH:MM", por exemplo.
    pub horario_fim: NaiveTime, // No formato "HH:MM"
}

impl InputValidation for Horario {
    fn validate(&mut self) -> Result<(), String> {
        // dia_semana deve ser 'domingo', 'segunda', 'terca', 'quarta', 'quinta', 'sexta', 'sabado'

        println!("Validando dia_semana: {}", self.dia_semana);

        if !DIAS_SEMANA.contains(&self.dia_semana.as_str()) {
            println!("Valores válidos: {:?}", DIAS_SEMANA);

            return Err("Dia da semana inválido".into());
        }

        let start_time: NaiveTime = NaiveTime::parse_from_str(
            &self.horario_inicio.to_string(),
            "%H:%M:%S"
        ).map_err(|e| format!("Erro ao converter horario_inicio: {}", e))?;
        let end_time: NaiveTime = NaiveTime::parse_from_str(
            &self.horario_fim.to_string(),
            "%H:%M:%S"
        ).map_err(|e| format!("Erro ao converter horario_fim: {}", e))?;

        self.horario_inicio = start_time;
        self.horario_fim = end_time;

        if start_time >= end_time {
            return Err("Horário de início deve ser anterior ao horário de fim".into());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterQuadraInput {
    pub quadra: Quadra,
    pub horarios: Vec<Horario>,
}

impl InputValidation for RegisterQuadraInput {
    fn validate(&mut self) -> Result<(), String> {
        self.quadra.validate()?;
        for horario in self.horarios.iter_mut() {
            horario.validate()?;
        }

        for dia in DIAS_SEMANA.iter() {
            if !self.horarios.iter().any(|h| h.dia_semana == *dia) {
                return Err(format!("Horário para {} não informado", dia));
            }
        }

        Ok(())
    }
}
