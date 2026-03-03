use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::sync::OnceLock;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Lang {
    En,
    Zh,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Theme {
    Dark,
    Light,
}

fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
}

fn setup_keyboard_shortcuts() {
    if let Some(window) = web_sys::window() {
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.ctrl_key() || event.meta_key() && event.key().as_str() == "k" {
                    event.prevent_default();
                }
            }) as Box<dyn FnMut(_)>);
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

#[component]
fn CopyButton(text: ReadSignal<String>) -> impl IntoView {
    let (copied, set_copied) = create_signal(false);

    view! {
        <button
            class="copy-btn"
            on:click=move |_| {
                copy_to_clipboard(&text.get());
                set_copied.set(true);
                set_timeout(move || set_copied.set(false), std::time::Duration::from_millis(2000));
            }
        >
            {move || if copied.get() { "✓" } else { "📋" }}
        </button>
    }
}

#[component]
fn ClearButton(on_click: impl Fn() + 'static) -> impl IntoView {
    view! {
        <button class="clear-btn" on:click=move |_| on_click()>
            "🗑️"
        </button>
    }
}

#[component]
fn ToolHeader(
    lang: ReadSignal<Lang>,
    title_en: &'static str,
    title_zh: &'static str,
) -> impl IntoView {
    view! {
        <h2 style="font-size:3rem;font-weight:900;margin:0">
            {move || match lang.get() {
                Lang::En => title_en,
                Lang::Zh => title_zh,
            }}
        </h2>
    }
}

const ROUTER_BASE: &str = "/do-everything-like-a-god";

static ROUTER_BASE_ONCE: OnceLock<String> = OnceLock::new();

fn resolve_router_base() -> &'static str {
    let s = ROUTER_BASE_ONCE.get_or_init(|| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Ok(Some(base_el)) = doc.query_selector("base") {
                    if let Some(href) = base_el.get_attribute("href") {
                        let parsed = parse_router_base(&href);
                        return parsed;
                    }
                }
            }
        }
        ROUTER_BASE.to_string()
    });
    s.as_str()
}

fn parse_router_base(base_href: &str) -> String {
    let href_lower = base_href.to_ascii_lowercase();
    let mut path = if href_lower.starts_with("http://") || href_lower.starts_with("https://") {
        let without_scheme = base_href
            .find("://")
            .map(|idx| &base_href[idx + 3..])
            .unwrap_or(base_href);
        format!(
            "/{}",
            without_scheme.split_once('/').map(|(_, p)| p).unwrap_or("")
        )
    } else {
        base_href.to_string()
    };

    path = path
        .split(['?', '#'])
        .next()
        .unwrap_or_default()
        .to_string();

    if !path.starts_with('/') && !path.is_empty() {
        path = format!("/{path}");
    }

    if path.ends_with('/') {
        path.pop();
    }

    path
}

fn output_wav_name(input_name: &str) -> String {
    let stem = input_name
        .rsplit_once('.')
        .map(|(base, _)| base)
        .unwrap_or(input_name)
        .trim();
    if stem.is_empty() {
        "converted.wav".to_string()
    } else {
        format!("{stem}.wav")
    }
}

fn convert_audio_to_wav_bytes(input: Vec<u8>) -> Result<Vec<u8>, String> {
    let decoder = rodio::Decoder::try_from(std::io::Cursor::new(input))
        .map_err(|e| format!("Decode failed: {e}"))?;
    let mut writer = std::io::Cursor::new(Vec::new());
    rodio::wav_to_writer(decoder, &mut writer)
        .map_err(|e| format!("WAV conversion failed: {e}"))?;
    Ok(writer.into_inner())
}

#[cfg(test)]
mod router_base_tests {
    use super::parse_router_base;

    #[test]
    fn test_parse_router_base_gh_pages_subpath() {
        assert_eq!(
            parse_router_base("/do-everything-like-a-god/"),
            "/do-everything-like-a-god".to_string()
        );
    }

    #[test]
    fn test_parse_router_base_custom_domain_root() {
        assert_eq!(parse_router_base("/"), "".to_string());
    }

    #[test]
    fn test_parse_router_base_absolute_url() {
        assert_eq!(
            parse_router_base("https://tools.example.com/app/"),
            "/app".to_string()
        );
    }

    #[test]
    fn test_parse_router_base_relative_path() {
        assert_eq!(parse_router_base("app/"), "/app".to_string());
    }

