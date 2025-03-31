use crate::modules::arenas::arenas_model::Local;
use regex::Regex;

/// Trait que define a validação de código postal.
pub trait PostalCodeValidator {
    fn validate_postal_code(&self, code: &str) -> bool;
}

/// Implementação para o Brasil – valida CEP no formato "XXXXX-XXX" ou "XXXXXXXX".
pub struct BrazilPostalCodeValidator;

impl PostalCodeValidator for BrazilPostalCodeValidator {
    fn validate_postal_code(&self, code: &str) -> bool {
        let re = Regex::new(r"^\d{5}-?\d{3}$").unwrap();
        re.is_match(code)
    }
}

/// Função fábrica que retorna o validador adequado para o país.
pub fn postal_validator_for(country: &str) -> Result<Box<dyn PostalCodeValidator>, String> {
    match country.to_lowercase().as_str() {
        "brasil" => Ok(Box::new(BrazilPostalCodeValidator)),
        _ => Err(format!(
            "País não identificado para validação de código postal: {}",
            country
        )),
    }
}

/// Valida o campo `codigo_postal` do local.
/// Retorna Ok(()) se estiver no formato esperado para o país do local,
/// ou um Err com mensagem de erro caso contrário.
pub fn validate_local_code(local: &Local) -> Result<(), String> {
    let validator = postal_validator_for(&local.country)?;
    if !validator.validate_postal_code(&local.codigo_postal) {
        return Err("Formato de código postal inválido para o país especificado.".into());
    }
    Ok(())
}
