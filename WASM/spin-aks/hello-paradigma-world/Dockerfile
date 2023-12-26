FROM scratch
COPY spin.toml /spin.toml
COPY target/wasm32-wasi/release/hello_paradigma_world.wasm /target/wasm32-wasi/release/hello_paradigma_world.wasm
ENTRYPOINT ["/spin.toml"]