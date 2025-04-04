-- Add down migration script here
-- First, drop tables that reference others.
DROP TABLE IF EXISTS reserva_usuarios;
DROP TABLE IF EXISTS times_convidados_reserva;
DROP TABLE IF EXISTS times_reserva;

-- Next, drop the primary reservation table.
DROP TABLE IF EXISTS reservas;

-- Finally, drop the status table.
DROP TABLE IF EXISTS status_reserva;