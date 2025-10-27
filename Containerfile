# Build stage
FROM docker.io/library/rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev sqlite-dev

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY templates ./templates

# Build release binary
RUN cargo build --release

# Runtime stage
FROM docker.io/library/alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libgcc sqlite-libs

# Create app user and data directory
RUN addgroup -g 1000 secrets && \
    adduser -D -u 1000 -G secrets secrets && \
    mkdir -p /data && \
    chown -R secrets:secrets /data

WORKDIR /app

# Copy binary and templates from builder
COPY --from=builder /build/target/release/secrets /usr/local/bin/secrets
COPY --from=builder /build/templates ./templates

# Change ownership
RUN chown -R secrets:secrets /app

# Switch to non-root user
USER secrets

# Expose port
EXPOSE 5150

# Set environment variables
ENV DATABASE_URL=sqlite:///data/secrets.db?mode=rwc
ENV HOST=0.0.0.0
ENV PORT=5150
ENV RUST_LOG=secrets=info,tower_http=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:5150/ || exit 1

# Volume for database
VOLUME ["/data"]

# Run the application
CMD ["/usr/local/bin/secrets"]
