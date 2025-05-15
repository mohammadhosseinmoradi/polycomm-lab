const wasm = await WebAssembly.instantiateStreaming(
    fetch('target/wasm32-unknown-unknown/release/wasm_add.wasm'),
    {
        env: {
            log: (val) => console.log("Called from Rust: ", val),
        }
    }
);

const {add} = wasm.instance.exports;
add(3, 4);
