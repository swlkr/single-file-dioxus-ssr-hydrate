# Single file dioxus axum hydrate example

_A starting point for a dioxus ssr app with hydration_

## Quickstart

If you don't care about wasm size, you can ignore the rest of the stuff below

```sh
git clone https://github.com/swlkr/single-file-dioxus-ssr-hydrate
cd single-file-dioxus-ssr-hydrate
dioxus build --features frontend
cargo run --features backend --no-default-features
```

Then visit the page in your browser:

```
open http://localhost:9001
```

To run dioxus in dev mode with live reload without the server part:

(I couldn't get hot reload to not refresh the page)

```sh
dioxus serve --features frontend --hot-reload
```

## Optimizing WASM Size

You do care about wasm size after all, eh? Welcome to the bleeding edge.

This uses a fork of the dioxus-cli that adds plugin support to build a smaller version of the wasm file.

We'll grab that first:

```sh
git clone https://github.com/mrxiaozhuox/dioxus-cli
cd dioxus-cli
cargo install --path .
```

Next up, we'll need to install the wasm-opt-rs crate and plugin:

```sh
cargo install wasm-opt --locked
# make sure you're in the project root (single-file-dioxus-ssr-hydrate)
dioxus plugin add --git https://github.com/brson/wasm-opt-rs.git
```

Finally:

```sh
dioxus build --release --features frontend
```

Should give you a wasm size around ~200KB