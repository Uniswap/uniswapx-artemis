#!/bin/bash

# Script to build UniswapX Artemis Docker image
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="uniswapx-artemis"
REGISTRY=${REGISTRY:-""}
VERSION=${VERSION:-"latest"}
BUILD_PLATFORM=${BUILD_PLATFORM:-"linux/amd64"}

# Get git commit hash for tagging
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

echo -e "${GREEN}Building UniswapX Artemis Docker Image${NC}"
echo "Image: ${IMAGE_NAME}:${VERSION}"
echo "Platform: ${BUILD_PLATFORM}"
echo "Git Commit: ${GIT_COMMIT}"
echo "Build Date: ${BUILD_DATE}"
echo ""

# Build arguments
BUILD_ARGS=(
    --build-arg "GIT_COMMIT=${GIT_COMMIT}"
    --build-arg "BUILD_DATE=${BUILD_DATE}"
    --platform "${BUILD_PLATFORM}"
)

# Add registry prefix if specified
if [ -n "$REGISTRY" ]; then
    FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}"
else
    FULL_IMAGE_NAME="${IMAGE_NAME}"
fi

# Build the image
echo -e "${YELLOW}Building Docker image...${NC}"
docker build \
    "${BUILD_ARGS[@]}" \
    -t "${FULL_IMAGE_NAME}:${VERSION}" \
    -t "${FULL_IMAGE_NAME}:${GIT_COMMIT}" \
    -t "${FULL_IMAGE_NAME}:latest" \
    .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Docker image built successfully!${NC}"
    echo ""
    echo "Available tags:"
    echo "  ${FULL_IMAGE_NAME}:${VERSION}"
    echo "  ${FULL_IMAGE_NAME}:${GIT_COMMIT}"
    echo "  ${FULL_IMAGE_NAME}:latest"
    echo ""
    
    # Show image size
    echo "Image size:"
    docker images "${FULL_IMAGE_NAME}:latest" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"
    
    # Optional: Run security scan if trivy is available
    if command -v trivy &> /dev/null; then
        echo ""
        echo -e "${YELLOW}Running security scan...${NC}"
        trivy image --exit-code 0 --severity HIGH,CRITICAL "${FULL_IMAGE_NAME}:latest"
    fi
    
else
    echo -e "${RED}❌ Docker build failed!${NC}"
    exit 1
fi

# Optional: Push to registry
if [ "$PUSH" = "true" ] && [ -n "$REGISTRY" ]; then
    echo ""
    echo -e "${YELLOW}Pushing to registry...${NC}"
    docker push "${FULL_IMAGE_NAME}:${VERSION}"
    docker push "${FULL_IMAGE_NAME}:${GIT_COMMIT}"
    docker push "${FULL_IMAGE_NAME}:latest"
    echo -e "${GREEN}✅ Images pushed successfully!${NC}"
fi 