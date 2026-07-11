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
    use rumble_ai_practices_web::App;

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
        // Register the service worker for offline install (best-effort; the
        // file is served next to the app so its scope covers the shell).
        use_effect(|| {
            document::eval(
                "if ('serviceWorker' in navigator) { \
                   navigator.serviceWorker.register('./sw.js').catch(function () {}); \
                 }",
            );
        });

        rsx! {
            document::Title { "rumble-ai-practices — réflexes IA" }
            document::Meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1, viewport-fit=cover",
            }
            // Platform manifests require a literal; this is the canonical Vert Libre token.
            document::Meta { name: "theme-color", content: "#22C55E" }
            document::Meta { name: "mobile-web-app-capable", content: "yes" }
            document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
            document::Meta {
                name: "apple-mobile-web-app-status-bar-style",
                content: "black-translucent",
            }
            document::Meta { name: "apple-mobile-web-app-title", content: "AI Practices" }
            document::Link { rel: "manifest", href: MANIFEST }
            document::Link { rel: "icon", r#type: "image/svg+xml", href: ICON }
            document::Link { rel: "apple-touch-icon", href: ICON }
            document::Stylesheet { href: FONTS_CSS }
            document::Stylesheet { href: TOKENS_CSS }
            document::Stylesheet { href: THEMES_CSS }
            document::Stylesheet { href: BRIDGE_CSS }
            document::Stylesheet { href: COMPONENTS_CSS }
            document::Stylesheet { href: STYLES_CSS }
            App {}
        }
    }

    pub fn run() {
        dioxus::launch(Root);
    }
}

fn main() {
    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    web_app::run();
}
