-- Tabela de estabelecimentos que gerenciam os locais
CREATE TABLE estabelecimentos (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    tax_id VARCHAR(50) UNIQUE NOT NULL, -- Identificador fiscal genérico (ex: CNPJ, VAT, etc.)
    tipo VARCHAR(20) NOT NULL CHECK (tipo IN ('publico', 'privado')),
    pais VARCHAR(50) NOT NULL    
);

-- TODO
-- Tabela de formas de pagamento para estabelecimentos com restrição de tipos
CREATE TABLE estabelecimento_formas_pagamento (
    id SERIAL PRIMARY KEY,
    estabelecimento_id INT NOT NULL,                     -- Referência ao estabelecimento
    tipo_pagamento VARCHAR(20) NOT NULL 
        CHECK (tipo_pagamento IN ('pix', 'cartao', 'transferencia')), -- Tipos permitidos
    identificador VARCHAR(255) NOT NULL,                   -- Chave PIX, número do cartão, etc.
    verificado BOOLEAN NOT NULL DEFAULT FALSE,           -- Indica se a forma de pagamento foi verificada
    data_verificacao TIMESTAMP,                           -- Data da verificação (opcional)
    observacoes TEXT,                                     -- Detalhes adicionais, se houver
    FOREIGN KEY (estabelecimento_id) REFERENCES estabelecimentos(id) ON DELETE CASCADE
);

-- Tabela de locais onde estão as quadras
CREATE TABLE locais (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    rua VARCHAR(255) NOT NULL,
    numero VARCHAR(50) NOT NULL,
    complemento VARCHAR(255),
    bairro VARCHAR(100) NOT NULL,
    cidade VARCHAR(100) NOT NULL,
    estado VARCHAR(100) NOT NULL,
    codigo_postal VARCHAR(20) NOT NULL, -- CEP genérico
    country VARCHAR(50) NOT NULL,     -- País do endereço
    latitude DECIMAL(10,7) NOT NULL,
    longitude DECIMAL(10,7) NOT NULL,
    estabelecimento_id INT NOT NULL,
    FOREIGN KEY (estabelecimento_id) REFERENCES estabelecimentos(id) ON DELETE CASCADE
);

-- Tabela de quadras pertencentes a um local
CREATE TABLE quadras (
    id SERIAL PRIMARY KEY,
    local_id INT NOT NULL,
    nome VARCHAR(100) NOT NULL,
    photo_url TEXT, 
    UNIQUE(local_id, nome),
    FOREIGN KEY (local_id) REFERENCES locais(id) ON DELETE CASCADE
);


CREATE TABLE quadras_horarios (
    id SERIAL PRIMARY KEY,
    quadra_id INT NOT NULL,
    dia_semana VARCHAR(10) NOT NULL,
    horario_inicio TIME NOT NULL,
    horario_fim TIME NOT NULL,
    CHECK (horario_fim > horario_inicio),
    CHECK (dia_semana IN ('domingo', 'segunda', 'terca', 'quarta', 'quinta', 'sexta', 'sabado')),
    FOREIGN KEY (quadra_id) REFERENCES quadras(id) ON DELETE CASCADE
);

-- TODO
-- Tabela para registrar horários excepcionais de funcionamento de uma quadra
CREATE TABLE quadras_horarios_excepcionais (
    id SERIAL PRIMARY KEY,
    quadra_id INT NOT NULL,
    data DATE NOT NULL, -- Data específica do dia atípico (ex: feriado)
    horario_inicio TIME NOT NULL,
    horario_fim TIME NOT NULL,
    CHECK (horario_fim > horario_inicio),
    FOREIGN KEY (quadra_id) REFERENCES quadras(id) ON DELETE CASCADE,
    UNIQUE (quadra_id, data)  -- Garante que cada quadra tenha no máximo um horário especial por data
);

-- TODO
CREATE TABLE admin_estabelecimento (
    usuario_id INT NOT NULL,
    estabelecimento_id INT NOT NULL,
    cargo VARCHAR(50) NOT NULL,
    data_inicio TIMESTAMP NOT NULL DEFAULT NOW(),
    data_fim TIMESTAMP NULL,
    PRIMARY KEY (usuario_id, estabelecimento_id),
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE,
    FOREIGN KEY (estabelecimento_id) REFERENCES estabelecimentos(id) ON DELETE CASCADE
);