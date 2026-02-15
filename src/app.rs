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
                <div class="brand">"GOD MODE"</div>
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
                </Routes>
            </main>
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
