
## Timing

1. Cd into directory (`cd day01`)
2. Compile packages
  - `poetry run python -m compileall src/`
  - `cargo build --release`
3. Time and run
  - `time poetry run python src/main.py`
  - `time cargo run --release`