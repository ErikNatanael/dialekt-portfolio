use leptos::*;
use leptos_router::*;
mod portfolio;
use portfolio::*;

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
             class="bg-emerald-1000 flex flex-col md:flex-row margin-0 text-white font-normal font-urbanist"
             >
          // all our routes will appear inside <main>
          <Routes>
            /* ... */
    <Route path="/" view=|cx| view! { cx, <Home/> }/>

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

#[component]
fn HomeBio(cx: Scope) -> impl IntoView {
    let bio_links = "hover:underline text-zinc-300 font-bold text-xl underline";
    view! { cx,
            <div
             class="flex flex-col h-full place-content-evenly font-urbanist m-8"
             >
            <div>
            <h1
             class="text-9xl font-thin"
             >"Erik Natanael"</h1>
            <h1 class="text-5xl font-thin">"Gustafsson"</h1>
            </div>
            <p>"is a "<A class=bio_links href="/bio/musician">"musician"</A>", "<A class=bio_links href="/bio/composer">"composer"</A>", "<A class=bio_links href="/bio/coder">"coder"</A>", "<A class=bio_links href="/bio/visual_artist">"visual artist"</A>" and "<A class=bio_links href="/bio/digital_luthier">"digital luthier"</A></p>
            <p>"Through bespoke software and devices, Erik creates ephemeral and elaborate moments and worlds: musically abstract visuals, fragmented fine grained sounds, digital materiality. Engaging with sublimity and fragility, wonder and care, his work brings out the tiny and invisible inside vast architectural systems."</p>
                <A class=bio_links href="commissions">"Commissions"</A>
                <A class=bio_links href="contact">"Contact"</A>
                <A class=bio_links href="cv">"CV"</A>
            </div>
    }
}

#[component]
fn BioPage(cx: Scope) -> impl IntoView {
    view! { cx,
            // <h1
            //     class="text-red-600 text-3xl"
            // >
            //     "BioPage"</h1>
        <div
         class=("basis-1/3 flex flex-col")
         >
            <MainNav/>
        <HomeBio/>
        </div>
        <div
         class=("basis-2/3 h-full")
        >
            <div class="grid place-content-center font-urbanist h-full">
            <div class="text-right m-16 max-w-prose">
        <Outlet/>
            </div>
            </div>
        </div>

    }
}

#[component]
fn MainNav(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="grid grid-flow-col auto-cols-max basis-0 mx-8 mt-4 place-content-center">
            <MainNavLink text="Home" url="/"/>
            <MainNavLink text="Projects" url="/projects"/>
            <MainNavLink text="Writing" url="/writing"/>
            <MainNavLink text="Shop" url="/shop"/>
            <MainNavLink text="About" url="/bio"/>
            </div>
    }
}

#[component]
fn MainNavLink(cx: Scope, text: &'static str, url: &'static str) -> impl IntoView {
    view! { cx,
            <A href=url
            class="flex-1 w-full border-white rounded-lg text-sm text-center  flex flex-row "
             >
            // <div >
            <img class="h-full mr-2" src="/assets/images/menu_icon.svg"/>
            <span class="opacity-30 hover:opacity-100">{text}</span>
            <img class="h-full ml-2" src="/assets/images/menu_icon2.svg"/>
            // </div>
            </A>
    }
}
