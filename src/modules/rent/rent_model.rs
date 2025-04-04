use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::InputValidation;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reserva {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub quadra_id: i32,
    pub inicio: NaiveDateTime,      // Full timestamp for start
    pub fim: NaiveDateTime,         // Full timestamp for end
    pub status_id: String,          // e.g.: "Aberto", "Reservado", etc.
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
    pub status_id: Option<String>,
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
