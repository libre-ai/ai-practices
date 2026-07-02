//! Client entrypoint: mounts the Dioxus PWA in the browser (WASM) and loads the
//! portal-* design tokens + component styles. Business logic stays in the Rust
//! core crates; this only composes the UI shell and its styling.
//!
//! The web entrypoint is cfg-gated on the `web` feature so that host builds
//! (default features, used by CI) compile an empty `main` with no web deps.

#[cfg(feature = "web")]
mod web_app {
    use dioxus::prelude::*;
    use rumble_ai_practices_web::App;

    const TOKENS_CSS: Asset = asset!("/assets/tokens.css");
    const STYLES_CSS: Asset = asset!("/assets/styles.css");

    /// Wraps the SSR-tested `App` with the design-system stylesheets, so the
    /// pure `App` component (and its tests) stay style-agnostic.
    #[component]
    fn Root() -> Element {
        rsx! {
            document::Stylesheet { href: TOKENS_CSS }
            document::Stylesheet { href: STYLES_CSS }
            App {}
        }
    }

    pub fn run() {
        dioxus::launch(Root);
    }
}

fn main() {
    #[cfg(feature = "web")]
    web_app::run();
}
