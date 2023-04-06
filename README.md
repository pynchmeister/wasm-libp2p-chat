# wasm-libp2p-chat
A wasm chat app connected to the libp2p network. This implementation is a basic version that logs received messages to the browser console.

## prequisites

1. Install Node.js and npm (Node package manager) if you haven't already.
2. Install the required tools for compiling Rust to WebAssembly:

```curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh```

### instructions

1. ```cargo new --lib wasm_libp2p_chat
      cd wasm_libp2p_chat```
      
2. Build the Rust library and generate the WebAssembly module:

```bash wasm-pack build --target web```

3. Serve chat app using a simple web server:

```npx http-server```

Now, open your browser and navigate to http://localhost:8080. You should see your chat app running and connecting to the libp2p network. 
