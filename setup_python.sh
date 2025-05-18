#!/bin/bash
set -e

# Create Python virtual environment if it doesn't exist
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
fi

# Activate the virtual environment
echo "Activating virtual environment..."
source .venv/bin/activate

# Upgrade pip to the latest version
echo "Upgrading pip..."
pip install --upgrade pip

# Install required dependencies
echo "Installing dependencies..."
pip install --index-url https://download.pytorch.org/whl/cpu torch torchvision
pip install ckip-transformers maturin pytest

echo "Setup completed successfully."

