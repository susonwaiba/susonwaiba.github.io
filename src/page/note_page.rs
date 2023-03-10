use crate::api;
use crate::model::note::Note;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn NotePage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let path = params().get("path").cloned().unwrap_or_default();
    let note_resource = create_resource(
        cx,
        move || path.clone(),
        move |path| async move {
            let api_path = format!("/assets/exports/notes/{}.json", path);
            api::fetch_api::<Note>(cx, &api_path).await
        },
    );

    view! {cx,
        <Stylesheet id="font_fira_code" href="https://fonts.googleapis.com/css2?family=Fira+Code&display=swap"/>
        <Stylesheet id="highlight_css" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css"/>
        <Stylesheet id="highlight_css_dark" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/dark.min.css"/>
        <Script id="hightlight_js" src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"/>
        <div>
            <Suspense fallback=move || view! { cx, <p class="text-center">"Loading..."</p> }>
                {move || match note_resource.read(cx) {
                    None => None,
                    Some(None) => Some(view! { cx,
                            <div class="app-container text-center">
                                <div class="my-12">
                                    <h1>"Note not found."</h1>
                                </div>
                            </div>
                        }.into_any()),
                    Some(Some(note)) => {
                        Some(view! { cx,
                            <div class="app-container mb-8">
                                <Script id="hightlight_js_init">
                                    "document.addEventListener('DOMContentLoaded', (event) => { document.querySelectorAll('pre code').forEach((el) => { console.log(el); hljs.highlightElement(el); }); });"
                                </Script>
                                <Title text={format!("{} - Suson Waiba", note.title.clone().to_string())}/>
                                <div class="my-12 text-center">
                                    <h1>{note.title}</h1>
                                </div>
                                <div>
                                    <div class="app-md-content" inner_html={note.content} />
                                </div>
                                <div>
                                    <small>"Last updated: " {note.date}</small>
                                </div>
                            </div>
                        }.into_any())
                }
            }}
            </Suspense>
        </div>
        <Script id="hightlight_js_init">
            "setTimeout(function () { document.querySelectorAll('pre code').forEach((el) => { hljs.highlightElement(el); }); }, 1000);"
        </Script>
    }
}
