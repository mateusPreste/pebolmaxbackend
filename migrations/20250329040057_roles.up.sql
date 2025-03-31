-- Tabela de papéis do sistema
CREATE TABLE papeis (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(50) UNIQUE NOT NULL, -- 'jogador', 'arbitro', 'admin_estabelecimento', etc.
    descricao TEXT NOT NULL,
    permissoes JSONB -- Armazena permissões específicas em formato JSON
);

-- Tabela de associação entre usuários e papéis
CREATE TABLE usuario_papeis (
    usuario_id INT NOT NULL,
    papel_id INT NOT NULL,
    data_atribuicao TIMESTAMP NOT NULL DEFAULT NOW(),
    situacao VARCHAR(20) NOT NULL DEFAULT 'Pendente' CHECK (situacao IN ('Pendente', 'Aprovado', 'Rejeitado')),
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (usuario_id, papel_id),
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE,
    FOREIGN KEY (papel_id) REFERENCES papeis(id) ON DELETE CASCADE
);