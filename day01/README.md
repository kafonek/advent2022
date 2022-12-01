
```
~/advent2022/day01 is 📦 v0.1.0 via 🐍 v3.9.11 (.venv) via 🦀 v1.67.0-nightly 
❯ poetry run python -m compileall src/
Listing 'src/'...
Compiling 'src/main.py'...

~/advent2022/day01 is 📦 v0.1.0 via 🐍 v3.9.11 (.venv) via 🦀 v1.67.0-nightly 
❯ time poetry run python src/main.py
70698
206643

real    0m0.498s
user    0m0.450s
sys     0m0.049s

~/advent2022/day01 is 📦 v0.1.0 via 🐍 v3.9.11 (.venv) via 🦀 v1.67.0-nightly 
❯ cargo build --release
    Finished release [optimized] target(s) in 0.00s

~/advent2022/day01 is 📦 v0.1.0 via 🐍 v3.9.11 (.venv) via 🦀 v1.67.0-nightly 
❯ time cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/problem01`
70698
206643

real    0m0.052s
user    0m0.053s
sys     0m0.000s
```