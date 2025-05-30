use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::InputValidation;

use super::validator::{
    estabelecimento_validator::validate_establishment, local_validator::validate_local_code,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalInput {
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
    pub latitude: f64,
    pub longitude: f64,
}

impl InputValidation for LocalInput {
    fn validate(&mut self) -> Result<(), String> {
        let local = Local {
            id: self.id,
            nome: self.nome.clone(),
            rua: self.rua.clone(),
            numero: self.numero.clone(),
            complemento: self.complemento.clone(),
            bairro: self.bairro.clone(),
            cidade: self.cidade.clone(),
            estado: self.estado.clone(),
            codigo_postal: self.codigo_postal.clone(),
            country: self.country.clone(),
            latitude: self.latitude,
            longitude: self.longitude,
            estabelecimento_id: None,
        };
        validate_local_code(&local)
    }
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
    pub latitude: f64,
    pub longitude: f64,
    pub estabelecimento_id: Option<i32>,
}

impl InputValidation for Local {
    fn validate(&mut self) -> Result<(), String> {
        validate_local_code(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Estabelecimento {
    pub id: Option<i32>,
    pub nome: String,
    pub tax_id: String,
    pub tipo: String, // deve ser "publico" ou "privado"
    pub pais: String,
    pub locais: Vec<LocalInput>,
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
    "domingo", "segunda", "terca", "quarta", "quinta", "sexta", "sabado",
];

// Exemplo de struct para Horário de Funcionamento.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Horario {
    pub dia_semana: String,
    pub horario_inicio: NaiveTime, // No formato "HH:MM", por exemplo.
    pub horario_fim: NaiveTime,    // No formato "HH:MM"
}

impl InputValidation for Horario {
    fn validate(&mut self) -> Result<(), String> {
        // dia_semana deve ser 'domingo', 'segunda', 'terca', 'quarta', 'quinta', 'sexta', 'sabado'

        if !DIAS_SEMANA.contains(&self.dia_semana.as_str()) {
            return Err("Dia da semana inválido".into());
        }

        let start_time: NaiveTime =
            NaiveTime::parse_from_str(&self.horario_inicio.to_string(), "%H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_inicio: {}", e))?;
        let end_time: NaiveTime =
            NaiveTime::parse_from_str(&self.horario_fim.to_string(), "%H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_fim: {}", e))?;

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
