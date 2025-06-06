#!/bin/bash

# Script to run UniswapX Artemis Docker container
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="uniswapx-artemis"
CONTAINER_NAME="uniswapx-artemis"
ENV_FILE=".env"

echo -e "${GREEN}Running UniswapX Artemis Docker Container${NC}"

# Check if .env file exists
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${RED}❌ Environment file .env not found!${NC}"
    echo "Please create a .env file with required environment variables."
    echo "See environment-variables.md for details."
    exit 1
fi

# Check if container is already running
if [ "$(docker ps -q -f name=${CONTAINER_NAME})" ]; then
    echo -e "${YELLOW}Container ${CONTAINER_NAME} is already running${NC}"
    echo "Use 'docker logs -f ${CONTAINER_NAME}' to view logs"
    echo "Use 'docker stop ${CONTAINER_NAME}' to stop it"
    exit 0
fi

# Remove existing stopped container if it exists
if [ "$(docker ps -aq -f name=${CONTAINER_NAME})" ]; then
    echo -e "${YELLOW}Removing existing stopped container...${NC}"
    docker rm ${CONTAINER_NAME}
fi

# Run the container
echo -e "${YELLOW}Starting container...${NC}"
docker run -d \
    --name ${CONTAINER_NAME} \
    --env-file ${ENV_FILE} \
    --restart unless-stopped \
    --log-driver json-file \
    --log-opt max-size=10m \
    --log-opt max-file=3 \
    ${IMAGE_NAME}:latest

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Container started successfully!${NC}"
    echo ""
    echo "Container name: ${CONTAINER_NAME}"
    echo "Image: ${IMAGE_NAME}:latest"
    echo ""
    echo "Useful commands:"
    echo "  View logs:        docker logs -f ${CONTAINER_NAME}"
    echo "  Stop container:   docker stop ${CONTAINER_NAME}"
    echo "  Restart:          docker restart ${CONTAINER_NAME}"
    echo "  Container stats:  docker stats ${CONTAINER_NAME}"
    echo "  Shell access:     docker exec -it ${CONTAINER_NAME} /bin/bash"
    
    # Show initial logs
    echo ""
    echo -e "${YELLOW}Initial logs (last 20 lines):${NC}"
    sleep 2  # Give container time to start
    docker logs --tail 20 ${CONTAINER_NAME}
    
else
    echo -e "${RED}❌ Failed to start container!${NC}"
    exit 1
fi 