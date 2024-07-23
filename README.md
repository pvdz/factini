# Factini

Mini factory game.

Written in Rust. Compiled to WASM to run in the browser using html5 canvas.

GitHub: [github.com/pvdz/factini](https://github.com/pvdz/factini)
Demo: [pvdz.ee/project/factini](https://pvdz.ee/project/factini/)
Blog post: [pvdz.ee/weblog/454](https://pvdz.ee/weblog/454)

# Building from source

The cargo.toml was intentionally kept as minimal as possible. Project does not depend on `unsafe` code (afaik).

Numbers for my machine :shrug:

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

# Usage

You'll need to compile it first (see above). Then;

Open [html/index.html](html/index.html) in a (modern) browser. Due to CORS limitations you'll need to serve it (either locally like with a simple static server, or from some website host online). This needs access to the [resource] (_build output, containing the wasm_) and [src] folders.

You can open [html/debug.html](html/debug.html) for a wide breadth of options, editor, tech tree status, sprite maps, etc. As the name suggests. It does not depend on a (Rust) debug/prod build (yes, the production build includes all debug code, heh).

Changes to the `.md.js` config files do not require compilation. They are always parsed on (page) startup.

After building from source, no additional installation steps required. Everything happens client side in the browser. All files are static.

# About

This game is inspired by games like [Factorio](https://www.factorio.com/) and [Mindustry](https://mindustrygame.github.io/). The basic premise is to draw belts to move items from one machine to another and create new things.

The project was a learning project to get my hands dirty writing Rust. Excuse my rookie mistakes. I was really just trying to find my way in Rust (: Additionally, the project started as a CLI app and then moved to a wasm/web app. Some legacy code remained, which hopefully explains why it's there.

For more details [read the blog post(s)](https://pvdz.ee/weblog/454) where I've tried to go into detail.

# Notes on Rust

Most of this was a discovery on how to use Rust in a web app.

I've since then read an actual book on Rust and certain concepts and syntax have been clarified, but that's not reflected in the source code here.

Notable example: I had no idea what the question mark did (_I know now it propagates errors_) and just slapped it where-ever the compiler complained. For example, I would have made the config parser less app-crashing if I had known about that mechanism sooner.

Another example are labeled `format!` arguments, which in some cases would have been much better for the legibility.

There's a bunch more examples like that, details on module system, Traits, Box/Rc, etc. And even though I probably would not have used Traits even knowing better about what they are now, I'm not sure I would have significantly used them more than present.

The code uses a "god object" approach, which circumvents a bunch of borrow restrictions, as well as indirection through indexed access for the same reason. I'm sure some will frown upon that but I liked my mental model on that (_sorry future self_).

Either way, I've left the source as before-read-a-book-on-it and for the time being am not planning to update it either :)
