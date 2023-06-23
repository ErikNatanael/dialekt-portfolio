use leptos::*;
use leptos_router::*;
mod bio;
use bio::*;
mod project;
use project::*;
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
             class="bg-emerald-950 flex flex-col md:flex-row h-screen margin-0 overflow-hidden text-white font-normal font-urbanist"
             >
          // all our routes will appear inside <main>
          <Routes>
            /* ... */
    <Route path="/" view=|cx| view! { cx, <Home/> }/>
    <Route path="/bio" view=|cx| view! { cx, <BioPage/> }>
        <Route path="musician" view=|cx| view! { cx, <BioMusician/> }/>
        <Route path="coder" view=|cx| view! { cx, <BioCoder/> }/>
        <Route path="visual_artist" view=|cx| view! { cx, <BioVisualArtist/> }/>
        <Route path="digital_luthier" view=|cx| view! { cx, <BioDigitalLuthier/> }/>
        <Route path="composer" view=|cx| view! { cx, <BioComposer/> }/>
        <Route path="" view=|cx| view! { cx,
                                         <img src="/assets/images/DSC01437_04_web.jpg"/>
                                         // <img src="/assets/images/DSC01437_04_web_ascii.jpg" class="hover:visible"/>
          }/>
    </Route>
    <Route path="/projects" view=|cx| view! { cx, <ProjectOverview/> }/>
    <Route path="/projects/:slug" view=|cx| view! { cx, <ProjectDispatcher/> }/>
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
                // <h1
                //     class="text-red-600 text-3xl"
                // >
                //     "Home"</h1>
        <div class="w-full">
            <Header />
            <Portfolio />
        </div>

    }
}
#[component]
fn Portfolio(cx: Scope) -> impl IntoView {
    view! { cx,
            <h1>"Projects"</h1>
    }
}
#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <div
        class="flex flex-row w-full"
        >
            <div class="basis-1/2">
                <h1
             class="text-5xl font-thin"
             >"Erik Natanael"</h1>
                <h1 class="text-5xl font-thin">"Gustafsson"</h1>
            </div>
            <div>
                <h1 class="text-3xl font-thin">"Portfolio for the Dialekt open call application"</h1>
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
fn BioMusician(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"musician"</h1>
            <p>"Nunc eget lorem dolor sed viverra ipsum nunc aliquet bibendum. Dolor morbi non arcu risus quis varius quam quisque id. Lectus sit amet est placerat in. Sed risus pretium quam vulputate dignissim suspendisse. Purus ut faucibus pulvinar elementum integer enim neque volutpat. Lorem sed risus ultricies tristique. Varius duis at consectetur lorem donec massa sapien. Aliquet lectus proin nibh nisl condimentum id venenatis a condimentum. Imperdiet dui accumsan sit amet."</p>
    }
}
#[component]
fn BioComposer(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"composer"</h1>
            <p>"
    Lacinia quis vel eros donec. Faucibus interdum posuere lorem ipsum. Porttitor lacus luctus accumsan tortor posuere. Scelerisque varius morbi enim nunc faucibus a pellentesque sit amet. Quam id leo in vitae. Orci nulla pellentesque dignissim enim sit. Porta nibh venenatis cras sed felis eget velit aliquet sagittis. Adipiscing diam donec adipiscing tristique. A cras semper auctor neque vitae. Lacus sed viverra tellus in hac habitasse platea dictumst. Viverra tellus in hac habitasse platea dictumst. Eget dolor morbi non arcu risus quis. Aliquam id diam maecenas ultricies mi. Fames ac turpis egestas sed tempus urna. Lectus proin nibh nisl condimentum id venenatis a condimentum. Massa eget egestas purus viverra accumsan in. Elit at imperdiet dui accumsan sit. Erat nam at lectus urna duis. Lacus viverra vitae congue eu consequat ac felis donec."</p>
    }
}
#[component]
fn BioCoder(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"coder"</h1>
            <p>"Nunc eget lorem dolor sed viverra ipsum nunc aliquet bibendum. Dolor morbi non arcu risus quis varius quam quisque id. Lectus sit amet est placerat in. Sed risus pretium quam vulputate dignissim suspendisse. Purus ut faucibus pulvinar elementum integer enim neque volutpat. Lorem sed risus ultricies tristique. Varius duis at consectetur lorem donec massa sapien. Aliquet lectus proin nibh nisl condimentum id venenatis a condimentum. Imperdiet dui accumsan sit amet."</p>
    }
}
#[component]
fn BioVisualArtist(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"visual artist"</h1>
            <p>"Viverra justo nec ultrices dui. Sit amet consectetur adipiscing elit duis tristique sollicitudin nibh. Rhoncus mattis rhoncus urna neque viverra justo. Magnis dis parturient montes nascetur ridiculus mus. Pellentesque nec nam aliquam sem et tortor consequat id. Vitae tortor condimentum lacinia quis vel. Ac orci phasellus egestas tellus rutrum. Integer eget aliquet nibh praesent tristique magna sit."</p>
    }
}
#[component]
fn BioDigitalLuthier(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"digital luthier"</h1>
            <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Orci a scelerisque purus semper eget duis at. Consequat id porta nibh venenatis cras sed felis eget velit. Eu volutpat odio facilisis mauris sit amet massa vitae. Ut tristique et egestas quis ipsum suspendisse ultrices gravida dictum. Posuere lorem ipsum dolor sit amet consectetur adipiscing elit duis. Pharetra vel turpis nunc eget lorem dolor. Sit amet nisl purus in mollis nunc sed. Orci nulla pellentesque dignissim enim. Id cursus metus aliquam eleifend mi in nulla posuere sollicitudin. Iaculis at erat pellentesque adipiscing commodo elit at imperdiet dui. Mattis aliquam faucibus purus in massa tempor nec feugiat. Turpis egestas sed tempus urna et. Ullamcorper eget nulla facilisi etiam dignissim. Eleifend quam adipiscing vitae proin. Id neque aliquam vestibulum morbi blandit cursus. Varius quam quisque id diam vel quam elementum. Sed elementum tempus egestas sed sed risus pretium quam vulputate. Ullamcorper malesuada proin libero nunc consequat interdum varius.

Elit ut aliquam purus sit amet luctus. Ornare aenean euismod elementum nisi quis eleifend quam adipiscing vitae. Vitae tempus quam pellentesque nec nam. Tincidunt augue interdum velit euismod in pellentesque. Platea dictumst quisque sagittis purus sit amet volutpat consequat mauris. "</p>
    }
}

