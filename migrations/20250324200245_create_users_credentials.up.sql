-- Add up migration script here
CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL, -- Nome completo do usuário
    cpf VARCHAR(14) UNIQUE NOT NULL, -- CPF do usuário (formato: XXX.XXX.XXX-XX)
    apelido VARCHAR(50) UNIQUE NOT NULL, -- Apelido dentro da plataforma
    foto TEXT, -- URL da foto do usuário
    reputacao INT DEFAULT 100 CHECK (reputacao BETWEEN 0 AND 100) -- Score de reputação (0 a 100)
);

-- Tabela de credenciais dos usuários
CREATE TABLE credenciais (
    id SERIAL PRIMARY KEY,
    usuario_id INT NOT NULL UNIQUE, -- Relacionado ao usuário
    email VARCHAR(255) NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    phone_number VARCHAR(20),
    phone_verified BOOLEAN NOT NULL DEFAULT FALSE,
    oauth_provider VARCHAR(50),
    oauth_provider_id VARCHAR(255),
    password_hash VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
);