    #[test]
    fn test_parse_router_base_with_port_query_and_fragment() {
        assert_eq!(
            parse_router_base("http://localhost:8080/app/?x=1#y"),
            "/app".to_string()
        );
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (lang, set_lang) = create_signal(Lang::Zh);
    let (theme, set_theme) = create_signal(Theme::Dark);
    let (is_sidebar_open, set_sidebar_open) = create_signal(false);

    create_effect(move |_| {
        setup_keyboard_shortcuts();
    });

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let storage = window.local_storage().ok().flatten();
            if let Some(storage) = storage {
                if let Ok(Some(theme_val)) = storage.get_item("theme") {
                    if theme_val == "light" {
                        set_theme.set(Theme::Light);
                    } else {
                        set_theme.set(Theme::Dark);
                    }
                }
            }
        }
    });

    let toggle_theme = move |_| {
        let new_theme = if theme.get() == Theme::Dark {
            Theme::Light
        } else {
            Theme::Dark
        };
        set_theme.set(new_theme);

        if let Some(window) = web_sys::window() {
            let storage = window.local_storage().ok().flatten();
            if let Some(storage) = storage {
                let theme_str = if new_theme == Theme::Dark {
                    "dark"
                } else {
                    "light"
                };
                let _ = storage.set_item("theme", theme_str);
            }
        }
    };

    view! {
        <Title text="工具箱 | Useful Tools"/>

        <Router base=resolve_router_base() trailing_slash=TrailingSlash::Redirect>
            <div class=move || format!("layout {}", match theme.get() { Theme::Light => "light", Theme::Dark => "" })>
                <div class="mobile-header">
                    <button class="menu-toggle" aria-label="Toggle menu" on:click=move |_| set_sidebar_open.update(|v| *v = !*v)>
                        "Menu"
                    </button>
                    <div class="mobile-brand" aria-label="Utility Tools">"UTILITIES"</div>
                </div>

                <nav class=move || if is_sidebar_open.get() { "sidebar open" } else { "sidebar" } aria-label="Main navigation">
                    <div class="sidebar-header">
                        <A href="" class="brand" on:click=move |_| set_sidebar_open.set(false)>"GOD MODE"</A>
                        <div class="header-buttons">
                            <a href="https://ko-fi.com/kautism" target="_blank" rel="noopener" class="support-btn kofi" aria-label="Support on Ko-fi">"☕"</a>
                            <a href="https://paypal.me/kautism" target="_blank" rel="noopener" class="support-btn paypal" aria-label="Support on PayPal">"💳"</a>
                            <button class="theme-switch" aria-label="Toggle theme" on:click=toggle_theme>
                                {move || match theme.get() { Theme::Dark => "☀️", Theme::Light => "🌙" }}
                            </button>
                            <button class="lang-switch" aria-label="Toggle language" on:click=move |_| {
                                set_lang.update(|l| *l = if *l == Lang::En { Lang::Zh } else { Lang::En });
                            }>
                                {move || match lang.get() { Lang::En => "中文", Lang::Zh => "EN", }}
                            </button>
                        </div>
                    </div>

                    <div class="category">
                        <div class="category-title">"Convert"</div>
                        <A href="audio-convert" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Audio Convert"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Encoders"</div>
                        <A href="base64" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base64"</A>
                        <A href="base32" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base32"</A>
                        <A href="base58" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base58"</A>
                        <A href="html-escape" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"HTML Escape"</A>
                        <A href="url-escape" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"URL Escape"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Cryptography"</div>
                        <A href="hash" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Hash"</A>
                        <A href="aes" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"AES"</A>
                        <A href="jwt" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"JWT"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Development"</div>
                        <A href="json" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"JSON"</A>
                        <A href="regex" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Regex"</A>
                        <A href="diff" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Diff"</A>
                        <A href="uuid" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"UUID"</A>
                        <A href="timestamp" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Timestamp"</A>
                        <A href="base-conv" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base Conv"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Utilities"</div>
                        <A href="cron" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Cron"</A>
                        <A href="image-base64" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Image Base64"</A>
                    </div>
                </nav>

                <div class=move || if is_sidebar_open.get() { "overlay show" } else { "overlay" }
                     on:click=move |_| set_sidebar_open.set(false)></div>

                <main class="main-content">
                    <Routes>
                        <Route path="" view=move || view! { <HomePage lang=lang /> }/>
                        <Route path="/audio-convert" view=move || view! { <AudioConvertPage lang=lang /> }/>
                        <Route path="/base64" view=move || view! { <Base64Page lang=lang /> }/>
                        <Route path="/base32" view=move || view! { <Base32Page lang=lang /> }/>
                        <Route path="/base58" view=move || view! { <Base58Page lang=lang /> }/>
                        <Route path="/html-escape" view=move || view! { <HtmlEscapePage lang=lang /> }/>
                        <Route path="/url-escape" view=move || view! { <UrlEscapePage lang=lang /> }/>
                        <Route path="/json" view=move || view! { <JsonPage lang=lang /> }/>
                        <Route path="/hash" view=move || view! { <HashPage lang=lang /> }/>
                        <Route path="/aes" view=move || view! { <AesPage lang=lang /> }/>
                        <Route path="/jwt" view=move || view! { <JwtPage lang=lang /> }/>
                        <Route path="/uuid" view=move || view! { <UuidPage lang=lang /> }/>
                        <Route path="/regex" view=move || view! { <RegexPage lang=lang /> }/>
                        <Route path="/timestamp" view=move || view! { <TimestampPage lang=lang /> }/>
                        <Route path="/base-conv" view=move || view! { <BaseConvPage lang=lang /> }/>
                        <Route path="/diff" view=move || view! { <DiffPage lang=lang /> }/>
                        <Route path="/cron" view=move || view! { <CronPage lang=lang /> }/>
                        <Route path="/image-base64" view=move || view! { <ImageBase64Page lang=lang /> }/>
                        <Route path="/*" view=move || view! { <HomePage lang=lang /> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <div class="hero">
            <h1>
                {move || match lang.get() {
                    Lang::En => "Do Everything Like a God",
                    Lang::Zh => "做甚麼都有如神助",
                }}
            </h1>
            <p>
                {move || match lang.get() {
                    Lang::En => "Empowering your workflow with divine efficiency.",
                    Lang::Zh => "賦予你的工作流神一般的效率。",
                }}
            </p>
            <a href="#" class="btn">
                {move || match lang.get() {
                    Lang::En => "Get Started",
                    Lang::Zh => "立即開始",
                }}
            </a>
        </div>
    }
}

