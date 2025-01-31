#!/bin/bash

IMAGE_NAME="rustacean"
CONTAINER_NAME="rust-by-mr9-container"

echo "Building the Docker image..."
docker build -t $IMAGE_NAME .

if [ $? -ne 0 ]; then
    echo "Error: Failed to build the Docker image."
    exit 1
fi

echo "Docker image built successfully."

echo "Removing any existing container with the name '$CONTAINER_NAME'..."
docker rm -f $CONTAINER_NAME || true

echo "Starting the Docker container..."
docker run --name $CONTAINER_NAME -it $IMAGE_NAME /bin/bash

if [ $? -ne 0 ]; then
    echo "Error: Failed to start the Docker container."
    exit 1
fi

echo "You are now inside the Docker container. Type 'exit' to leave the container."