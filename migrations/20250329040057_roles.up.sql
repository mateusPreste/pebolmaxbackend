-- Tabela de papéis do sistema
CREATE TABLE niveis (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(50) UNIQUE NOT NULL, -- 'full', 'pix'.
    descricao TEXT NOT NULL
);

-- Tabela de associação entre usuários e papéis
CREATE TABLE usuario_niveis (
    usuario_id INT NOT NULL,
    niveis_id INT NOT NULL,
    data_atribuicao TIMESTAMP NOT NULL DEFAULT NOW(),
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (usuario_id, niveis_id),
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE,
    FOREIGN KEY (niveis_id) REFERENCES niveis(id) ON DELETE CASCADE
);

INSERT INTO niveis (nome, descricao) VALUES ('full', 'acesso completo ao sistema');
INSERT INTO niveis (nome, descricao) VALUES ('conta pix', 'acesso restrito'); 
INSERT INTO niveis (nome, descricao) VALUES ('visitante', 'apenas acesso aos dados publicos'); 