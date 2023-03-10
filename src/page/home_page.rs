use crate::api;
use crate::model::note::NoteItem;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let recent_notes_resource = create_resource(
        cx,
        move || (),
        move |()| async move {
            api::fetch_api::<Vec<NoteItem>>(cx, "/assets/exports/recent_notes.json").await
        },
    );

    view! { cx,
        <div class="app-container hero-greeting text-center py-12">
            <img src="/assets/logo.svg" width="96px" alt="Logo"/>
            <h1 class="hero-greeting-text">"Hi, I am "<span class="color-primary-text">"Suson Waiba"</span>"."</h1>
            <p class="hero-greeting-text-secondary">"Software developer writing quality software solutions"</p>
            <div>
                <a class="app-btn m-2" href="https://github.com/susonwaiba" target="_blank">"Github"</a>
                <a class="app-btn m-2" href="https://www.linkedin.com/in/susonwaiba" target="_blank">"LinkedIn"</a>
                <a class="app-btn m-2" href="https://twitter.com/susonwaiba" target="_blank">"Twitter"</a>
            </div>
            <p class="color-alt-text">"I am usually occupied but open to hire, hit me on Linkedin for chat"</p>
        </div>
        <div class="color-primary-light-bg p-4 mb-8 text-center">
            <h4 class="text-8">"Recent notes"</h4>
        </div>
        <div class="app-container mb-6">
            <Suspense fallback=move || view! { cx, <p class="text-center">"Loading..."</p> }>
                {move || match recent_notes_resource.read(cx) {
                    None => None,
                    Some(None) => Some(view! { cx, <p>"Error loading notes."</p> }.into_any()),
                    Some(Some(note_list)) => {
                        Some(view! { cx,
                            <div class="app-note-grid mb-6">
                                <For
                                    each=move || note_list.clone()
                                    key=|note| note.id.clone()
                                    view=move |cx, note: NoteItem| {
                                        let note_url = "/notes/".to_string() + &note.path;
                                        view! { cx,
                                            <a class="app-note-card-link" href={note_url}>
                                                <div class="app-note-card">
                                                    <h4>{note.title}</h4>
                                                    <p><small>"Last updated: " {note.date}</small></p>
                                                </div>
                                            </a>
                                        }
                                    }
                                />
                            </div>
                    }.into_any())
                }
            }}
            </Suspense>
            <div class="text-center">
                <a class="app-btn" href="/notes">"View notes"</a>
            </div>
        </div>
    }
}
