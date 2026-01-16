# ---- Builder Stage ----
FROM rust:1.91.1-bookworm AS builder

WORKDIR /usr/src/app

# Install git and ssh client for private git dependencies
RUN apt-get update && apt-get install -y \
    git \
    openssh-client \
    && rm -rf /var/lib/apt/lists/*

# Setup SSH for GitHub
RUN mkdir -p /root/.ssh && \
    ssh-keyscan github.com >> /root/.ssh/known_hosts

# Copy SSH key (Requires id_rsa context)
# COPY id_rsa /root/.ssh/id_rsa
# RUN chmod 600 /root/.ssh/id_rsa

# Configure git to use SSH
RUN git config --global url."ssh://git@github.com/".insteadOf "https://github.com/"

# Configure Cargo to use git CLI for fetching
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Copy only dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY cf-api/Cargo.toml ./cf-api/
COPY cf-entity/Cargo.toml ./cf-entity/
COPY cf-migration/Cargo.toml ./cf-migration/
COPY cf-repository/Cargo.toml ./cf-repository/
COPY cf-util/Cargo.toml ./cf-util/

# Create dummy source files to cache dependencies
RUN mkdir -p cf-api/src cf-entity/src cf-migration/src cf-repository/src cf-util/src && \
    echo "fn main() {}" > cf-api/src/main.rs && \
    echo "fn main() {}" > cf-entity/src/main.rs && \
    echo "" > cf-entity/src/lib.rs && \
    echo "fn main() {}" > cf-migration/src/main.rs && \
    echo "" > cf-migration/src/lib.rs && \
    echo "fn main() {}" > cf-repository/src/main.rs && \
    echo "" > cf-repository/src/lib.rs && \
    echo "" > cf-util/src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release || true

# Copy the actual source code
COPY . .

# Touch the source files to ensure they're rebuilt
RUN touch cf-api/src/main.rs && \
    touch cf-entity/src/main.rs cf-entity/src/lib.rs && \
    touch cf-migration/src/main.rs cf-migration/src/lib.rs && \
    touch cf-repository/src/main.rs cf-repository/src/lib.rs && \
    touch cf-util/src/lib.rs

# Build the actual application
RUN cargo build --release

# ---- Final Stage ----
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 -s /bin/bash appuser

WORKDIR /app

# Create bin directory for binaries
RUN mkdir -p /app/bin

# Copy binaries
COPY --from=builder /usr/src/app/target/release/cf-api /app/bin/
COPY --from=builder /usr/src/app/target/release/cf-entity /app/bin/
COPY --from=builder /usr/src/app/target/release/cf-migration /app/bin/

# Copy project source (needed for entity generation paths)
COPY --from=builder /usr/src/app/cf-entity ./cf-entity

# Copy entrypoint script
COPY entrypoint.sh /app/entrypoint.sh

# Change ownership to non-root user
RUN chown -R appuser:appuser /app && \
    chmod +x /app/bin/cf-api && \
    chmod +x /app/bin/cf-entity && \
    chmod +x /app/bin/cf-migration && \
    chmod +x /app/entrypoint.sh

# Switch to non-root user
USER appuser

EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["/app/entrypoint.sh"]