## Structure

 - May create a `notebooks/` folder in the future, top level pyproject is set up to run local Jupyter.
 - All Python/Rust file-based solutions will be in specific folders `day##`. Each folder is a hybrid Rust/Python repo, effectively with `cargo init` and `poetry init`. `main.py` and `main.rs` will both be in the `src/` folder.
 
## Jupyter Rust Kernel

In order to run the Notebooks with Rust Kernels locally, you'll need to set up the Rust toolchain and [evcxr](https://github.com/google/evcxr).

 - Install Rust with rustup: https://www.rust-lang.org/tools/install
 - Install `evcxr_jupyter`: https://github.com/google/evcxr/tree/main/evcxr_jupyter
   - `cargo install evcxr_jupyter`
   - `evcxr_jupyter --install`

Once that's one, you should have a Rust kernel available. You can check with `jupyter kernelspec list`

```
❯ poetry run jupyter kernelspec list
Available kernels:
  python3    /home/kafonek/advent2022/.venv/share/jupyter/kernels/python3
  rust       /home/kafonek/.local/share/jupyter/kernels/rust
```  


## Timing

1. Cd into directory (`cd day01`)
2. Compile packages
  - `poetry run python -m compileall src/`
  - `cargo build --release`
3. Time and run
  - `time poetry run python src/main.py`
  - `time cargo run --release`