-- Novo script: Cria a tabela reservas atualizada para armazenar timestamps completos.
CREATE TABLE reservas (
    id SERIAL PRIMARY KEY,
    usuario_id INT NOT NULL,           -- Criador da reserva
    quadra_id INT NOT NULL,
    inicio TIMESTAMP NOT NULL,         -- Data e horário de início da reserva
    fim TIMESTAMP NOT NULL,            -- Data e horário de fim da reserva
    status_id VARCHAR(50) 
        CHECK (status_id IN ('Aberto', 'Reservado', 'Em Disputa', 'Cancelado', 'Perdido')) 
        NOT NULL DEFAULT 'Aberto',
    modalidade VARCHAR(20) 
        CHECK (modalidade IN ('usuarios', 'times')),
    min_pagantes INT DEFAULT 0,        -- Mínimo necessário de pagantes para reservas de usuários
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE,
    FOREIGN KEY (quadra_id) REFERENCES quadras(id) ON DELETE CASCADE
);

/* 
-- Tabela de times em reservas do tipo "times"
CREATE TABLE times_reserva (
    time_id INT NOT NULL,
    reserva_id INT NOT NULL,
    nome VARCHAR(100) NOT NULL,
    min_jogadores INT NOT NULL DEFAULT 0, 
    PRIMARY KEY (reserva_id, time_id),
    FOREIGN KEY (reserva_id) REFERENCES reservas(id) ON DELETE CASCADE,
    FOREIGN KEY (time_id) REFERENCES times(id) ON DELETE CASCADE
);


-- Tabela de usuários participantes da reserva (modalidade "usuarios") com coluna "status"
CREATE TABLE reserva_usuarios (
    reserva_id INT NOT NULL,
    usuario_id INT NOT NULL,
    time_id INT NULL, -- Time do jogador na reserva (opcional)
    status VARCHAR(20) NOT NULL DEFAULT 'pendente'
        CHECK (status IN ('pendente', 'rejeitado', 'aceito', 'cancelado', 'expirado')),
    PRIMARY KEY (reserva_id, usuario_id),
    FOREIGN KEY (reserva_id) REFERENCES reservas(id) ON DELETE CASCADE,
    FOREIGN KEY (time_id) REFERENCES times_reserva(id) ON DELETE CASCADE,
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
); 


-- Tabela que registra os times cujos integrantes são todos convidados a participar de uma reserva
CREATE TABLE times_convidados_reserva (
    reserva_id INT NOT NULL,
    time_id INT NOT NULL,
    data_convite TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'pendente'
        CHECK (status IN ('pendente', 'aceito', 'rejeitado', 'cancelado', 'expirado')),
    PRIMARY KEY (reserva_id, time_id),
    FOREIGN KEY (reserva_id) REFERENCES reservas(id) ON DELETE CASCADE,
    FOREIGN KEY (time_id) REFERENCES times(id) ON DELETE CASCADE
); */