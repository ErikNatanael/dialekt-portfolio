use leptos::*;
use leptos_router::*;
mod bio;
use bio::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <p>"Hello, world!"</p> <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main
            class="container mx-auto"
            >
            <div
             class="bg-slate-900 flex flex-col md:flex-row h-screen margin-0 overflow-hidden text-white font-normal"
             >
          // all our routes will appear inside <main>
          <Routes>
            /* ... */
    <Route path="/" view=|cx| view! { cx, <Home/> }/>
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
            <BioMasonry/>
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
    view! { cx,
            <div
             class="flex flex-col"
             >
            <h1
             class="text-3xl"
             >"Erik Natanael Gustafsson"</h1>
            <p>"is a musician, composer, coder, visual artist and instrument builder"</p>
            <p>"... distilled artist statement ..."</p>
                <A href="commissions">"Commissions"</A>
                <A href="contact">"Contact"</A>
                <A href="cv">"CV"</A>
            </div>
    }
}
