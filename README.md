A Simple Noscript Boilerplate
====

wasm_bindgen Rust boilerplate for writing and deploying Noscript. This is also a noscript example that filters Nostr text note events with the Japanese language. Other language filtering is available at [branches](https://github.com/digi-monkey/noscript-boilerplate/branches)

---
required:

- `wasm-pack` installed https://github.com/rustwasm/wasm-pack 
  
## Code Structure

- `script` is the crate of Noscript source code, which is what will be compiled to wasm bytecode that saved on relay.
- `deploy` is the crate of Noscript deploying tool, turn the script source code to a special event and then sign it and send it to relay.
- `index.js` is the js test environment which creates a fake runtime interface, and runs the wasm script in browser for test

## Compile

```
cd script && wasm-pack build --target web
```

## Test

```
yarn && yarn serve
```

check browser `http://localhost:8080/` see console

### Deploy Noscript

create a `config.toml` file inside `deploy` folder and paste your Nostr private key

```
cd deploy
cp example-config.toml config.toml

// generate and sign event
cargo run --bin deploy
```

## What is Noscript?

Noscript means `Nostr Script`, which refers to the idea that we can put code on Nostr protocol(save the code in a special kind of Event, sign it and send it to the relay, basically), a Noscript is a piece of code to be running in a standard environment provided by Nostr client so there is no central control but user-generated alternatives, it can be used to provide custom feeds/recommend algorithms and so much more.

right now we are experimenting Noscript with wasm runtime under the Nostr web client [flycat.club](https://github.com/digi-monkey/flycat-web)