// #[component]
// fn MainNav(cx: Scope) -> impl IntoView {
//     view! { cx,
//                 // <!-- Main navigation container -->
//     <nav
//       class="relative flex w-full flex-wrap items-center justify-between bg-neutral-900 py-2 text-neutral-200 shadow-lg lg:flex-wrap lg:justify-start lg:py-4"
//       data-te-navbar-ref>
//       <div class="flex w-full flex-wrap items-center justify-between px-3">
//         // <!-- Hamburger button for mobile view -->
//         <button
//           class="block border-0 bg-transparent px-2 text-neutral-200 hover:no-underline hover:shadow-none focus:no-underline focus:shadow-none focus:outline-none focus:ring-0 lg:hidden"
//           type="button"
//           data-te-collapse-init
//           data-te-target="#navbarSupportedContent4"
//           aria-controls="navbarSupportedContent4"
//           aria-expanded="false"
//           aria-label="Toggle navigation">
//           // <!-- Hamburger icon -->
//           <span class="[&>svg]:w-7">
//             <svg
//               xmlns="http://www.w3.org/2000/svg"
//               viewBox="0 0 24 24"
//               fill="currentColor"
//               class="h-7 w-7">
//               <path
//                 fill-rule="evenodd"
//                 d="M3 6.75A.75.75 0 013.75 6h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 6.75zM3 12a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 12zm0 5.25a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75z"
//                 clip-rule="evenodd" />
//             </svg>
//           </span>
//         </button>

//         // <!-- Collapsible navigation container -->
//         <div
//           class="!visible mt-2 hidden flex-grow basis-[100%] items-center lg:mt-0 lg:!flex lg:basis-auto"
//           id="navbarSupportedContent4"
//           data-te-collapse-item>
//           // <!-- Navbar title -->
//           <a class="pr-2 text-xl font-semibold text-white" href="#">"Navbar"</a>
//           // <!-- Left navigation links -->
//           <ul
//             class="list-style-none mr-auto flex flex-col pl-0 lg:flex-row"
//             data-te-navbar-nav-ref>
//             // <!-- Dashboard link -->
//             <li class="my-4 lg:my-0 lg:pr-2" data-te-nav-item-ref>
//               <a
//                 class="text-white disabled:text-black/30 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
//                 href="#"
//                 data-te-nav-link-ref
//                 >"Dashboard"</a
//               >
//             </li>
//             // <!-- Team link -->
//             <li class="mb-4 lg:mb-0 lg:pr-2" data-te-nav-item-ref>
//               <a
//                 class="p-0 text-white opacity-60 hover:opacity-80 focus:opacity-80 disabled:text-black/30 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
//                 href="#"
//                 data-te-nav-link-ref
//                 >"Team"</a
//               >
//             </li>
//             // <!-- Projects link -->
//             <li class="mb-4 lg:mb-0 lg:pr-2" data-te-nav-item-ref>
//               <a
//                 class="p-0 text-white opacity-60 hover:opacity-80 focus:opacity-80 disabled:text-black/30 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
//                 href="#"
//                 data-te-nav-link-ref
//                 >"Projects"</a
//               >
//             </li>
//           </ul>
//         </div>

