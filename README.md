# Single file dioxus axum hydrate example

_A starting point for a dioxus ssr app with hydration_

Welcome to the bleeding edge.

This uses a fork of the dioxus-cli that adds plugin support to build a smaller version of the wasm file.

We'll grab that first:

```sh
git clone https://github.com/DioxusLabs/cli
cd cli
cargo install --path .
```

Next up, we'll need to install the wasm-opt-rs crate and plugin:

```sh
cargo install wasm-opt --locked
dioxus plugin add --git https://github.com/brson/wasm-opt-rs.git
```

Now we're ready roll:

```sh
git clone https://github.com/swlkr/single-file-dioxus-ssr-hydrate
cd single-file-dioxus-ssr-hydrate
dioxus build --release --features frontend
cargo run --features backend --no-default-features
```

Then visit the page in your browser:

```
open http://localhost:9001
```

To build dioxus in dev mode, use:

```sh
dioxus build --features frontend
```

To run dioxus in dev mode with live reload:

(I couldn't get hot reload to not refresh the page)

```sh
dioxus serve --features frontend --hot-reload
```