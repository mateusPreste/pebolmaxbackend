-- tipos de servicos
CREATE TABLE services_types (
    service_type_id SERIAL PRIMARY KEY,
    service_type VARCHAR(50) NOT NULL, -- ex: 'reserva', 'aluguel', 'emprestimo', 'pagamento_direto'
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Now insert some initial values into services_types.
INSERT INTO services_types (service_type, service_name, description)
VALUES 
  ('reserva', 'Reserva de Quadra', 'Serviço de reserva de quadras esportivas');


CREATE TABLE services (
    service_id SERIAL PRIMARY KEY,
    service_type_id INTEGER NOT NULL,
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (service_type_id) REFERENCES services_types(service_type_id)
);

-- identificador do pagamento
-- reserva_id
CREATE TABLE transactions (
    transaction_id SERIAL PRIMARY KEY,
    service_id INTEGER NOT NULL,
    conta_id INTEGER NOT NULL, -- Added conta_id, allowing NULL for flexibility
    amount NUMERIC(12, 2) NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pendente', 'rejeitado', 'aceito', 'cancelado', 'expirado')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES services(service_id),
    FOREIGN KEY (conta_id) REFERENCES contas(id) -- Added foreign key constraint
);

/* CREATE TABLE payment_records (
    payment_id SERIAL PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    finalized_at TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
); */

CREATE TABLE transaction_status_history (
    history_id SERIAL PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pendente', 'rejeitado', 'aceito', 'cancelado', 'expirado')),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);
