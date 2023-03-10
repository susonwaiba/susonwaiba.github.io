use crate::api;
use crate::model::about::About;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    let about_resource = create_resource(
        cx,
        move || (),
        move |()| async move { api::fetch_api::<Vec<About>>(cx, "/assets/exports/about.json").await },
    );

    view! { cx,
        <Title text="About - Suson Waiba"/>
        <div class="app-container">
            <div class="profile-widget">
                <div class="app-row row-align-items-center row-justify-content-center my-12">
                    <div class="p-2">
                        <img class="profile-img border-radius-circle" src="/assets/profile.jpeg" alt="Profile" />
                    </div>
                    <div class="p-2">
                        <h1 class="profile-label m-0">"Suson Waiba"</h1>
                        <div>
                            <span class="ml-1" style="position: absolute; margin-top: 4px;">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="profile-icon-svg">
                                    <path d="M64 112c-8.8 0-16 7.2-16 16v22.1L220.5 291.7c20.7 17 50.4 17 71.1 0L464 150.1V128c0-8.8-7.2-16-16-16H64zM48 212.2V384c0 8.8 7.2 16 16 16H448c8.8 0 16-7.2 16-16V212.2L322 328.8c-38.4 31.5-93.7 31.5-132 0L48 212.2zM0 128C0 92.7 28.7 64 64 64H448c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128z" />
                                </svg>
                            </span>
                            <span class="ml-7">"susonwaiba@gmail.com"</span>
                        </div>
                        <div class="text-3">
                            <a class="app-btn-link" href="https://github.com/susonwaiba" target="_blank">"Github"</a>
                            <span>"·"</span>
                            <a class="app-btn-link" href="https://www.linkedin.com/in/susonwaiba" target="_blank">"LinkedIn"</a>
                            <span>"·"</span>
                            <a class="app-btn-link" href="https://twitter.com/susonwaiba" target="_blank">"Twitter"</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="color-primary-light-bg p-4 mb-6 text-center">
            <h4 class="text-8">"My software development exploration timeline"</h4>
        </div>
            <Suspense fallback=move || view! { cx, <p class="text-center">"Loading..."</p> }>
                {move || match about_resource.read(cx) {
                    None => None,
                    Some(None) => Some(view! { cx, <p class="text-center">"Error loading about."</p> }.into_any()),
                    Some(Some(about_list)) => {
                        Some(view! { cx,
                            <div class="app-container mb-6">
                                <For
                                    each=move || about_list.clone()
                                    key=|about| about.year.clone()
                                    view=move |cx, about: About| {
                                        view! { cx,
                                            <div class="app-row">
                                                <div class="col-shrink-1 p-4" style="line-height: 1.7;">
                                                    <div class="color-primary-light-bg pt-1 pb-1 pl-2 pr-2 border-radius-1">{about.year}</div>
                                                </div>
                                                <div class="col-grow-1">
                                                    <h3>{about.title}</h3>
                                                    <div class="app-md-content" inner_html={about.content} />
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                    }.into_any())
                }
            }}
            </Suspense>


    }
}
