use leptos::*;
// use leptos_router::*;
mod portfolio;
use portfolio::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    //   <Router>
    //     <nav>
    //       /* ... */
    //     </nav>
        <main
            class="mx-auto"
            >
            <div
             class="bg-emerald-1000 flex flex-col md:flex-row margin-0 text-white font-normal font-urbanist"
             >
          // all our routes will appear inside <main>
    //       <Routes>
    //         /* ... */
    // <Route path="/" view=|cx| view! { cx, <Home/> }/>

    // <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
    //       </Routes>
            </div>
        </main>
    //   </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
                // <h1
                //     class="text-red-600 text-3xl"
                // >
                //     "Home"</h1>
        <div class="w-full flex flex-col items-center">
            <Header />
            <Portfolio />
            <div>
            <p class="text-center">"Please reach me at erik@eriknatanael.com for any questions."</p>
            </div>
        </div>

    }
}
#[component]
fn Portfolio(cx: Scope) -> impl IntoView {
    let items: Vec<_> = all_portfolio_items()
        .into_iter()
        .map(|i| i.view(cx))
        .collect();
    let particle_images: Vec<_> = graphics_images().into_iter().map(|i| i.view(cx)).collect();
    let me_images: Vec<_> = me_images().into_iter().map(|i| i.view(cx)).collect();
    let image_container_c = "columns-2 gap-8 mx-4 lg:mx-0";
    view! { cx,
        <div class=("w-full lg:w-2/3")>
            <h1 class="text-5xl text-center">"Video/Audio"</h1>
            {items}
            <h1 class="text-5xl text-center mb-6">"Images"</h1>
            <p class="text-italic text-thin text-center">"Output from the graphics engine I am currently using"</p>
            <div class=image_container_c>

            {particle_images}
            </div>
            <p class="text-italic text-thin text-center">"My instruments and me"</p>
            <div class=image_container_c>
            {me_images}
            </div>
        </div>
    }
}
#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <div
        class="flex flex-col lg:flex-row w-full m-6 p-3"
        >
            <div class="basis-1/2 text-right px-2">
                <h1
             class="text-5xl font-thin"
             >"Erik Natanael"</h1>
                <h1 class="text-3xl font-thin">"Gustafsson"</h1>
            </div>
            <div class="px-3">
                <h1 class="text-3xl font-thin">"Portfolio for the Nordic residency exchange open call Dialekt"</h1>
            </div>
        </div>
    }
}
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
            <h1>"404 NotFound"</h1>
    }
}