//         // <!-- Right elements -->
//         <div class="relative flex items-center">
//           // <!-- Cart Icon -->
//           <a
//             class="mr-4 text-white opacity-60 hover:opacity-80 focus:opacity-80"
//             href="#">
//             <span class="[&>svg]:w-5">
//               <svg
//                 xmlns="http://www.w3.org/2000/svg"
//                 viewBox="0 0 24 24"
//                 fill="currentColor"
//                 class="h-5 w-5">
//                 <path
//                   d="M2.25 2.25a.75.75 0 000 1.5h1.386c.17 0 .318.114.362.278l2.558 9.592a3.752 3.752 0 00-2.806 3.63c0 .414.336.75.75.75h15.75a.75.75 0 000-1.5H5.378A2.25 2.25 0 017.5 15h11.218a.75.75 0 00.674-.421 60.358 60.358 0 002.96-7.228.75.75 0 00-.525-.965A60.864 60.864 0 005.68 4.509l-.232-.867A1.875 1.875 0 003.636 2.25H2.25zM3.75 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM16.5 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0z" />
//               </svg>
//             </span>
//           </a>

//           // <!-- Container with two dropdown menus -->
//           <div class="relative" data-te-dropdown-ref>
//             // <!-- First dropdown trigger -->
//             <a
//               class="hidden-arrow mr-4 flex items-center text-white opacity-60 hover:opacity-80 focus:opacity-80"
//               href="#"
//               id="dropdownMenuButton1"
//               role="button"
//               data-te-dropdown-toggle-ref
//               aria-expanded="false">
//               // <!-- Dropdown trigger icon -->
//               <span class="[&>svg]:w-5">
//                 <svg
//                   xmlns="http://www.w3.org/2000/svg"
//                   viewBox="0 0 24 24"
//                   fill="currentColor"
//                   class="h-5 w-5">
//                   <path
//                     fill-rule="evenodd"
//                     d="M5.25 9a6.75 6.75 0 0113.5 0v.75c0 2.123.8 4.057 2.118 5.52a.75.75 0 01-.297 1.206c-1.544.57-3.16.99-4.831 1.243a3.75 3.75 0 11-7.48 0 24.585 24.585 0 01-4.831-1.244.75.75 0 01-.298-1.205A8.217 8.217 0 005.25 9.75V9zm4.502 8.9a2.25 2.25 0 104.496 0 25.057 25.057 0 01-4.496 0z"
//                     clip-rule="evenodd" />
//                 </svg>
//               </span>
//               // <!-- Notification counter -->
//               <span
//                 class="absolute -mt-2.5 ml-2 rounded-full bg-red-700 px-1.5 py-0 text-xs text-white"
//                 >1</span
//               >
//             </a>
//             // <!-- First dropdown menu -->
//             <ul
//               class="absolute left-auto right-0 z-[1000] float-left m-0 mt-1 hidden min-w-max list-none overflow-hidden rounded-lg border-none bg-white bg-clip-padding text-left text-base shadow-lg dark:bg-neutral-700 [&[data-te-dropdown-show]]:block"
//               aria-labelledby="dropdownMenuButton1"
//               data-te-dropdown-menu-ref>
//               // <!-- First dropdown menu items -->
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Action"</a
//                 >
//               </li>
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Another action"</a
//                 >
//               </li>
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Something else here"</a
//                 >
//               </li>
//             </ul>
//           </div>

//           // <!-- Second dropdown container -->
//           <div class="relative" data-te-dropdown-ref>
//             <ul
//               class="absolute left-auto right-0 z-[1000] float-left m-0 mt-1 hidden min-w-max list-none overflow-hidden rounded-lg border-none bg-white bg-clip-padding text-left text-base shadow-lg dark:bg-neutral-700 [&[data-te-dropdown-show]]:block"
//               aria-labelledby="dropdownMenuButton2"
//               data-te-dropdown-menu-ref>
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Action"</a
//                 >
//               </li>
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Another action"</a
//                 >
//               </li>
//               <li>
//                 <a
//                   class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
//                   href="#"
//                   data-te-dropdown-item-ref
//                   >"Something else here"</a
//                 >
//               </li>
//             </ul>
//           </div>
//         </div>
//       </div>
//     </nav>
//         }
// }
//
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
