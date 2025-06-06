# Docker Deployment Guide for UniswapX Artemis

This guide covers building and deploying the UniswapX Artemis service using Docker.

## Quick Start

### 1. Prerequisites
- Docker installed and running
- Docker Compose (optional, for easier management)
- `.env` file configured (see [environment-variables.md](environment-variables.md))

### 2. Build the Image
```bash
# Using the build script (recommended)
./scripts/build-docker.sh

# Or using Docker directly
docker build -t uniswapx-artemis:latest .
```

### 3. Run the Container
```bash
# Using the run script (recommended)
./scripts/run-docker.sh

# Or using Docker directly
docker run -d --name uniswapx-artemis --env-file .env uniswapx-artemis:latest

# Or using Docker Compose
docker-compose up -d
```

## Detailed Instructions

### Building the Image

The Dockerfile is optimized for:
- **Multi-stage builds** - Separate build and runtime stages
- **Layer caching** - Dependencies built separately for faster rebuilds
- **Security** - Non-root user, minimal runtime image
- **Size optimization** - Only necessary runtime dependencies

#### Build Options

```bash
# Basic build
docker build -t uniswapx-artemis:latest .

# Build with custom registry
REGISTRY=your-registry.com ./scripts/build-docker.sh

# Build for specific platform
BUILD_PLATFORM=linux/arm64 ./scripts/build-docker.sh

# Build and push to registry
REGISTRY=your-registry.com PUSH=true ./scripts/build-docker.sh
```

### Running the Container

#### Environment Variables
The container requires several environment variables. Create a `.env` file:

```bash
# Required variables
HTTP_ENDPOINT=https://aeneid.storyrpc.io
CHAIN_ID=1315
ORDER_TYPE=Limit
EXECUTOR_ADDRESS=0x25327881066a09F12B066ceA76ADaa1ca61E6c82
PRIVATE_KEY=your-private-key-here

# Optional variables
BID_PERCENTAGE=1
REACTOR_ADDRESS=0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87
# ... see environment-variables.md for complete list
```

#### Container Management

```bash
# Start container
docker run -d \
  --name uniswapx-artemis \
  --env-file .env \
  --restart unless-stopped \
  uniswapx-artemis:latest

# View logs
docker logs -f uniswapx-artemis

# Stop container
docker stop uniswapx-artemis

# Restart container
docker restart uniswapx-artemis

# Remove container
docker rm uniswapx-artemis

# Shell access (for debugging)
docker exec -it uniswapx-artemis /bin/bash
```

### Using Docker Compose

Docker Compose provides easier management and configuration:

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Restart services
docker-compose restart

# Update and restart
docker-compose pull && docker-compose up -d
```

#### Docker Compose Configuration

The `docker-compose.yml` includes:
- Environment variable management
- Health checks
- Resource limits
- Log rotation
- Restart policies

You can override settings with `docker-compose.override.yml`:

```yaml
version: '3.8'

services:
  uniswapx-artemis:
    environment:
      - RUST_LOG=debug  # Enable debug logging
    deploy:
      resources:
        limits:
          memory: 2G      # Increase memory limit
```

## Production Deployment

### Security Considerations

1. **Non-root user**: Container runs as `artemis` user
2. **Read-only filesystem**: Consider using `--read-only` flag
3. **Secrets management**: Use Docker secrets or external secret management
4. **Network isolation**: Use custom Docker networks

```bash
# Example secure deployment
docker run -d \
  --name uniswapx-artemis \
  --user artemis \
  --read-only \
  --tmpfs /tmp \
  --env-file .env \
  --restart unless-stopped \
  --memory=1g \
  --cpus=0.5 \
  uniswapx-artemis:latest
```

### Resource Management

Set appropriate resource limits:

```yaml
# In docker-compose.yml
deploy:
  resources:
    limits:
      memory: 1G
      cpus: '0.5'
    reservations:
      memory: 512M
      cpus: '0.25'
```

### Monitoring and Logging

#### Health Checks
The Docker Compose setup includes health checks:

```bash
# Check container health
docker inspect --format='{{.State.Health.Status}}' uniswapx-artemis
```

#### Log Management
Logs are automatically rotated:
- Max size: 10MB per file
- Max files: 3
- Format: JSON

```bash
# View logs with timestamps
docker logs -t uniswapx-artemis

# Follow logs in real-time
docker logs -f --tail 100 uniswapx-artemis
```

### Advanced Configuration

#### Multi-stage Environment Setup

For different environments (dev, staging, prod):

```bash
# Development
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d

# Production
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

#### Registry and CI/CD

```bash
# Tag for registry
docker tag uniswapx-artemis:latest your-registry.com/uniswapx-artemis:v1.0.0

# Push to registry
docker push your-registry.com/uniswapx-artemis:v1.0.0

# Deploy from registry
docker run -d \
  --env-file .env \
  your-registry.com/uniswapx-artemis:v1.0.0
```

## Troubleshooting

### Common Issues

1. **Container won't start**
   ```bash
   # Check logs
   docker logs uniswapx-artemis
   
   # Check environment variables
   docker exec uniswapx-artemis env
   ```

2. **Permission issues**
   ```bash
   # Check if running as non-root
   docker exec uniswapx-artemis whoami
   ```

3. **Network connectivity**
   ```bash
   # Test network connectivity
   docker exec uniswapx-artemis curl -I https://aeneid.storyrpc.io
   ```

4. **Resource constraints**
   ```bash
   # Check resource usage
   docker stats uniswapx-artemis
   ```

### Debug Mode

Run container with debug logging:

```bash
docker run -d \
  --name uniswapx-artemis-debug \
  --env-file .env \
  -e RUST_LOG=debug \
  -e RUST_BACKTRACE=full \
  uniswapx-artemis:latest
```

### Manual Testing

Test the built binary manually:

```bash
# Run container interactively
docker run -it --rm --env-file .env uniswapx-artemis:latest /bin/bash

# Inside container, test the binary
/usr/local/bin/artemis --help
```

## Build Optimization

### Cache Optimization
The Dockerfile uses multi-stage builds and dependency caching:

1. Dependencies are built first (cached layer)
2. Source code is copied and built separately
3. Final runtime image contains only necessary components

### Image Size
The optimized image includes:
- Debian slim base image
- Only runtime dependencies
- Compressed binary
- No build tools or source code

Current image size: ~100-200MB (vs ~2GB+ with build tools)

## Scripts Reference

- `scripts/build-docker.sh` - Build Docker image with tagging and optional push
- `scripts/run-docker.sh` - Run container with proper configuration
- `docker-compose.yml` - Full-featured container orchestration
- `.dockerignore` - Optimized build context exclusions