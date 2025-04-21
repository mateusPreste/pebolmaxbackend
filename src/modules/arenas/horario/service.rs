use crate::modules::arenas::arenas_model::{ Horario, DIAS_SEMANA, InputValidation }; // Adicionado InputValidation
use tokio_postgres::Client;

use super::repository::{
    delete_all_horarios_for_quadra, // Adicionado delete_all_horarios_for_quadra
    delete_single_horario, // Adicionado delete_single_horario
    find_horarios_by_quadra_id,
    update_horarios_for_quadra,
    update_single_horario, // Adicionado update_single_horario
};

pub async fn find_horarios_by_quadra_id_service(
    client: &mut Client,
    id: i32
) -> Result<Vec<Horario>, String> {
    find_horarios_by_quadra_id(client, id).await
}

pub async fn update_horarios_service(
    client: &mut Client,
    quadra_id: i32,
    horarios: &Vec<Horario> // Recebe referência
) -> Result<(), String> {
    // Validação 1: Garantir que todos os dias da semana estão presentes
    for dia in DIAS_SEMANA.iter() {
        if !horarios.iter().any(|h| h.dia_semana == *dia) {
            return Err(format!("Horário para {} não informado", dia));
        }
    }
    // Validação 2: (Já feita no controller, mas pode ser reforçada aqui se necessário)
    // for horario in horarios {
    //     horario.validate()?; // Assume que validate retorna Result<(), String>
    // }

    // Chama o repositório para efetuar a atualização
    update_horarios_for_quadra(client, quadra_id, horarios).await.map_err(|e| e.to_string())
}

pub async fn update_single_horario_service(
    client: &Client, // Não precisa ser mutável
    quadra_id: i32,
    mut horario: Horario // Torna mutável para validar e potencialmente ajustar
) -> Result<(), String> {
    // Valida o dia da semana recebido (já deve estar no Horario)
    if !DIAS_SEMANA.contains(&horario.dia_semana.as_str()) {
        return Err("Dia da semana inválido".into());
    }

    // Valida os tempos (inicio < fim) usando o método validate
    horario.validate()?; // Chama a validação existente no Horario

    // Chama o repositório para efetuar a atualização
    match update_single_horario(client, quadra_id, &horario).await {
        Ok(0) =>
            Err(
                format!(
                    "Nenhum horário encontrado para quadra {} e dia {}",
                    quadra_id,
                    horario.dia_semana
                )
            ), // Nenhum registro foi atualizado
        Ok(_) => Ok(()), // Pelo menos um registro foi atualizado
        Err(e) => Err(e.to_string()), // Erro do banco
    }
}

pub async fn delete_single_horario_service(
    client: &Client,
    quadra_id: i32,
    dia_semana: &str
) -> Result<(), String> {
    // Valida o dia da semana
    if !DIAS_SEMANA.contains(&dia_semana) {
        return Err("Dia da semana inválido".into());
    }

    match delete_single_horario(client, quadra_id, dia_semana).await {
        Ok(0) =>
            Err(
                format!("Nenhum horário encontrado para quadra {} e dia {}", quadra_id, dia_semana)
            ), // Nenhum registro foi deletado
        Ok(_) => Ok(()), // Pelo menos um registro foi deletado
        Err(e) => Err(e.to_string()), // Erro do banco
    }
}

pub async fn delete_all_horarios_service(client: &Client, quadra_id: i32) -> Result<(), String> {


    match delete_all_horarios_for_quadra(client, quadra_id).await {
        // Mesmo que 0 linhas sejam afetadas (quadra não tinha horários), consideramos sucesso.
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()), // Erro do banco
    }
}
