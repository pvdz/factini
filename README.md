# Factini

Mini factory game.

Written in Rust. Compiled to WASM to run in the browser using html5 canvas.

# Usage

Open index.html in a browser. Due to CORS limitations you'll need to serve it (either locally like with a simple static server, or from some website host online).

No additional installation steps required. Everything happens client side in the browser. All files are static.

# Building from source

Numbers for my machine :shrug:

Changes to the `.md.js` config files do not require compilation. They are always parsed on startup. 

For development;

- cold cache: ~22s
- warm cache: 4-8s (change in `src`)
```
wasm-pack build --target web --dev
```

For production;
- about 200k smaller
- cold cache: ~45s (33s compile, 12s optimization)
- warm cache: ~30s
```
wasm-pack build --target web
```
