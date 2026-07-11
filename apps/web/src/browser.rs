//! Typed browser integrations used by the WASM shell.
//!
//! Every operation has a no-op host implementation so SSR, desktop scaffolding
//! and unit tests keep compiling without JavaScript evaluation.

#[cfg(target_arch = "wasm32")]
mod imp {
    use std::cell::RefCell;

    use js_sys::Array;
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use web_sys::{
        Blob, BlobPropertyBag, Document, FocusOptions, HtmlAnchorElement, HtmlButtonElement,
        HtmlElement, KeyboardEvent, Url,
    };

    type KeyHandler = Closure<dyn FnMut(KeyboardEvent)>;

    thread_local! {
        static KEY_HANDLER: RefCell<Option<KeyHandler>> = RefCell::new(None);
        static SELECTED_AT: RefCell<Option<f64>> = const { RefCell::new(None) };
        static DELAYS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    }

    fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    fn document() -> Option<Document> {
        window()?.document()
    }

    fn visible_element(selector: &str) -> Option<HtmlElement> {
        let element = document()?.query_selector(selector).ok()??;
        let visibility = window()?
            .get_computed_style(&element)
            .ok()??
            .get_property_value("visibility")
            .ok()?;
        if visibility == "hidden" {
            return None;
        }
        element.dyn_into().ok()
    }

    fn animate_keycap(element: &HtmlElement) {
        let Ok(Some(keycap)) = element.query_selector(".cap") else {
            return;
        };
        let classes = keycap.class_list();
        let _ = classes.add_1("is-down");
        if let Some(window) = window() {
            let callback = Closure::once_into_js(move || {
                let _ = classes.remove_1("is-down");
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.unchecked_ref(),
                160,
            );
        }
    }

    fn click_choice(selector: &str) -> bool {
        let Some(element) = visible_element(selector) else {
            return false;
        };
        animate_keycap(&element);
        element.click();
        true
    }

    fn click_first(selectors: &[&str]) -> bool {
        selectors.iter().any(|selector| {
            let Some(element) = visible_element(selector) else {
                return false;
            };
            if element
                .dyn_ref::<HtmlButtonElement>()
                .is_some_and(HtmlButtonElement::disabled)
            {
                return false;
            }
            element.click();
            true
        })
    }

    pub fn install_global_keyboard_navigation() {
        KEY_HANDLER.with(|slot| {
            if slot.borrow().is_some() {
                return;
            }
            let handler = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if event.meta_key() || event.ctrl_key() || event.alt_key() {
                    return;
                }
                let key = event.key();
                let handled = if key.len() == 1
                    && key
                        .chars()
                        .next()
                        .is_some_and(|key| ('1'..='9').contains(&key))
                {
                    let selector = format!(".choice[data-key=\"{key}\"]");
                    click_choice(&selector)
                } else {
                    match key.as_str() {
                        "Enter" => click_first(&[
                            "[data-action=\"validate\"]",
                            "[data-action=\"continue\"]",
                            "[data-action=\"start\"]",
                            "[data-action=\"restart\"]",
                        ]),
                        " " => click_first(&["[data-action=\"idk\"]"]),
                        "r" | "R" => {
                            click_first(&["[data-action=\"replay\"]", "[data-action=\"restart\"]"])
                        }
                        "e" | "E" => click_first(&["[data-action=\"export\"]"]),
                        _ => false,
                    }
                };
                if handled {
                    event.prevent_default();
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            if let Some(document) = document()
                && document
                    .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
                    .is_ok()
            {
                *slot.borrow_mut() = Some(handler);
            }
        });
    }

    pub fn focus_continue() {
        let Some(element) = visible_element("[data-action=\"continue\"]") else {
            return;
        };
        let options = FocusOptions::new();
        options.set_prevent_scroll(true);
        let _ = element.focus_with_options(&options);
    }

    pub fn reset_rum() {
        SELECTED_AT.with(|value| *value.borrow_mut() = None);
        DELAYS.with(|values| values.borrow_mut().clear());
    }

    pub fn mark_selection() {
        let now = window()
            .and_then(|window| window.performance())
            .map(|p| p.now());
        SELECTED_AT.with(|value| *value.borrow_mut() = now);
    }

    pub fn mark_validation() {
        let now = window()
            .and_then(|window| window.performance())
            .map(|p| p.now());
        let selected = SELECTED_AT.with(|value| value.borrow_mut().take());
        if let (Some(now), Some(selected)) = (now, selected) {
            let delay = (now - selected).max(0.0).round() as u64;
            DELAYS.with(|values| {
                let mut values = values.borrow_mut();
                values.push(delay);
                if let (Some(window), Ok(serialized)) = (window(), serde_json::to_string(&*values))
                    && let Ok(Some(storage)) = window.local_storage()
                {
                    let _ = storage.set_item("raip_delays", &serialized);
                }
            });
        }
    }

    pub fn rum_snapshot() -> (Vec<u64>, Option<u64>) {
        DELAYS.with(|values| {
            let values = values.borrow().clone();
            let median = if values.is_empty() {
                None
            } else {
                let mut sorted = values.clone();
                sorted.sort_unstable();
                Some(sorted[sorted.len() / 2])
            };
            (values, median)
        })
    }

    pub fn download_json(contents: &str) {
        let parts = Array::new();
        parts.push(&JsValue::from_str(contents));
        let options = BlobPropertyBag::new();
        options.set_type("application/json");
        let Ok(blob) = Blob::new_with_str_sequence_and_options(&parts, &options) else {
            return;
        };
        let Ok(url) = Url::create_object_url_with_blob(&blob) else {
            return;
        };
        let Some(document) = document() else {
            let _ = Url::revoke_object_url(&url);
            return;
        };
        let Some(body) = document.body() else {
            let _ = Url::revoke_object_url(&url);
            return;
        };
        let Ok(element) = document.create_element("a") else {
            let _ = Url::revoke_object_url(&url);
            return;
        };
        let Ok(anchor) = element.dyn_into::<HtmlAnchorElement>() else {
            let _ = Url::revoke_object_url(&url);
            return;
        };
        anchor.set_href(&url);
        anchor.set_download("rumble-ai-practices-synthese.json");
        if body.append_child(&anchor).is_ok() {
            anchor.click();
            let _ = body.remove_child(&anchor);
        }
        let _ = Url::revoke_object_url(&url);
    }

    pub fn stable_client_id() -> Option<String> {
        const KEY: &str = "raip_cohort_id";
        let window = window()?;
        let storage = window.local_storage().ok()??;
        if let Ok(Some(value)) = storage.get_item(KEY)
            && !value.is_empty()
        {
            return Some(value);
        }
        let value = window.crypto().ok()?.random_uuid();
        storage.set_item(KEY, &value).ok()?;
        Some(value)
    }

    pub fn register_service_worker() {
        if let Some(window) = window() {
            let _ = window.navigator().service_worker().register("./sw.js");
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub fn install_global_keyboard_navigation() {}
    pub fn focus_continue() {}
    pub fn reset_rum() {}
    pub fn mark_selection() {}
    pub fn mark_validation() {}
    pub fn rum_snapshot() -> (Vec<u64>, Option<u64>) {
        (Vec::new(), None)
    }
    pub fn download_json(_contents: &str) {}
    pub fn stable_client_id() -> Option<String> {
        None
    }
    pub fn register_service_worker() {}
}

pub use imp::*;
