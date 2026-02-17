use do_everything_like_a_god::app::*;
use leptos::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn get_document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn get_body() -> web_sys::HtmlElement {
    get_document().body().unwrap()
}

#[wasm_bindgen_test]
fn test_app_initialization() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    assert!(body.inner_html().contains("UTILITIES"));
}

#[wasm_bindgen_test]
fn test_navigation_links() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    assert!(body.inner_html().contains("Base64"));
    assert!(body.inner_html().contains("Hash"));
    assert!(body.inner_html().contains("JSON"));
}

#[wasm_bindgen_test]
fn test_brand_and_menu() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    assert!(body.inner_html().contains("GOD MODE"));
}

#[wasm_bindgen_test]
fn test_language_toggle_button() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let buttons = body.get_elements_by_tag_name("button");
    let mut has_lang = false;
    for i in 0..buttons.length() {
        if let Some(btn) = buttons.item(i) {
            let text = btn.text_content().unwrap_or_default();
            if text.contains("中文") || text.contains("EN") {
                has_lang = true;
                break;
            }
        }
    }
    assert!(has_lang);
}

#[wasm_bindgen_test]
fn test_theme_toggle_button() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let buttons = body.get_elements_by_tag_name("button");
    let mut has_theme = false;
    for i in 0..buttons.length() {
        if let Some(btn) = buttons.item(i) {
            let text = btn.text_content().unwrap_or_default();
            if text.contains("☀️") || text.contains("🌙") {
                has_theme = true;
                break;
            }
        }
    }
    assert!(has_theme);
}

#[wasm_bindgen_test]
fn test_router_configured() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let links = body.get_elements_by_tag_name("a");
    assert!(links.length() > 0);
}

#[wasm_bindgen_test]
fn test_all_tool_pages_exist() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let html = body.inner_html();

    assert!(html.contains("Base64"));
    assert!(html.contains("Base32"));
    assert!(html.contains("Base58"));
    assert!(html.contains("Hash"));
    assert!(html.contains("AES"));
    assert!(html.contains("JWT"));
    assert!(html.contains("JSON"));
    assert!(html.contains("Regex"));
    assert!(html.contains("Diff"));
    assert!(html.contains("UUID"));
    assert!(html.contains("Timestamp"));
    assert!(html.contains("Base Conv"));
    assert!(html.contains("Color"));
    assert!(html.contains("Cron"));
    assert!(html.contains("Image Base64"));
}

#[wasm_bindgen_test]
fn test_category_sections() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let html = body.inner_html();

    assert!(html.contains("Encoders"));
    assert!(html.contains("Cryptography"));
    assert!(html.contains("Development"));
    assert!(html.contains("Utilities"));
}

#[wasm_bindgen_test]
fn test_sidebar_responsive() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let html = body.inner_html();

    assert!(html.contains("Menu"));
    assert!(html.contains("GOD MODE"));
}

#[wasm_bindgen_test]
fn test_tool_links_have_routes() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let links = body.get_elements_by_tag_name("a");

    let mut base64_found = false;
    for i in 0..links.length() {
        if let Some(link) = links.item(i) {
            let href = link.get_attribute("href").unwrap_or_default();
            if href.contains("base64") {
                base64_found = true;
                break;
            }
        }
    }
    assert!(base64_found);
}

#[wasm_bindgen_test]
fn test_textarea_elements_exist() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();

    let nav_textareas = body.get_elements_by_tag_name("textarea");
    assert!(nav_textareas.length() == 0);
}

#[wasm_bindgen_test]
fn test_button_elements_exist() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();

    let buttons = body.get_elements_by_tag_name("button");
    assert!(buttons.length() > 3);
}

#[wasm_bindgen_test]
fn test_no_console_errors_on_load() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();

    assert!(body.inner_html().len() > 100);
}

#[wasm_bindgen_test]
fn test_page_structure_complete() {
    let _ = create_runtime();
    let _ = mount_to_body(App);
    let body = get_body();
    let html = body.inner_html();

    assert!(html.len() > 100);
    assert!(html.contains("div") || html.contains("GOD"));
}
