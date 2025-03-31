use crate::modules::arenas::arenas_model::Estabelecimento;
use regex::Regex; // ou o caminho correto para Estabelecimento

// Trait que define a validação do tax_id.
pub trait TaxIdValidator {
    fn validate_tax_id(&self, tax_id: &str) -> bool;
}

// Implementação para o Brasil usando uma regex para CNPJ.
pub struct BrazilTaxIdValidator;

impl TaxIdValidator for BrazilTaxIdValidator {
    fn validate_tax_id(&self, tax_id: &str) -> bool {
        // Regex no formato: "XX.XXX.XXX/XXXX-XX"
        let re = Regex::new(r"^\d{2}\.\d{3}\.\d{3}/\d{4}-\d{2}$").unwrap();
        re.is_match(tax_id)
    }
}

/// Retorna o validador adequado para o país.
/// Se o país não for reconhecido, retorna um erro.
pub fn validator_for(country: &str) -> Result<Box<dyn TaxIdValidator>, String> {
    match country.to_lowercase().as_str() {
        "brasil" => Ok(Box::new(BrazilTaxIdValidator)),
        _ => Err(format!(
            "País não identificado para validação de tax_id: {}",
            country
        )),
    }
}

/// Valida os dados de um estabelecimento.
/// Se o país não for reconhecido ou o tax_id estiver em formato inválido, retorna um erro.
pub fn validate_establishment(est: &Estabelecimento) -> Result<(), String> {
    let validator = validator_for(&est.pais)?;
    if !validator.validate_tax_id(&est.tax_id) {
        return Err("Formato do tax_id inválido para o país especificado.".into());
    }
    Ok(())
}
