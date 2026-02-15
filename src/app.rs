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

    view! {
        <Stylesheet id="leptos" href="/pkg/do_everything_like_a_god.css"/>
        <Title text="做甚麼都有如神助 | Do Everything Like a God"/>
        
        <Router>
            <nav class="nav">
                <div style="display:flex;gap:20px;align-items:center">
                    <A href="/" class="brand" style="text-decoration:none">"GOD MODE"</A>
                    <A href="/base64" class="nav-link">"Base64"</A>
                </div>
                <button class="lang-switch" on:click=move |_| {
                    set_lang.update(|l| *l = if *l == Lang::En { Lang::Zh } else { Lang::En });
                }>
                    {move || match lang.get() {
                        Lang::En => "中文",
                        Lang::Zh => "English",
                    }}
                </button>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=move || view! { <HomePage lang=lang /> }/>
                    <Route path="/base64" view=move || view! { <Base64Page lang=lang /> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Base64Page(lang: ReadSignal<Lang>) -> impl IntoView {
    use base64::{Engine as _, engine::general_purpose};

    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());

    let encode = move |val: String| {
        let encoded = general_purpose::STANDARD.encode(val.as_bytes());
        set_input.set(val);
        set_output.set(encoded);
    };

    let decode = move |val: String| {
        set_output.set(val.clone());
        if let Ok(bytes) = general_purpose::STANDARD.decode(val.trim()) {
            if let Ok(s) = String::from_utf8(bytes) {
                set_input.set(s);
            }
        }
    };

    view! {
        <div class="tool-container">
            <h2 style="font-size:3rem;font-weight:900;margin:0">
                {move || match lang.get() {
                    Lang::En => "Base64 Converter",
                    Lang::Zh => "Base64 編解碼",
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
                        on:input=move |ev| encode(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                </div>
                <div class="box">
                    <div class="box-label">
                        {move || match lang.get() {
                            Lang::En => "Base64",
                            Lang::Zh => "Base64 結果",
                        }}
                    </div>
                    <textarea 
                        prop:value=output
                        on:input=move |ev| decode(event_target_value(&ev))
                        placeholder="..."
                    ></textarea>
                </div>
            </div>
        </div>
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