// ==================== Base64 Page ====================
#[component]
fn Base64Page(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{engine::general_purpose, Engine as _};

    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let encode = move |_| {
        set_error.set(None);
        let encoded = general_purpose::STANDARD.encode(input.get().as_bytes());
        set_output.set(encoded);
    };

    let decode = move |_| {
        set_error.set(None);
        match general_purpose::STANDARD.decode(output.get().trim()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => set_input.set(s),
                Err(e) => set_error.set(Some(format!("UTF-8 Error: {}", e))),
            },
            Err(e) => set_error.set(Some(format!("Decode Error: {}", e))),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Base64" title_zh="Base64 工具"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Text", Lang::Zh => "文字", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/>
                    </div>
                    <textarea
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=encode>{move || match lang.get() { Lang::En => "Encode →", Lang::Zh => "編碼 →", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">"Base64"</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea
                        prop:value=output
                        on:input=move |ev| set_output.set(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=decode>{move || match lang.get() { Lang::En => "← Decode", Lang::Zh => "← 解碼", }}</button>
                    </div>
                </div>
            </div>
            {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
        </div>
    }
}

// ==================== Base32 Page ====================
#[component]
fn Base32Page(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let encode = move |_| {
        set_error.set(None);
        let encoded = base32::encode(base32::Alphabet::Crockford, input.get().as_bytes());
        set_output.set(encoded);
    };

    let decode = move |_| {
        set_error.set(None);
        match base32::decode(base32::Alphabet::Crockford, output.get().trim()) {
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(s) => set_input.set(s),
                Err(e) => set_error.set(Some(format!("UTF-8 Error: {}", e))),
            },
            None => set_error.set(Some("Invalid Base32".to_string())),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Base32" title_zh="Base32 工具"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Text", Lang::Zh => "文字", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=encode>{move || match lang.get() { Lang::En => "Encode →", Lang::Zh => "編碼 →", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">"Base32"</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea prop:value=output on:input=move |ev| set_output.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=decode>{move || match lang.get() { Lang::En => "← Decode", Lang::Zh => "← 解碼", }}</button>
                    </div>
                </div>
            </div>
            {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
        </div>
    }
}

// ==================== Base58 Page ====================
#[component]
fn Base58Page(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let encode = move |_| {
        set_error.set(None);
        let encoded = bs58::encode(input.get().as_bytes()).into_string();
        set_output.set(encoded);
    };

    let decode = move |_| {
        set_error.set(None);
        match bs58::decode(output.get().trim()).into_vec() {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => set_input.set(s),
                Err(e) => set_error.set(Some(format!("UTF-8 Error: {}", e))),
            },
            Err(e) => set_error.set(Some(format!("Decode Error: {}", e))),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Base58" title_zh="Base58 工具"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Text", Lang::Zh => "文字", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=encode>{move || match lang.get() { Lang::En => "Encode →", Lang::Zh => "編碼 →", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">"Base58"</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea prop:value=output on:input=move |ev| set_output.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=decode>{move || match lang.get() { Lang::En => "← Decode", Lang::Zh => "← 解碼", }}</button>
                    </div>
                </div>
            </div>
            {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
        </div>
    }
}

// ==================== HTML Escape Page ====================
#[component]
fn HtmlEscapePage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());

    let escape = move |_| {
        let escaped = html_escape::encode_safe(&input.get()).to_string();
        set_output.set(escaped);
    };

    let unescape = move |_| {
        let unescaped = html_escape::decode_html_entities(&output.get()).to_string();
        set_input.set(unescaped);
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="HTML Escape" title_zh="HTML 轉義"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Raw HTML", Lang::Zh => "原始 HTML", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder="<div>...</div>"></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=escape>{move || match lang.get() { Lang::En => "Escape →", Lang::Zh => "轉義 →", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Escaped", Lang::Zh => "轉義結果", }}</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea prop:value=output on:input=move |ev| set_output.set(event_target_value(&ev)) placeholder="&lt;div&gt;..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=unescape>{move || match lang.get() { Lang::En => "← Unescape", Lang::Zh => "← 還原", }}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ==================== URL Escape Page ====================
#[component]
fn UrlEscapePage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());

    let encode = move |_| {
        let encoded = urlencoding::encode(&input.get()).to_string();
        set_output.set(encoded);
    };

    let decode = move |_| {
        if let Ok(decoded) = urlencoding::decode(&output.get()) {
            set_input.set(decoded.into_owned());
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="URL Encode" title_zh="URL 編碼"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Raw URL", Lang::Zh => "原始 URL", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder="https://example.com/測試"></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=encode>{move || match lang.get() { Lang::En => "Encode →", Lang::Zh => "編碼 →", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Encoded", Lang::Zh => "編碼結果", }}</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea prop:value=output on:input=move |ev| set_output.set(event_target_value(&ev)) placeholder="https%3A%2F%2F..."></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=decode>{move || match lang.get() { Lang::En => "← Decode", Lang::Zh => "← 解碼", }}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ==================== JSON Page ====================
#[component]
fn JsonPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let process = move |minify: bool| {
        set_error.set(None);
        match serde_json::from_str::<serde_json::Value>(&input.get()) {
            Ok(v) => {
                let res = if minify {
                    serde_json::to_string(&v).unwrap()
                } else {
                    serde_json::to_string_pretty(&v).unwrap()
                };
                set_output.set(res);
            }
            Err(e) => set_error.set(Some(e.to_string())),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="JSON Tool" title_zh="JSON 工具"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Input", Lang::Zh => "輸入", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); set_error.set(None); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder=r#"{"key":"value"}"#></textarea>
                    <div class="btn-row">
                        <button class="btn" on:click=move |_| process(false)>{move || match lang.get() { Lang::En => "Prettify", Lang::Zh => "格式化", }}</button>
                        <button class="btn" on:click=move |_| process(true)>{move || match lang.get() { Lang::En => "Minify", Lang::Zh => "壓縮", }}</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Result", Lang::Zh => "結果", }}</div>
                        <CopyButton text=output/>
                    </div>
                    <textarea prop:value=output readonly placeholder="..."></textarea>
                    {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
                </div>
            </div>
        </div>
    }
}
// ==================== Hash Page ====================
#[component]
fn HashPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use md5::Md5;
    use sha1::Sha1;
    use sha2::{Digest, Sha256, Sha512};
    use sha3::Sha3_256;

    let (input, set_input) = create_signal(String::new());
    let (md5_res, set_md5) = create_signal(String::new());
    let (sha1_res, set_sha1) = create_signal(String::new());
    let (sha256_res, set_sha256) = create_signal(String::new());
    let (sha512_res, set_sha512) = create_signal(String::new());
    let (sha3_res, set_sha3) = create_signal(String::new());
    let (is_loading, set_loading) = create_signal(false);

    let compute_hashes = move |data: &[u8]| {
        let mut md5_hasher = Md5::new();
        md5_hasher.update(data);
        set_md5.set(hex::encode(md5_hasher.finalize()));

        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(data);
        set_sha1.set(hex::encode(sha1_hasher.finalize()));

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(data);
        set_sha256.set(hex::encode(sha256_hasher.finalize()));

        let mut sha512_hasher = Sha512::new();
        sha512_hasher.update(data);
        set_sha512.set(hex::encode(sha512_hasher.finalize()));

        let mut sha3_hasher = Sha3_256::new();
        sha3_hasher.update(data);
        set_sha3.set(hex::encode(sha3_hasher.finalize()));
    };

    let on_text_input = move |val: String| {
        set_input.set(val.clone());
        if val.is_empty() {
            set_md5.set(String::new());
            set_sha1.set(String::new());
            set_sha256.set(String::new());
            set_sha512.set(String::new());
            set_sha3.set(String::new());
            return;
        }
        compute_hashes(val.as_bytes());
    };

    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(files) = target.files() {
            if let Some(file) = files.get(0) {
                set_loading.set(true);
                let reader = web_sys::FileReader::new().unwrap();
                let reader_c = reader.clone();
                let onload =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        let array_buffer = reader_c.result().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let bytes = uint8_array.to_vec();
                        compute_hashes(&bytes);
                        set_loading.set(false);
                    })
                        as Box<dyn FnMut(_)>);
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload.forget();
            }
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Hash" title_zh="Hash 工具"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header">
                        <div class="box-label">{move || match lang.get() { Lang::En => "Input", Lang::Zh => "輸入", }}</div>
                        <ClearButton on_click=move || { set_input.set(String::new()); set_md5.set(String::new()); set_sha1.set(String::new()); set_sha256.set(String::new()); set_sha512.set(String::new()); set_sha3.set(String::new()); }/>
                    </div>
                    <textarea prop:value=input on:input=move |ev| on_text_input(event_target_value(&ev)) placeholder="..."></textarea>
                </div>
                <div class="box">
                    <div class="box-label">{move || match lang.get() { Lang::En => "File Upload", Lang::Zh => "上傳檔案", }}</div>
                    <input type="file" on:change=on_file_change class="file-input"/>
                    {move || if is_loading.get() { view! { <div class="loading">"..."</div> } } else { view! { <div></div> } }}
                </div>
            </div>
            <div class="hash-results">
                <div class="box"><div class="box-header"><div class="box-label">"MD5"</div><CopyButton text=md5_res/></div><input type="text" prop:value=md5_res readonly class="hash-output"/></div>
                <div class="box"><div class="box-header"><div class="box-label">"SHA1"</div><CopyButton text=sha1_res/></div><input type="text" prop:value=sha1_res readonly class="hash-output"/></div>
                <div class="box"><div class="box-header"><div class="box-label">"SHA256"</div><CopyButton text=sha256_res/></div><input type="text" prop:value=sha256_res readonly class="hash-output"/></div>
                <div class="box"><div class="box-header"><div class="box-label">"SHA512"</div><CopyButton text=sha512_res/></div><input type="text" prop:value=sha512_res readonly class="hash-output"/></div>
                <div class="box"><div class="box-header"><div class="box-label">"SHA3-256"</div><CopyButton text=sha3_res/></div><input type="text" prop:value=sha3_res readonly class="hash-output"/></div>
            </div>
        </div>
    }
}

// ==================== AES Page ====================
#[component]
fn AesPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
    use rand::Rng;

    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (key, set_key) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let generate_key = move |_| {
        let random_key: [u8; 32] = rand::thread_rng().gen();
        set_key.set(hex::encode(random_key));
    };

    let encrypt = move |_| {
        set_error.set(None);
        let key_bytes = match hex::decode(key.get().trim()) {
            Ok(b) if b.len() == 32 => b,
            _ => {
                set_error.set(Some("Key must be 32 bytes (64 hex chars)".into()));
                return;
            }
        };
        let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();
        let nonce = Nonce::from_slice(b"unique nonce");
        match cipher.encrypt(nonce, input.get().as_bytes()) {
            Ok(encrypted) => set_output.set(hex::encode(encrypted)),
            Err(e) => set_error.set(Some(e.to_string())),
        }
    };

    let decrypt = move |_| {
        set_error.set(None);
        let key_bytes = match hex::decode(key.get().trim()) {
            Ok(b) if b.len() == 32 => b,
            _ => {
                set_error.set(Some("Key must be 32 bytes (64 hex chars)".into()));
                return;
            }
        };
        let encrypted = match hex::decode(output.get().trim()) {
            Ok(b) => b,
            Err(e) => {
                set_error.set(Some(format!("Invalid hex: {}", e)));
                return;
            }
        };
        let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();
        let nonce = Nonce::from_slice(b"unique nonce");
        match cipher.decrypt(nonce, encrypted.as_slice()) {
            Ok(decrypted) => match String::from_utf8(decrypted) {
                Ok(s) => set_input.set(s),
                Err(e) => set_error.set(Some(format!("UTF-8 Error: {}", e))),
            },
            Err(e) => set_error.set(Some(e.to_string())),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="AES" title_zh="AES 加密"/>
            <div class="box" style="margin-bottom:20px">
                <div class="box-header">
                    <div class="box-label">{move || match lang.get() { Lang::En => "Key (64 hex chars)", Lang::Zh => "密鑰 (64位十六進制)", }}</div>
                    <CopyButton text=key/>
                </div>
                <div class="btn-row">
                    <input type="text" prop:value=key on:input=move |ev| set_key.set(event_target_value(&ev)) class="key-input" placeholder="64 hex chars"/>
                    <button class="btn" on:click=generate_key>{move || match lang.get() { Lang::En => "Generate", Lang::Zh => "生成", }}</button>
                </div>
            </div>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header"><div class="box-label">{move || match lang.get() { Lang::En => "Plaintext", Lang::Zh => "明文", }}</div><ClearButton on_click=move || { set_input.set(String::new()); set_output.set(String::new()); }/></div>
                    <textarea prop:value=input on:input=move |ev| set_input.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row"><button class="btn" on:click=encrypt>{move || match lang.get() { Lang::En => "Encrypt →", Lang::Zh => "加密 →", }}</button></div>
                </div>
                <div class="box">
                    <div class="box-header"><div class="box-label">{move || match lang.get() { Lang::En => "Ciphertext", Lang::Zh => "密文", }}</div><CopyButton text=output/></div>
                    <textarea prop:value=output on:input=move |ev| set_output.set(event_target_value(&ev)) placeholder="..."></textarea>
                    <div class="btn-row"><button class="btn" on:click=decrypt>{move || match lang.get() { Lang::En => "← Decrypt", Lang::Zh => "← 解密", }}</button></div>
                </div>
            </div>
            {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
        </div>
    }
}

// ==================== JWT Page ====================
#[component]
fn JwtPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{engine::general_purpose, Engine as _};
    use hmac::{Hmac, Mac};

    type HmacSha256 = Hmac<sha2::Sha256>;

    let (input, set_input) = create_signal(String::new());
    let (secret, set_secret) = create_signal(String::new());
    let (header, set_header) = create_signal(String::new());
    let (payload, set_payload) = create_signal(String::new());
    let (signature, set_signature) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let decode = move |val: String| {
        set_input.set(val.clone());
        set_error.set(None);
        set_header.set(String::new());
        set_payload.set(String::new());
        set_signature.set(String::new());

        let parts: Vec<&str> = val.split('.').collect();
        if parts.len() < 2 {
            if !val.is_empty() {
                set_error.set(Some("Invalid JWT format".into()));
            }
            return;
        }

        let decode_part = |part: &str| -> Result<String, String> {
            let bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(part)
                .map_err(|e| format!("Base64 Error: {}", e))?;
            let json_str = String::from_utf8(bytes).map_err(|e| format!("UTF-8 Error: {}", e))?;
            let val: serde_json::Value =
                serde_json::from_str(&json_str).map_err(|e| format!("JSON Error: {}", e))?;
            Ok(serde_json::to_string_pretty(&val).unwrap())
        };

        match decode_part(parts[0]) {
            Ok(h) => set_header.set(h),
            Err(e) => {
                set_error.set(Some(format!("Header: {}", e)));
                return;
            }
        }
        if let Ok(p) = decode_part(parts[1]) {
            set_payload.set(p);
        }

        if parts.len() >= 3 && !secret.get().is_empty() {
            let secret_key = secret.get();
            let message = format!("{}.{}", parts[0], parts[1]);

            let mac = HmacSha256::new_from_slice(secret_key.as_bytes())
                .map_err(|e| format!("HMAC error: {}", e));

            match mac {
                Ok(mut m) => {
                    m.update(message.as_bytes());
                    let result = m.finalize().into_bytes();
                    let expected_sig = general_purpose::URL_SAFE_NO_PAD.encode(&result[..]);
                    let actual_sig = parts[2].to_string();

                    if expected_sig == actual_sig {
                        set_signature.set("✅ Signature Valid".to_string());
                    } else {
                        set_signature.set("❌ Signature Invalid".to_string());
                    }
                }
                Err(e) => {
                    set_signature.set(format!("Error: {}", e));
                }
            }
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="JWT Decoder" title_zh="JWT 解碼"/>
            <div class="box" style="margin-bottom:20px">
                <div class="box-header">
                    <div class="box-label">{move || match lang.get() { Lang::En => "Secret (for signature verification)", Lang::Zh => "密鑰 (用於簽名驗證)", }}</div>
                </div>
                <input
                    type="text"
                    class="secret-input"
                    prop:value=secret
                    on:input=move |ev| {
                        set_secret.set(event_target_value(&ev));
                        decode(input.get());
                    }
                    placeholder={move || match lang.get() { Lang::En => "Enter secret key", Lang::Zh => "輸入密鑰", }}
                />
            </div>
            <div class="box">
                <div class="box-header">
                    <div class="box-label">"JWT Token"</div>
                    <ClearButton on_click=move || { set_input.set(String::new()); set_header.set(String::new()); set_payload.set(String::new()); set_signature.set(String::new()); set_error.set(None); }/>
                </div>
                <textarea prop:value=input on:input=move |ev| decode(event_target_value(&ev)) placeholder="eyJhbGci..."></textarea>
                {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
            </div>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header"><div class="box-label">"Header"</div><CopyButton text=header/></div>
                    <textarea prop:value=header readonly placeholder="..."></textarea>
                </div>
                <div class="box">
                    <div class="box-header"><div class="box-label">"Payload"</div><CopyButton text=payload/></div>
                    <textarea prop:value=payload readonly placeholder="..."></textarea>
                </div>
            </div>
            <div class="box">
                <div class="box-header"><div class="box-label">{move || match lang.get() { Lang::En => "Signature", Lang::Zh => "簽名", }}</div></div>
                <div class="jwt-signature">{signature}</div>
            </div>
        </div>
    }
}

// ==================== UUID Page ====================
#[component]
fn UuidPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (uuid_v4, set_uuid_v4) = create_signal(String::new());

    let generate = move |_| {
        set_uuid_v4.set(uuid::Uuid::new_v4().to_string());
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="UUID Generator" title_zh="UUID 生成器"/>
            <div class="box" style="text-align:center">
                <div class="box-label">"UUID v4"</div>
                <div class="uuid-display">
                    {move || { let current = uuid_v4.get(); if current.is_empty() { "Click to generate".into() } else { current } }}
                </div>
                <div class="btn-row" style="justify-content:center;margin-top:20px">
                    <button class="btn" on:click=generate>{move || match lang.get() { Lang::En => "Generate", Lang::Zh => "生成", }}</button>
                    <CopyButton text=uuid_v4/>
                </div>
            </div>
        </div>
    }
}

// ==================== Regex Page ====================
#[component]
fn RegexPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use regex::Regex;

    let (pattern, set_pattern) = create_signal(String::new());
    let (text, set_text) = create_signal(String::new());
    let (result, set_result) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let test_regex = move |_| {
        set_error.set(None);
        match Regex::new(&pattern.get()) {
            Ok(re) => {
                let matches: Vec<_> = re
                    .find_iter(&text.get())
                    .map(|m| m.as_str().to_string())
                    .collect();
                set_result.set(if matches.is_empty() {
                    "No match".into()
                } else {
                    matches.join(", ")
                });
            }
            Err(e) => set_error.set(Some(e.to_string())),
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Regex Tester" title_zh="正則測試"/>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "Pattern", Lang::Zh => "正則表達式", }}</div>
                <input type="text" prop:value=pattern on:input=move |ev| { set_pattern.set(event_target_value(&ev)); test_regex(()); } placeholder="^[a-z]+$" class="regex-input"/>
                {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
            </div>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "Test Text", Lang::Zh => "測試文本", }}</div>
                <textarea prop:value=text on:input=move |ev| { set_text.set(event_target_value(&ev)); test_regex(()); } placeholder="..."></textarea>
                <div class="regex-result">{result}</div>
            </div>
        </div>
    }
}

// ==================== Timestamp Page ====================
#[component]
fn TimestampPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use chrono::{TimeZone, Utc};

    let (ts, set_ts) = create_signal(Utc::now().timestamp().to_string());
    let (iso, set_iso) = create_signal(String::new());

    let convert = move || {
        let s = ts.get();
        match s.parse::<i64>() {
            Ok(val) => match Utc.timestamp_opt(val, 0).single() {
                Some(dt) => set_iso.set(dt.to_rfc3339()),
                None => set_iso.set("Invalid".to_string()),
            },
            Err(_) => set_iso.set("Invalid".to_string()),
        }
    };

    let now_ts = move |_| {
        set_ts.set(Utc::now().timestamp().to_string());
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Timestamp" title_zh="時間戳"/>
            <div class="box">
                <div class="box-label">"Unix Timestamp"</div>
                <div class="btn-row">
                    <input type="text" prop:value=ts on:input=move |ev| { set_ts.set(event_target_value(&ev)); convert(); } class="ts-input"/>
                    <button class="btn" on:click=now_ts>{move || match lang.get() { Lang::En => "Now", Lang::Zh => "現在", }}</button>
                </div>
            </div>
            <div class="box">
                <div class="box-label">"ISO 8601"</div>
                <div class="iso-display">{iso}</div>
            </div>
        </div>
    }
}

// ==================== BaseConv Page ====================
#[component]
fn BaseConvPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (val, set_val) = create_signal(String::new());
    let (from_base, set_from) = create_signal(10u32);

    let get_val = move |base: u32| {
        let current = val.get();
        if current.is_empty() {
            return String::new();
        }
        u128::from_str_radix(&current, from_base.get())
            .map(|n| match base {
                2 => format!("{:b}", n),
                8 => format!("{:o}", n),
                10 => format!("{}", n),
                16 => format!("{:x}", n),
                _ => String::new(),
            })
            .unwrap_or_else(|_| "Error".into())
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Base Converter" title_zh="進制轉換"/>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "Input Value", Lang::Zh => "輸入值", }}</div>
                <div class="btn-row">
                    <input type="text" on:input=move |ev| set_val.set(event_target_value(&ev)) class="base-input"/>
                    <select on:change=move |ev| set_from.set(event_target_value(&ev).parse().unwrap()) class="base-select">
                        <option value="10">"Dec"</option>
                        <option value="16">"Hex"</option>
                        <option value="2">"Bin"</option>
                        <option value="8">"Oct"</option>
                    </select>
                </div>
            </div>
            <div class="tool-grid">
                <div class="box"><div class="box-label">"Decimal"</div><div class="base-result">{move || get_val(10)}</div></div>
                <div class="box"><div class="box-label">"Hex"</div><div class="base-result">{move || get_val(16)}</div></div>
                <div class="box"><div class="box-label">"Binary"</div><div class="base-result">{move || get_val(2)}</div></div>
                <div class="box"><div class="box-label">"Octal"</div><div class="base-result">{move || get_val(8)}</div></div>
            </div>
        </div>
    }
}

// ==================== Diff Page ====================
#[component]
fn DiffPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use similar::{ChangeTag, TextDiff};

    let (old_text, set_old) = create_signal(String::new());
    let (new_text, set_new) = create_signal(String::new());

    let diff_view = move || {
        let old = old_text.get();
        let new = new_text.get();
        let diff = TextDiff::from_lines(&old, &new);
        diff.iter_all_changes().map(|change| {
            let (sign, color) = match change.tag() {
                ChangeTag::Delete => ("-", "#f00"),
                ChangeTag::Insert => ("+", "#0f0"),
                ChangeTag::Equal => (" ", "#888"),
            };
            view! { <div style=format!("color:{}", color)> {sign} {change.value().to_string()} </div> }
        }).collect_view()
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Diff" title_zh="文本比對"/>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">"Original"</div>
                    <textarea on:input=move |ev| set_old.set(event_target_value(&ev))></textarea>
                </div>
                <div class="box">
                    <div class="box-label">"Modified"</div>
                    <textarea on:input=move |ev| set_new.set(event_target_value(&ev))></textarea>
                </div>
            </div>
            <div class="box diff-output">{diff_view}</div>
        </div>
    }
}

// ==================== Cron Page ====================
#[component]
fn CronPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (cron_expr, set_cron) = create_signal(String::new());
    let (description, set_desc) = create_signal(String::new());

    let parse_cron = move |_| {
        let expr = cron_expr.get();
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() < 5 {
            set_desc.set("Invalid cron expression".to_string());
            return;
        }

        let minute = parts[0];
        let hour = parts[1];
        let day = parts[2];
        let month = parts[3];
        let weekday = parts[4];

        let desc = format!(
            "Minute: {}, Hour: {}, Day: {}, Month: {}, Weekday: {}",
            minute, hour, day, month, weekday
        );
        set_desc.set(desc);
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Cron Parser" title_zh="Cron 解析"/>
            <div class="box">
                <div class="box-label">"Cron Expression"</div>
                <div class="btn-row">
                    <input type="text" prop:value=cron_expr on:input=move |ev| set_cron.set(event_target_value(&ev)) placeholder="* * * * *" class="cron-input"/>
                    <button class="btn" on:click=move |_| parse_cron(())>{move || match lang.get() { Lang::En => "Parse", Lang::Zh => "解析", }}</button>
                </div>
            </div>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "Description", Lang::Zh => "描述", }}</div>
                <div class="cron-desc">{description}</div>
            </div>
        </div>
    }
}

