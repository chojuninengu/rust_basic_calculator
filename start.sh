#!/bin/bash


IMAGE_NAME="rust-calculator"
CONTAINER_NAME="rust-calculator-container"

echo "Building the Docker image..."
docker build -t $IMAGE_NAME .

if [ $? -ne 0 ]; then
    echo "Error: Failed to build the Docker image."
    exit 1
fi

echo "Docker image built successfully."


IMAGE_NAME="rust-calculator"
CONTAINER_NAME="rust-calculator-container"

echo "Building the Docker image..."
docker build -t $IMAGE_NAME .

if [ $? -ne 0 ]; then
    echo "Error: Failed to build the Docker image."
    exit 1
fi

echo "Docker image built successfully."


echo "Starting the Docker container..."
docker run --name $CONTAINER_NAME -it $IMAGE_NAME /bin/bash

if [ $? -ne 0 ]; then
    echo "Error: Failed to start the Docker container."ls
    exit 1
fi

echo "You are now inside the Docker container. Type 'exit' to leave the container."
echo "Starting the Docker container..."
docker run --name $CONTAINER_NAME -it $IMAGE_NAME /bin/bash

if [ $? -ne 0 ]; then
    echo "Error: Failed to start the Docker container."
    exit 1
fi

echo "You are now inside the Docker container. Type 'exit' to leave the container."