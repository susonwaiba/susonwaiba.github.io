use crate::api;
use crate::model::note::NoteItem;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotesPage(cx: Scope) -> impl IntoView {
    let note_resource = create_resource(
        cx,
        move || (),
        move |()| async move { api::fetch_api::<Vec<NoteItem>>(cx, "/assets/exports/notes.json").await },
    );

    view! { cx,
        <Title text="Notes - Suson Waiba"/>
        <div class="app-container text-center">
            <div class="my-12">
                <h1>"Notes"</h1>
            </div>
        </div>
        <div class="app-container mb-8">
            <Suspense fallback=move || view! { cx, <p class="text-center">"Loading..."</p> }>
                {move || match note_resource.read(cx) {
                    None => None,
                    Some(None) => Some(view! { cx,  <p class="text-center">"Error loading notes."</p> }.into_any()),
                    Some(Some(note_list)) => {
                        Some(view! { cx,
                            <div class="app-note-grid">
                                <For
                                    each=move || note_list.clone()
                                    key=|note| note.id.clone()
                                    view=move |cx, note: NoteItem| {
                                        let note_url = "/notes/".to_string() + &note.path;
                                        view! { cx,
                                            <div>
                                                <a class="app-note-card-link" href={note_url}>
                                                    <div class="app-note-card">
                                                        <h4>{note.title}</h4>
                                                        <p><small>"Last updated: " {note.date}</small></p>
                                                    </div>
                                                </a>
                                            </div>

                                        }
                                    }
                                />
                            </div>
                        }.into_any())
                    }
                }}
            </Suspense>
            <div class="text-center py-6">
                <p>"More updates coming soon..."</p>
            </div>
        </div>
    }
}
