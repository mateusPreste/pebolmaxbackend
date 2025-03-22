#!/bin/sh
echo "Waiting for pgAdmin to be ready..."
until curl -s http://pgAdmin:80/ > /dev/null; do
  echo "pgAdmin is unavailable - sleeping"
  sleep 5
done
echo "pgAdmin is up - executing command"

# Your script commands here
echo "Running post-pgAdmin initialization script..."


# This script performs the following actions:
# 1. Installs the sqlx-cli tool using Cargo.
# 2. Runs the database migrations using sqlx.
# 3. Compiles and runs the Rust application.
cargo install sqlx-cli &&
sqlx migrate run &&
cargo run

# Keep container running
tail -f /dev/null