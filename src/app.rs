use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::index::Index;
use crate::components::layout::header::Header;



#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Header />
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div  class="bg-slate-100 w-full bg-no-repeat bg-center">
        <div class="my-0 mx-auto text-center bg-grid-slate-300 flex justify-center flex-col items-center">
            <img src="./rhq3ezvso9611.png" width="400" class="mt-5" />
            <h2 class="p-6 text-4xl">"Bienvenidos a Rust Lang en Español"</h2>
            <p class="px-10 pb-10 text-left">"Una comunidad de gente mal intencionada y tonta."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
        </div>
    </div>
    }
}
