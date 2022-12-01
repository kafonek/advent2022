## Structure

 - May create a `notebooks/` folder in the future, top level pyproject is set up to run local Jupyter.
 - All Python/Rust file-based solutions will be in specific folders `day##`. Each folder is a hybrid Rust/Python repo, effectively with `cargo init` and `poetry init`. `main.py` and `main.rs` will both be in the `src/` folder.
 
## Timing

1. Cd into directory (`cd day01`)
2. Compile packages
  - `poetry run python -m compileall src/`
  - `cargo build --release`
3. Time and run
  - `time poetry run python src/main.py`
  - `time cargo run --release`