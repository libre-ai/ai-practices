# rumble-ai-practices-web

The web layer of rumble-ai-practices. The crate is primarily a **library**
(`App` + components, exercised by the SSR tests), plus a **cfg-gated WASM
client binary** that mounts the PWA in the browser and loads the `portal-*`
design tokens.

## Run the PWA (client-side, WASM)

Needs the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started)
(`cargo install dioxus-cli`):

```bash
# from this directory
dx serve --platform web --features web     # dev server (default http://localhost:8080)
dx build --platform web --features web     # one-off bundle → target/dx/.../web/public
```

The web entrypoint (`src/main.rs`) is cfg-gated on the `web` feature, so default
builds — including host CI — compile an empty `main` with zero web dependencies.

> **PATH gotcha:** some setups ship a `dx` from Deno (`deno x`) that shadows the
> Dioxus CLI and fails with `Unable to choose binary for build`. Make sure the
> Dioxus `dx` comes first in `PATH`, or call it by its Cargo bin path.

## Design tokens

`assets/tokens.css` + `assets/styles.css` come from the `portal-*` design
system (`portal-forge` generates the tokens; the shared identity lives there).
`Root` in `src/main.rs` injects them via `document::Stylesheet`, keeping the
`App` component itself style-agnostic so the SSR tests stay pure.
