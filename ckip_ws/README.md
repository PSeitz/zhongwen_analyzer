# 1. Prepare venv
python -m venv .venv

# 2. Activate venv
source .venv/bin/activate.fish

#   Bash
#   source .venv/bin/activate

# 3. Install dependencies
pip install --index-url https://download.pytorch.org/whl/cpu torch torchvision
pip install ckip-transformers maturin pytest
