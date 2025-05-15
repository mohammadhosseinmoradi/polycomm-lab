# simple-wasm-binding

A minimal Rust → WebAssembly → JavaScript binding example  
with no `wasm-bindgen`, no macros, and direct `env` imports.

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```