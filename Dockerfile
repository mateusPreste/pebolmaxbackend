# Update Dockerfile
FROM rust:1.81-slim

# Create a non-root user
RUN useradd -m -u 1000 rust_user

# Create directory and set permissions
WORKDIR /usr/src/app
RUN mkdir -p target && \
  chown -R rust_user:rust_user /usr/src/app && \
  chown -R rust_user:rust_user $CARGO_HOME

# Install dependencies
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  git \
  curl \
  postgresql-client \
  pkg-config \
  build-essential \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

# Set environment
ENV PATH="/usr/local/cargo/bin:${PATH}"
ENV DATABASE_URL="postgres://admin:password123@postgres:5432/rust_sqlx"

# Copy files and set permissions
COPY --chown=rust_user:rust_user . .

# Switch to non-root user
USER rust_user

# Ensure script is executable
RUN chmod +x /usr/src/app/start.sh

CMD ["/usr/src/app/start.sh"]