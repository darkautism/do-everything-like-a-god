use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Lang {
    En,
    Zh,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (lang, set_lang) = create_signal(Lang::Zh);
    let (is_sidebar_open, set_sidebar_open) = create_signal(false);

    view! {
        <Title text="工具箱 | Useful Tools"/>
        <Meta name="description" content="High-performance WASM developer tools: Base64, JSON Formatter, JWT Decoder, Regex Tester and more. Fast and private."/>
        
        <Router trailing_slash=TrailingSlash::Redirect>
            <div class="layout">
                // Mobile Header
                <div class="mobile-header">
                    <button class="menu-toggle" on:click=move |_| set_sidebar_open.update(|v| *v = !*v)>
                        "Menu"
                    </button>
                    <div class="mobile-brand">"UTILITIES"</div>
                </div>
                // Sidebar
                <nav class=move || if is_sidebar_open.get() { "sidebar open" } else { "sidebar" }>
                    <div class="sidebar-header">
                        <A href="" class="brand" on:click=move |_| set_sidebar_open.set(false)>"GOD MODE"</A>
                        <button class="lang-switch" on:click=move |_| {
                            set_lang.update(|l| *l = if *l == Lang::En { Lang::Zh } else { Lang::En });
                        }>
                            {move || match lang.get() {
                                Lang::En => "中文",
                                Lang::Zh => "English",
                            }}
                        </button>
                    </div>

                    <div class="category">
                        <div class="category-title">"Converters"</div>
                        <A href="base64" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base64"</A>
                        <A href="html-escape" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"HTML Escape"</A>
                        <A href="url-escape" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"URL Escape"</A>
                        <A href="base-conv" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Base Converter"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Development"</div>
                        <A href="json" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"JSON Tool"</A>
                        <A href="jwt" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"JWT Decoder"</A>
                        <A href="regex" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Regex Tester"</A>
                        <A href="diff" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Diff Checker"</A>
                    </div>

                    <div class="category">
                        <div class="category-title">"Security & Data"</div>
                        <A href="hash" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Hash Utility"</A>
                        <A href="uuid" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"UUID Gen"</A>
                        <A href="timestamp" class="nav-link" on:click=move |_| set_sidebar_open.set(false)>"Timestamp"</A>
                    </div>
                </nav>

                // Overlay for mobile
                <div class=move || if is_sidebar_open.get() { "overlay show" } else { "overlay" }
                     on:click=move |_| set_sidebar_open.set(false)></div>

                <main class="main-content">
                    <Routes>
                        <Route path="" view=move || view! { <HomePage lang=lang /> }/>
                        <Route path="base64" view=move || view! { <Base64Page lang=lang /> }/>
                        <Route path="html-escape" view=move || view! { <HtmlEscapePage lang=lang /> }/>
                        <Route path="url-escape" view=move || view! { <UrlEscapePage lang=lang /> }/>
                        <Route path="json" view=move || view! { <JsonPage lang=lang /> }/>
                        <Route path="hash" view=move || view! { <HashPage lang=lang /> }/>
                        <Route path="jwt" view=move || view! { <JwtPage lang=lang /> }/>
                        <Route path="uuid" view=move || view! { <UuidPage lang=lang /> }/>
                        <Route path="regex" view=move || view! { <RegexPage lang=lang /> }/>
                        <Route path="timestamp" view=move || view! { <TimestampPage lang=lang /> }/>
                        <Route path="base-conv" view=move || view! { <BaseConvPage lang=lang /> }/>
                        <Route path="diff" view=move || view! { <DiffPage lang=lang /> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

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
            <Title text="Diff Checker - 工具箱"/>
            <Meta name="description" content="High-performance text comparison tool powered by Rust WASM. Fast diffing for large documents."/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "Text Diff",
                    Lang::Zh => "文本比對",
                }}
            </h2>
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
            <div class="box" style="margin-top:20px; font-family:monospace; background:#111; overflow-x:auto; white-space:pre-wrap">
                {diff_view}
            </div>
        </div>
    }
}

