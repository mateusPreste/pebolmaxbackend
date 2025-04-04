
CREATE TABLE services (
    service_id SERIAL PRIMARY KEY,
    service_type VARCHAR(50) NOT NULL, -- ex: 'reserva', 'aluguel', 'emprestimo', 'pagamento_direto'
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Now insert some initial values into services.
INSERT INTO services (service_type, service_name, description)
VALUES 
  ('reserva', 'Reserva de Quadra', 'Serviço de reserva de quadras esportivas');


CREATE TABLE transactions (
    transaction_id SERIAL PRIMARY KEY,
    service_id INTEGER NOT NULL,
    amount NUMERIC(12, 2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES services(service_id)
);

CREATE TABLE payment_records (
    payment_id SERIAL PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    payment_details JSONB,  -- Armazena informações específicas do pagamento (pode ser estruturado ou em JSON)
    finalized_at TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);

CREATE TABLE transaction_status_history (
    history_id SERIAL PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);