// ==================== Audio Convert Page ====================
#[component]
fn AudioConvertPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (download_url, set_download_url) = create_signal(String::new());
    let (output_name, set_output_name) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);
    let (is_loading, set_loading) = create_signal(false);
    let (drag_active, set_drag_active) = create_signal(false);

    on_cleanup(move || {
        let stale_url = download_url.get_untracked();
        if !stale_url.is_empty() {
            let _ = web_sys::Url::revoke_object_url(&stale_url);
        }
    });

    let process_file: std::rc::Rc<dyn Fn(web_sys::File)> =
        std::rc::Rc::new(move |file: web_sys::File| {
            set_error.set(None);
            set_loading.set(true);
            set_status.set(format!("Converting {}", file.name()));
            let stale_url = download_url.get_untracked();
            if !stale_url.is_empty() {
                let _ = web_sys::Url::revoke_object_url(&stale_url);
            }
            set_download_url.set(String::new());
            set_output_name.set(String::new());

            let output_file_name = output_wav_name(&file.name());
            let original_size = file.size() as usize;
            let reader = match web_sys::FileReader::new() {
                Ok(r) => r,
                Err(_) => {
                    set_error.set(Some("Failed to initialize FileReader.".to_string()));
                    set_loading.set(false);
                    return;
                }
            };
            let reader_c = reader.clone();

            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                let array_buffer = match reader_c.result() {
                    Ok(buf) => buf,
                    Err(_) => {
                        set_error.set(Some("Failed to read file bytes.".to_string()));
                        set_loading.set(false);
                        return;
                    }
                };
                let input_bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
                match convert_audio_to_wav_bytes(input_bytes) {
                    Ok(wav_bytes) => {
                        let stale_url = download_url.get_untracked();
                        if !stale_url.is_empty() {
                            let _ = web_sys::Url::revoke_object_url(&stale_url);
                        }

                        let wav_array = js_sys::Uint8Array::from(wav_bytes.as_slice());
                        let chunks = js_sys::Array::new();
                        chunks.push(&wav_array.buffer());
                        match web_sys::Blob::new_with_u8_array_sequence(&chunks)
                            .and_then(|blob| web_sys::Url::create_object_url_with_blob(&blob))
                        {
                            Ok(url) => {
                                set_download_url.set(url);
                                set_output_name.set(output_file_name.clone());
                                set_status.set(format!(
                                    "Input {} bytes -> Output {} bytes",
                                    original_size,
                                    wav_bytes.len()
                                ));
                            }
                            Err(_) => {
                                set_error
                                    .set(Some("Failed to build downloadable file.".to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                    }
                }

                set_loading.set(false);
            }) as Box<dyn FnMut(_)>);

            let onerror =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    set_error.set(Some("Failed to read file.".to_string()));
                    set_loading.set(false);
                }) as Box<dyn FnMut(_)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            if reader.read_as_array_buffer(&file).is_err() {
                set_error.set(Some("Failed to start file read.".to_string()));
                set_loading.set(false);
            }
            onload.forget();
            onerror.forget();
        });

    let process_file_for_input = process_file.clone();
    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(files) = target.files() {
            if let Some(file) = files.get(0) {
                process_file_for_input(file);
            }
        }
    };

    let process_file_for_drop = process_file.clone();
    let on_drop = move |ev: ev::DragEvent| {
        ev.prevent_default();
        set_drag_active.set(false);
        if let Some(files) = ev.data_transfer().and_then(|transfer| transfer.files()) {
            if let Some(file) = files.get(0) {
                process_file_for_drop(file);
            }
        }
    };

    let on_drag_over = move |ev: ev::DragEvent| {
        ev.prevent_default();
        set_drag_active.set(true);
    };
    let on_drag_leave = move |ev: ev::DragEvent| {
        ev.prevent_default();
        set_drag_active.set(false);
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Audio Converter" title_zh="音訊轉換器"/>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "rodio capability", Lang::Zh => "rodio 能力說明", }}</div>
                <div class="convert-meta">{move || match lang.get() {
                    Lang::En => "Input decode support in this build: FLAC, MP3, MP4/AAC, OGG/Vorbis, WAV.",
                    Lang::Zh => "此版本可解碼輸入：FLAC、MP3、MP4/AAC、OGG/Vorbis、WAV。",
                }}</div>
                <div class="convert-meta">{move || match lang.get() {
                    Lang::En => "Output support: WAV only (rodio has no general multi-format encoder API).",
                    Lang::Zh => "輸出目前僅支援 WAV（rodio 沒有通用多格式編碼 API）。",
                }}</div>
            </div>
            <div class="box">
                <div
                    class=move || if drag_active.get() { "drop-zone active" } else { "drop-zone" }
                    on:dragover=on_drag_over
                    on:dragleave=on_drag_leave
                    on:drop=on_drop
                >
                    {move || match lang.get() {
                        Lang::En => "Drop an audio file here",
                        Lang::Zh => "將音訊檔拖曳到這裡",
                    }}
                </div>
                <div class="convert-separator">{move || match lang.get() { Lang::En => "or upload", Lang::Zh => "或上傳檔案", }}</div>
                <input type="file" accept=".flac,.mp3,.mp4,.m4a,.ogg,.wav,audio/*" on:change=on_file_change class="file-input"/>
                {move || if is_loading.get() { view! { <div class="loading">"..."</div> } } else { view! { <div></div> } }}
                <div class="convert-meta">{status}</div>
                {move || error.get().map(|e| view! { <div class="error">{e}</div> })}
                {move || {
                    if download_url.get().is_empty() {
                        view! { <div></div> }.into_view()
                    } else {
                        view! {
                            <div>
                                <a class="btn download-btn" href=download_url download=move || output_name.get()>
                                    {move || match lang.get() { Lang::En => "Download WAV", Lang::Zh => "下載 WAV", }}
                                </a>
                            </div>
                        }
                            .into_view()
                    }
                }}
            </div>
        </div>
    }
}