#[component]
fn BaseConvPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let _ = lang;
    let (val, set_val) = create_signal(String::new());
    let (from_base, set_from) = create_signal(10u32);

    let get_val = move |base: u32| {
        let current = val.get();
        if current.is_empty() { return String::new(); }
        u128::from_str_radix(&current, from_base.get())
            .map(|n| {
                match base {
                    2 => format!("{:b}", n),
                    8 => format!("{:o}", n),
                    10 => format!("{}", n),
                    16 => format!("{:x}", n),
                    _ => String::new(),
                }
            }).unwrap_or_else(|_| "Error".to_string())
    };

    view! {
        <div class="tool-container">
            <Title text="Number Base Converter - 工具箱"/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">"Base Converter"</h2>
            <div class="box">
                <div class="box-label">"Source Value"</div>
                <div style="display:flex; gap:10px">
                    <input type="text" on:input=move |ev| set_val.set(event_target_value(&ev)) style="flex:1; padding:10px; background:#111; color:#fff; border:1px solid #333" />
                    <select on:change=move |ev| set_from.set(event_target_value(&ev).parse().unwrap()) style="background:#111; color:#fff; border:1px solid #333">
                        <option value="10">"Dec"</option>
                        <option value="16">"Hex"</option>
                        <option value="2">"Bin"</option>
                        <option value="8">"Oct"</option>
                    </select>
                </div>
            </div>
            <div class="tool-grid">
                <div class="box"><div class="box-label">"Decimal"</div><div style="color:#0f0">{move || get_val(10)}</div></div>
                <div class="box"><div class="box-label">"Hex"</div><div style="color:#0f0">{move || get_val(16)}</div></div>
                <div class="box"><div class="box-label">"Binary"</div><div style="color:#0f0; font-size:0.8rem; word-break:break-all">{move || get_val(2)}</div></div>
                <div class="box"><div class="box-label">"Octal"</div><div style="color:#0f0">{move || get_val(8)}</div></div>
            </div>
        </div>
    }
}

#[component]
fn TimestampPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let _ = lang;
    use chrono::{Utc, TimeZone};

    let (ts, set_ts) = create_signal(Utc::now().timestamp().to_string());
    
    let date_str = move || {
        let s = ts.get();
        s.parse::<i64>().ok()
            .and_then(|val| Utc.timestamp_opt(val, 0).single())
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "Invalid Timestamp".to_string())
    };

    view! {
        <div class="tool-container">
            <Title text="Unix Timestamp Converter - 工具箱"/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">"Timestamp"</h2>
            <div class="box">
                <div class="box-label">"Unix Timestamp"</div>
                <input type="text" prop:value=ts on:input=move |ev| set_ts.set(event_target_value(&ev)) style="width:100%; padding:10px; background:#111; color:#fff; border:1px solid #333" />
                <button class="btn" on:click=move |_| set_ts.set(Utc::now().timestamp().to_string()) style="margin-top:10px">"Current Time"</button>
            </div>
            <div class="box">
                <div class="box-label">"UTC ISO 8601"</div>
                <div style="font-size:1.5rem; color:#0f0">{date_str}</div>
            </div>
        </div>
    }
}

#[component]
fn RegexPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use regex::Regex;

    let (pattern, set_pattern) = create_signal(String::new());
    let (text, set_text) = create_signal(String::new());
    let (is_match, set_is_match) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);

    let test_regex = move |_| {
        set_error.set(None);
        match Regex::new(&pattern.get()) {
            Ok(re) => {
                set_is_match.set(re.is_match(&text.get()));
            }
            Err(e) => set_error.set(Some(e.to_string())),
        }
    };

    view! {
        <div class="tool-container">
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "Regex Tester",
                    Lang::Zh => "正則表達式測試",
                }}
            </h2>
            <div class="box">
                <div class="box-label">"Regex Pattern"</div>
                <input 
                    type="text" 
                    prop:value=pattern
                    on:input=move |ev| { set_pattern.set(event_target_value(&ev)); test_regex(()); }
                    placeholder="^[a-z]+$"
                    style="width:100%; padding:10px; background:#111; color:#fff; border:1px solid #333"
                />
                {move || error.get().map(|e| view! { <div style="color:#f00; margin-top:5px">{e}</div> })}
            </div>
            <div class="box">
                <div class="box-label">"Test Text"</div>
                <textarea 
                    prop:value=text
                    on:input=move |ev| { set_text.set(event_target_value(&ev)); test_regex(()); }
                    placeholder="text to test..."
                ></textarea>
                <div style=format!("margin-top:10px; font-size:1.5rem; font-weight:bold; color:{}", if is_match.get() { "#0f0" } else { "#f00" })>
                    {move || if is_match.get() { "MATCH" } else { "NO MATCH" }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn UuidPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use uuid::Uuid;

    let (uuid_v4, set_uuid_v4) = create_signal(String::new());

    let generate = move |_| {
        set_uuid_v4.set(Uuid::new_v4().to_string());
    };

    view! {
        <div class="tool-container">
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "UUID Generator",
                    Lang::Zh => "UUID 生成器",
                }}
            </h2>
            <div class="box" style="text-align:center">
                <div class="box-label">"UUID v4"</div>
                <div style="font-size:2rem; font-family:monospace; margin:20px 0; color:#0f0">
                    {move || {
                        let current = uuid_v4.get();
                        if current.is_empty() { "---------点击生成---------".to_string() } else { current }
                    }}
                </div>
                <button class="btn" on:click=generate>"Generate New UUID"</button>
            </div>
        </div>
    }
}

