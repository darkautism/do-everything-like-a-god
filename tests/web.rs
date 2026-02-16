use wasm_bindgen_test::*;
use leptos::*;
use do_everything_like_a_god::app::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_app_initialization() {
    // 測試在瀏覽器環境下是否能正常渲染 App
    let _ = create_runtime();
    let _ = mount_to_body(App);
    
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    
    // 驗證 UTILITIES 導航標題是否存在
    assert!(body.inner_html().contains("UTILITIES"));
}
