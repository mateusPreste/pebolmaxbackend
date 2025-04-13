use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::InputValidation;

// Define the new enum for reserva status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Display)]
pub enum StatusReserva {
    Aberto,
    #[serde(rename = "Em Disputa")]
    #[strum(to_string = "Em Disputa")]
    EmDisputa,
    Cancelado,
    Perdido,
    Reservado,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reserva {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub service_id: i32,
    pub quadra_id: i32,
    pub inicio: NaiveDateTime,      // Full timestamp for start
    pub fim: NaiveDateTime,         // Full timestamp for end
    pub status_id: StatusReserva,   // Changed type to StatusReserva
    pub modalidade: Option<String>, // "usuarios" or "times"
    pub min_pagantes: i32,          // Minimum required number of paying participants
}

impl InputValidation for Reserva {
    fn validate(&mut self) -> Result<(), String> {
        let start_time: NaiveDateTime =
            NaiveDateTime::parse_from_str(&self.inicio.to_string(), "%Y-%m-%d %H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_inicio: {}", e))?;
        let end_time: NaiveDateTime =
            NaiveDateTime::parse_from_str(&self.fim.to_string(), "%Y-%m-%d %H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_fim: {}", e))?;

        self.inicio = start_time;
        self.fim = end_time;

        if start_time >= end_time {
            return Err("Horário de início deve ser anterior ao horário de fim".into());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservaInput {
    pub usuario_id: i32,
    pub quadra_id: i32,
    /// Reservation start in the format "YYYY-MM-DD HH:MM:SS"
    pub inicio: NaiveDateTime,
    /// Reservation end in the format "YYYY-MM-DD HH:MM:SS"
    pub fim: NaiveDateTime,
    /// Optional status, if not provided default "Aberto" will be used.
    pub status_id: Option<StatusReserva>, // Changed type to Option<StatusReserva>
    /// Optional modality (default "usuarios")
    pub modalidade: Option<String>,
    /// Optional minimum number of payers (default 0)
    pub min_pagantes: Option<i32>,
}

impl InputValidation for ReservaInput {
    fn validate(&mut self) -> Result<(), String> {
        let start_time: NaiveDateTime =
            NaiveDateTime::parse_from_str(&&self.inicio.to_string(), "%Y-%m-%d %H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_inicio: {}", e))?;
        let end_time: NaiveDateTime =
            NaiveDateTime::parse_from_str(&self.fim.to_string(), "%Y-%m-%d %H:%M:%S")
                .map_err(|e| format!("Erro ao converter horario_fim: {}", e))?;

        self.inicio = start_time;
        self.fim = end_time;

        if start_time >= end_time {
            return Err("Horário de início deve ser anterior ao horário de fim".into());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDTO {
    pub id: i32,
    pub nome: String,
    pub apelido: String,
    pub foto: Option<String>,
    pub reputacao: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuadraDTO {
    pub id: i32,
    pub nome: String,
    pub photo_url: Option<String>,
    pub local_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalDTO {
    pub id: i32,
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
    pub estabelecimento_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EstabelecimentoDTO {
    pub id: i32,
    pub nome: String,
    pub tax_id: String,
    pub tipo: String,
    pub pais: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservaDetalhesResponse {
    // Reservation data
    pub id: i32,
    pub inicio: NaiveDateTime,
    pub fim: NaiveDateTime,
    pub status_id: StatusReserva,
    pub modalidade: Option<String>,
    pub min_pagantes: i32,
    
    // Related data
    pub quadra: QuadraDTO,
    pub local: LocalDTO,
    pub estabelecimento: EstabelecimentoDTO,
    pub organizador: UserDTO,
    pub participantes: Vec<UserDTO>,
}