#[component]
fn JwtPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{Engine as _, engine::general_purpose};

    let (input, set_input) = create_signal(String::new());
    let (header, set_header) = create_signal(String::new());
    let (payload, set_payload) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let decode = move |val: String| {
        set_input.set(val.clone());
        set_error.set(None);
        set_header.set(String::new());
        set_payload.set(String::new());

        let parts: Vec<&str> = val.split('.').collect();
        if parts.len() < 2 {
            if !val.is_empty() {
                set_error.set(Some("Invalid JWT format (must have at least 2 parts)".to_string()));
            }
            return;
        }

        let decode_part = |part: &str| -> Result<String, String> {
            let bytes = general_purpose::URL_SAFE_NO_PAD.decode(part)
                .map_err(|e| format!("Base64 Decode Error: {}", e))?;
            let json_str = String::from_utf8(bytes)
                .map_err(|e| format!("UTF-8 Error: {}", e))?;
            let val: serde_json::Value = serde_json::from_str(&json_str)
                .map_err(|e| format!("JSON Error: {}", e))?;
            Ok(serde_json::to_string_pretty(&val).unwrap())
        };

        match decode_part(parts[0]) {
            Ok(h) => set_header.set(h),
            Err(e) => { set_error.set(Some(format!("Header: {}", e))); return; }
        }

        match decode_part(parts[1]) {
            Ok(p) => set_payload.set(p),
            Err(e) => { set_error.set(Some(format!("Payload: {}", e))); }
        }
    };

    view! {
        <div class="tool-container">
            <Title text="JWT Decoder - 工具箱"/>
            <Meta name="description" content="Decode JSON Web Tokens (JWT) Header and Payload instantly. No data leaves your browser. Fast WASM implementation."/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "JWT Decoder",
                    Lang::Zh => "JWT 解碼",
                }}
            </h2>
            <div class="box">
                <div class="box-label">"JWT Token"</div>
                <textarea 
                    prop:value=input
                    on:input=move |ev| decode(event_target_value(&ev))
                    placeholder="eyJhbGci..."
                ></textarea>
                {move || error.get().map(|e| view! { <div style="color:#f00; margin-top:5px">{e}</div> })}
            </div>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">"Header"</div>
                    <textarea prop:value=header readonly placeholder="..."></textarea>
                </div>
                <div class="box">
                    <div class="box-label">"Payload"</div>
                    <textarea prop:value=payload readonly placeholder="..."></textarea>
                </div>
            </div>
        </div>
    }
}

#[component]
fn JsonPage(lang: ReadSignal<Lang>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let process = move |minify: bool| {
        set_error.set(None);
        let val: Result<serde_json::Value, _> = serde_json::from_str(&input.get());
        match val {
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
            <Title text="JSON Formatter - 工具箱"/>
            <Meta name="description" content="Fastest WASM-powered JSON Prettifier and Minifier. Professional developer tool for formatting large JSON data instantly."/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "JSON Formatter",
                    Lang::Zh => "JSON 格式化",
                }}
            </h2>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">"Input"</div>
                    <textarea 
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        placeholder=r#"{"key":"value"}"#
                    ></textarea>
                    <div style="display:flex; gap:10px; margin-top:10px">
                        <button class="btn" on:click=move |_| process(false)>"Prettify"</button>
                        <button class="btn" on:click=move |_| process(true)>"Minify"</button>
                    </div>
                </div>
                <div class="box">
                    <div class="box-label">"Result"</div>
                    <textarea 
                        prop:value=output
                        readonly
                        placeholder="..."
                    ></textarea>
                    {move || error.get().map(|e| view! { <div style="color:#f00; margin-top:5px">{e}</div> })}
                </div>
            </div>
        </div>
    }
}

