use leptos::*;
use leptos_router::*;
mod bio;
use bio::*;
mod project;
use project::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main
            class="mx-auto"
            >
            <div
             class="bg-slate-900 flex flex-col md:flex-row h-screen margin-0 overflow-hidden text-white font-normal"
             >
          // all our routes will appear inside <main>
          <Routes>
            /* ... */
    <Route path="/" view=|cx| view! { cx, <Home/> }/>
    <Route path="/bio" view=|cx| view! { cx, <BioPage/> }>
        <Route path="musician" view=|cx| view! { cx, <BioMusician/> }/>
        <Route path="" view=|cx| view! { cx,
            <p>"Select a bio section to view more info."</p>
          }/>
    </Route>
    <Route path="/projects" view=|cx| view! { cx, <Projects/> }/>
    // <Route path="/users/:id" view=|cx| view! { cx, <UserProfile/> }/>
    <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
          </Routes>
            </div>
        </main>
      </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
                <h1
                    class="text-red-600 text-3xl"
                >
                    "Home"</h1>
            <div
             class=("basis-1/3")
             >
            <HomeBio/>
            </div>
            <div
             class=("basis-2/3")
            >
            <ProjectMasonry/>
            </div>

    }
}
#[component]
fn Projects(cx: Scope) -> impl IntoView {
    view! { cx,
            <h1>"Projects"</h1>
    }
}
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
            <h1>"404 NotFound"</h1>
    }
}

#[component]
fn HomeBio(cx: Scope) -> impl IntoView {
    let bio_links = "hover:underline text-zinc-300 font-bold";
    view! { cx,
            <div
             class="flex flex-col h-full place-content-evenly"
             >
            <h1
             class="text-3xl"
             >"Erik Natanael Gustafsson"</h1>
            <p>"is a "<A class=bio_links href="musician">"musician"</A>", "<A class=bio_links href="composer">"composer"</A>", "<A class=bio_links href="coder">"coder"</A>", "<A class=bio_links href="visual_artist">"visual artist"</A>" and "<A class=bio_links href="instrument_builder">"instrument builder"</A></p>
            <p>"Through bespoke software and devices, he creates ephemeral and elaborate moments and worlds: musically abstract visuals, fragmented fine grained sounds, digital materiality. Engaging with sublimity and fragility, wonder and care, his work brings out the tiny and invisible inside vast architectural systems."</p>
                <A class=bio_links href="commissions">"Commissions"</A>
                <A class=bio_links href="contact">"Contact"</A>
                <A class=bio_links href="cv">"CV"</A>
            </div>
    }
}

#[component]
fn BioPage(cx: Scope) -> impl IntoView {
    view! { cx,
            <h1
                class="text-red-600 text-3xl"
            >
                "BioPage"</h1>
        <div
         class=("basis-1/3")
         >
        <HomeBio/>
        </div>
        <div
         class=("basis-2/3")
        >
        <Outlet/>
        </div>

    }
}

#[component]
fn BioMusician(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Bio musician"</h1>
    }
}