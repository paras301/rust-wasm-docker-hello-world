FROM scratch
COPY ./target/wasm32-wasi/release/rust-wasm-docker-hello-world.wasm /rust-wasm-docker-hello-world.wasm
ENTRYPOINT [ "rust-wasm-docker-hello-world.wasm" ]