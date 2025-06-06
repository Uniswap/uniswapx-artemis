# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.81
ARG APP_NAME=uniswapx-artemis

################################################################################
# Create a stage for building the application.
FROM public.ecr.aws/docker/library/rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better Docker layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Create src directory and dummy main to enable dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --locked --release && rm -rf src

# Copy source code
COPY src/ ./src/

# Build the application with proper release optimizations
RUN cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create runtime stage
FROM public.ecr.aws/debian/debian:bookworm-slim AS final

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl3 \
        ca-certificates \
        tini && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get clean

# Create a non-root user for security
RUN groupadd -r artemis && useradd -r -g artemis artemis

# Copy the executable from the build stage
COPY --from=build /bin/server /usr/local/bin/artemis

# Set proper ownership and permissions
RUN chown artemis:artemis /usr/local/bin/artemis && \
    chmod +x /usr/local/bin/artemis

# Switch to non-root user
USER artemis

# Set default environment variables (can be overridden)
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Health check (optional - can be enabled if needed)
# HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
#   CMD /usr/local/bin/artemis --help || exit 1

# Use tini as init system to handle signals properly
ENTRYPOINT ["/usr/bin/tini", "--"]

# Run the application
CMD ["/usr/local/bin/artemis"]