#[component]
fn HashPage(lang: ReadSignal<Lang>) -> impl IntoView {
    use sha2::{Sha256, Digest};
    use sha1::Sha1;
    use md5::Md5;
    use wasm_bindgen::JsCast;

    let (input, set_input) = create_signal(String::new());
    let (md5_res, set_md5) = create_signal(String::new());
    let (sha1_res, set_sha1) = create_signal(String::new());
    let (sha256_res, set_sha256) = create_signal(String::new());
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
    };

    let on_text_input = move |val: String| {
        set_input.set(val.clone());
        if val.is_empty() {
            set_md5.set(String::new());
            set_sha1.set(String::new());
            set_sha256.set(String::new());
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
                
                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    let array_buffer = reader_c.result().unwrap();
                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                    let bytes = uint8_array.to_vec();
                    compute_hashes(&bytes);
                    set_loading.set(false);
                }) as Box<dyn FnMut(_)>);
                
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload.forget();
            }
        }
    };

    view! {
        <div class="tool-container">
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "Hash Utility",
                    Lang::Zh => "Hash 工具",
                }}
            </h2>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">"Input Text"</div>
                    <textarea 
                        prop:value=input
                        on:input=move |ev| on_text_input(event_target_value(&ev))
                        placeholder="Enter text to hash..."
                    ></textarea>
                </div>
                <div class="box">
                    <div class="box-label">"Or Upload File"</div>
                    <input 
                        type="file" 
                        on:change=on_file_change 
                        style="width:100%; padding:20px; background:#111; border:2px dashed #333; color:#fff; border-radius:12px"
                    />
                    {move || if is_loading.get() { view! { <div style="color:#38bdf8; margin-top:10px">"Processing large file..."</div> } } else { view! { <div></div> } }}
                </div>
            </div>
            
            <div style="margin-top:40px; display:flex; flex-direction:column; gap:20px">
                <div class="box">
                    <div class="box-label">"MD5"</div>
                    <input type="text" prop:value=md5_res readonly style="width:100%; background:#111; color:#0f0; border:1px solid #333; padding:10px; font-family:monospace" />
                </div>
                <div class="box">
                    <div class="box-label">"SHA1"</div>
                    <input type="text" prop:value=sha1_res readonly style="width:100%; background:#111; color:#0f0; border:1px solid #333; padding:10px; font-family:monospace" />
                </div>
                <div class="box">
                    <div class="box-label">"SHA256"</div>
                    <input type="text" prop:value=sha256_res readonly style="width:100%; background:#111; color:#0f0; border:1px solid #333; padding:10px; font-family:monospace" />
                </div>
            </div>
        </div>
    }
}

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
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "URL Utility",
                    Lang::Zh => "URL 工具",
                }}
            </h2>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">"Raw Text"</div>
                    <textarea 
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        placeholder="https://example.com/測試"
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=encode>
                        {move || match lang.get() {
                            Lang::En => "Encode ➔",
                            Lang::Zh => "編碼 ➔",
                        }}
                    </button>
                </div>
                <div class="box">
                    <div class="box-label">"Encoded"</div>
                    <textarea 
                        prop:value=output
                        on:input=move |ev| set_output.set(event_target_value(&ev))
                        placeholder="https%3A%2F%2Fexample.com%2F%E6%B8%AC%E8%A9%A6"
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=decode>
                        {move || match lang.get() {
                            Lang::En => "⬅ Decode",
                            Lang::Zh => "⬅ 解碼",
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

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
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "HTML Escape Utility",
                    Lang::Zh => "HTML 轉義工具",
                }}
            </h2>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">
                        {move || match lang.get() {
                            Lang::En => "Unescaped / Raw",
                            Lang::Zh => "原始 HTML",
                        }}
                    </div>
                    <textarea 
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        placeholder="<div>...</div>"
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=escape>
                        {move || match lang.get() {
                            Lang::En => "Escape ➔",
                            Lang::Zh => "轉義 ➔",
                        }}
                    </button>
                </div>
                <div class="box">
                    <div class="box-label">
                        {move || match lang.get() {
                            Lang::En => "Escaped Entities",
                            Lang::Zh => "轉義結果",
                        }}
                    </div>
                    <textarea 
                        prop:value=output
                        on:input=move |ev| set_output.set(event_target_value(&ev))
                        placeholder="&lt;div&gt;...&lt;/div&gt;"
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=unescape>
                        {move || match lang.get() {
                            Lang::En => "⬅ Unescape",
                            Lang::Zh => "⬅ 還原",
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_base64_logic() {
        let input = "Hello 🚀";
        let encoded = general_purpose::STANDARD.encode(input.as_bytes());
        assert_eq!(encoded, "SGVsbG8g8J+agA==");
        
        let decoded_bytes = general_purpose::STANDARD.decode(encoded).unwrap();
        let decoded_str = String::from_utf8(decoded_bytes).unwrap();
        assert_eq!(decoded_str, input);
    }

    #[test]
    fn test_html_escape_logic() {
        let input = "<script>alert('god')</script>";
        let escaped = html_escape::encode_safe(input).to_string();
        // html-escape encodes quotes and slashes for safety
        assert_eq!(escaped, "&lt;script&gt;alert(&#x27;god&#x27;)&lt;&#x2F;script&gt;");
        
        let unescaped = html_escape::decode_html_entities(&escaped).to_string();
        assert_eq!(unescaped, input);
    }
}

#[component]
fn Base64Page(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{Engine as _, engine::general_purpose};

    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());

    let encode = move |_| {
        let encoded = general_purpose::STANDARD.encode(input.get().as_bytes());
        set_output.set(encoded);
    };

    let decode = move |_| {
        if let Ok(bytes) = general_purpose::STANDARD.decode(output.get().trim()) {
            if let Ok(s) = String::from_utf8(bytes) {
                set_input.set(s);
            }
        }
    };

    view! {
        <div class="tool-container">
            <Title text="Base64 Decoder/Encoder - 工具箱"/>
            <Meta name="description" content="Encode and decode text to Base64 format with high performance WASM logic. Secure and local processing."/>
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "Base64 Utility",
                    Lang::Zh => "Base64 工具",
                }}
            </h2>
            <div class="tool-grid">
                <div class="box">
                    <div class="box-label">
                        {move || match lang.get() {
                            Lang::En => "Text / UTF-8",
                            Lang::Zh => "原始文字 / UTF-8",
                        }}
                    </div>
                    <textarea 
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=encode>
                        {move || match lang.get() {
                            Lang::En => "Encode ➔",
                            Lang::Zh => "編碼 ➔",
                        }}
                    </button>
                </div>
                <div class="box">
                    <div class="box-label">
                        {move || match lang.get() {
                            Lang::En => "Base64 Result",
                            Lang::Zh => "Base64 結果",
                        }}
                    </div>
                    <textarea 
                        prop:value=output
                        on:input=move |ev| set_output.set(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                    <button class="btn" style="margin-top:10px; width:100%" on:click=decode>
                        {move || match lang.get() {
                            Lang::En => "⬅ Decode",
                            Lang::Zh => "⬅ 解碼",
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn HomePage(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <div class="hero">
            <Title text="WASM 工具箱 | Do Everything Like a God"/>
            <Meta name="description" content="All-in-one developer toolbox powered by Rust WASM. Fast, secure, and SEO-friendly tools for daily development."/>
            <h1>
                {move || match lang.get() {
                    Lang::En => "Do Everything Like a God",
                    Lang::Zh => "做甚麼都有如神助",
                }}
            </h1>
            <p>
                {move || match lang.get() {
                    Lang::En => "Empowering your workflow with divine efficiency. Simple, clean, and ridiculously fast.",
                    Lang::Zh => "賦予你的工作流神一般的效率。簡單、乾淨、快得不可思議。",
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
