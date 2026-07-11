//! Application entrypoint: mounts the Dioxus app and loads the portal-* design
//! tokens + component styles. Business logic stays in the Rust core crates;
//! this only composes the UI shell and its styling.
//!
//! One shared `Root` targets web / desktop / mobile via `dioxus::launch`. The
//! module is cfg-gated on any render feature so that host builds (default
//! features, used by CI) compile an empty `main` with no renderer deps.

#[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
mod web_app {
    use dioxus::prelude::*;
    use rumble_ai_practices_web::{App, register_service_worker};

    const FONTS_CSS: Asset = asset!(
        "/assets/fonts/fonts.css",
        AssetOptions::css()
            .with_static_head(true)
            .with_hash_suffix(false)
    );
    #[used]
    static INTER_FONT: Asset = asset!(
        "/assets/fonts/inter-latin-wght-normal.woff2",
        AssetOptions::builder().with_hash_suffix(false)
    );
    #[used]
    static DISPLAY_FONT: Asset = asset!(
        "/assets/fonts/plus-jakarta-sans-latin-wght-normal.woff2",
        AssetOptions::builder().with_hash_suffix(false)
    );
    const TOKENS_CSS: Asset = asset!(
        "/assets/tokens.css",
        AssetOptions::css().with_static_head(true)
    );
    const THEMES_CSS: Asset = asset!(
        "/assets/themes.css",
        AssetOptions::css().with_static_head(true)
    );
    const BRIDGE_CSS: Asset = asset!(
        "/assets/libre-ia-bridge.css",
        AssetOptions::css().with_static_head(true)
    );
    const COMPONENTS_CSS: Asset = asset!(
        "/assets/components.css",
        AssetOptions::css().with_static_head(true)
    );
    const STYLES_CSS: Asset = asset!(
        "/assets/styles.css",
        AssetOptions::css().with_static_head(true)
    );
    const MANIFEST: Asset = asset!(
        "/assets/manifest.json",
        AssetOptions::builder().with_hash_suffix(false)
    );
    const ICON: Asset = asset!(
        "/assets/icon.svg",
        AssetOptions::builder().with_hash_suffix(false)
    );
    #[used]
    static MEDIA: Asset = asset!(
        "/assets/media/",
        AssetOptions::folder().with_hash_suffix(false)
    );

    /// Wraps the SSR-tested `App` with the design-system stylesheets and the
    /// mobile-first PWA head (installable, standalone, safe-area aware). The
    /// pure `App` component (and its tests) stay style-agnostic.
    #[component]
    fn Root() -> Element {
        // Register the service worker for offline install through the typed
        // browser adapter (best-effort; host/desktop builds use a no-op).
        use_effect(register_service_worker);

        // Keep all Manganis assets linked into the web bundle. CSS marked
        // `static_head` is injected by the Dioxus CLI at build time; PWA metadata
        // lives in index.html, so runtime document evaluation is unnecessary.
        let _assets = (
            FONTS_CSS,
            TOKENS_CSS,
            THEMES_CSS,
            BRIDGE_CSS,
            COMPONENTS_CSS,
            STYLES_CSS,
            MANIFEST,
            ICON,
            MEDIA,
        );

        rsx! { App {} }
    }

    pub fn run() {
        dioxus::launch(Root);
    }
}

fn main() {
    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    web_app::run();
}
