-- Add down migration script here

DROP TABLE IF EXISTS transaction_status_history;
/* DROP TABLE IF EXISTS payment_records; */
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS services_types;