// ==================== Image Base64 Page ====================
#[component]
fn ImageBase64Page(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{engine::general_purpose, Engine as _};

    let (base64_out, set_base64) = create_signal(String::new());
    let (data_uri, set_data_uri) = create_signal(String::new());
    let (is_loading, set_loading) = create_signal(false);

    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(files) = target.files() {
            if let Some(file) = files.get(0) {
                set_loading.set(true);
                let file_type = file.type_();
                let reader = web_sys::FileReader::new().unwrap();
                let reader_c = reader.clone();

                let onload =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        let array_buffer = reader_c.result().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let bytes = uint8_array.to_vec();
                        let encoded = general_purpose::STANDARD.encode(&bytes);
                        set_base64.set(encoded.clone());
                        set_data_uri.set(format!("data:{};base64,{}", file_type, encoded));
                        set_loading.set(false);
                    })
                        as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload.forget();
            }
        }
    };

    view! {
        <div class="tool-container">
            <ToolHeader lang=lang title_en="Image Base64" title_zh="圖片 Base64"/>
            <div class="box">
                <div class="box-label">{move || match lang.get() { Lang::En => "Upload Image", Lang::Zh => "上傳圖片", }}</div>
                <input type="file" accept="image/*" on:change=on_file_change class="file-input"/>
                {move || if is_loading.get() { view! { <div class="loading">"..."</div> } } else { view! { <div></div> } }}
            </div>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-header"><div class="box-label">"Base64"</div><CopyButton text=base64_out/></div>
                    <textarea prop:value=base64_out readonly placeholder="..."></textarea>
                </div>
                <div class="box">
                    <div class="box-header"><div class="box-label">"Data URI"</div><CopyButton text=data_uri/></div>
                    <textarea prop:value=data_uri readonly placeholder="..."></textarea>
                </div>
            </div>
        </div>
    }
}
