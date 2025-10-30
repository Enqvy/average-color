#!/bin/bash

if [ -f "../target/release/average-color" ]; then
    echo "running existing build..."
    time ../target/release/average-color benchmark.jpg
else
    echo "build not found, building..."
    if cargo build --release; then
        echo "build finished, running..."
        time ../target/release/average-color benchmark.jpg
    else
        echo "build failed, cant run app"
        exit 1
    fi
fi
