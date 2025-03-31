DROP TABLE IF EXISTS admin_estabelecimento;

DROP TABLE IF EXISTS quadras_horarios_excepcionais;

DROP TABLE IF EXISTS quadras_horarios;
-- Add down migration script here
DROP TABLE IF EXISTS quadras;

-- Drop locais next (references estabelecimentos)
DROP TABLE IF EXISTS locais;

-- Drop estabelecimento_formas_pagamento (references estabelecimentos)
DROP TABLE IF EXISTS estabelecimento_formas_pagamento;

-- Finally, drop estabelecimentos
DROP TABLE IF EXISTS estabelecimentos;

