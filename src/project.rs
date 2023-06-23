use crate::{MainNav, MainNavProps};
use leptos::*;
use leptos_router::*;
use rand::{seq::SliceRandom, thread_rng};
mod neod;
use neod::*;

use self::standard_project::StandardProject;

const IMAGES_ROOT: &'static str = "assets/images/";

#[derive(Params, PartialEq, Debug, Clone)]
struct ProjectParams {
    slug: String,
}

#[derive(Copy, Clone, Debug)]
pub struct ProjectImage {
    pub url: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub alt_text: &'static str,
}
impl Default for ProjectImage {
    fn default() -> Self {
        Self {
            url: "Nature_in_the_Anthropocene1.jpg",
            title: "",
            description: "",
            alt_text: "",
        }
    }
}
impl ProjectImage {
    pub fn full_view(&self, cx: Scope) -> impl IntoView {
        let img_c = "w-full mb-1 rounded";
        let mut url = String::from(IMAGES_ROOT);
        url.push_str(self.url);
        view! { cx,
             <div class=format!("mb-8 rounded hover:opacity-60") >
                 <img class=img_c src=url loading="lazy" />
                 <h2>{self.title}</h2>
                <p>{self.description}</p>
                 </div>

        }
    }
}

pub trait Project {
    fn main_image(&self) -> ProjectImage;
    fn name(&self) -> &'static str;
    fn short_description(&self) -> &'static str;
    fn page_view(&self, cx: Scope) -> View;
    fn tags(&self) -> &[&'static str];
    fn thumbnail(&self, cx: Scope, i: usize) -> View {
        let img_c = "w-full mb-1 rounded";
        let mut url = String::from(IMAGES_ROOT);
        url.push_str(self.main_image().url);
        let height = [
            "h-2/5", "h-1/5", "h-2/5", "h-1/5", "h-3/5", "h-1/5", "h-3/5", "h-1/5", "h-1/5",
        ][i % 9];
        view! { cx,
            <div class=format!("mb-8 rounded hover:opacity-60") >
                <img class=img_c src=url loading="lazy" />
                <h2>{self.name()}</h2>
                </div>
        }
        .into_view(cx)
    }
    fn slug(&self) -> String {
        let slug = String::from(self.name());
        let slug = slug.replace(" ", "_");
        let slug = slug.replace("å", "a");
        let slug = slug.replace("ä", "a");
        let slug = slug.replace("ö", "o");
        let slug = slug.replace("é", "e");
        slug.to_lowercase()
    }
}

pub fn all_projects(cx: Scope) -> Vec<Box<dyn Project>> {
    let nature_in_the_anthropocene = StandardProject {
        name: "Nature in the Anthropocene",
        author: "Erik Natanael Gustafsson, Billy Kemp, Hugh Heart, Melissa????",
        ..Default::default()
    };
    vec![
        Box::new(nature_in_the_anthropocene),
        Box::new(NeodProject::new()),
        // Project {main_image:"Nature_in_the_Anthropocene1.jpg",name:"Nature in the Anthropocene", short_description: "Nature short description", view:  },
    ]
}

#[component]
pub fn ProjectMasonry(cx: Scope) -> impl IntoView {
    let container_c = "grid grid-cols-3 auto-cols-max gap-3 h-full overflow-y-auto";
    let mut projects = all_projects(cx);
    let mut rng = thread_rng();
    projects.shuffle(&mut rng);

    view! { cx,

        <div
            class=container_c
            >
            {projects.into_iter().enumerate()
            .map(|(i, n)| n.thumbnail(cx, i))
            .collect::<Vec<_>>()}
         </div>
    }
}

#[component]
pub fn ProjectOverview(cx: Scope) -> impl IntoView {
    view! { cx,
        <div
         class=("basis-1/3 flex flex-col")
         >
            <MainNav/>
            <p>"En introduktionstext....."</p>
        </div>
        <div
         class=("basis-2/3 ")
        >

           <ProjectMasonry/>
        </div>

    }
}

#[component]
pub fn ProjectDispatcher(cx: Scope) -> impl IntoView {
    let params = use_params::<ProjectParams>(cx);

    let slug = params.with(|params| params.clone().map(|params| params.slug).unwrap_or_default());

    let projects = all_projects(cx);
    let main_body = if let Some(project_index) = projects.iter().position(|p| p.slug() == slug) {
        projects[project_index].page_view(cx)
    } else {
        view! {cx,
               <p>"Project not found"</p>
            <p>{format!("found slug: {slug}")}</p>
        }
        .into_view(cx)
    };
    view! {cx,
           {main_body}

    }
}

#[component]
fn ListProjects(cx: Scope) -> impl IntoView {
    view! { cx,
    }
}

mod standard_project {
    use super::*;
    use leptos::*;

    pub struct StandardProject {
        pub main_image: ProjectImage,
        pub name: &'static str,
        pub short_description: &'static str,
        pub images: Vec<ProjectImage>,
        pub tags: Vec<&'static str>,
        pub credits: &'static str,
        pub author: &'static str,
    }
    impl Default for StandardProject {
        fn default() -> Self {
            Self {
                main_image: Default::default(),
                name: Default::default(),
                short_description: Default::default(),
                images: Default::default(),
                credits: Default::default(),
                author: Default::default(),
                tags: vec![],
            }
        }
    }
    impl Project for StandardProject {
        fn main_image(&self) -> ProjectImage {
            self.main_image
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn short_description(&self) -> &'static str {
            self.short_description
        }

        fn page_view(&self, cx: Scope) -> View {
            view! { cx,
                          <div>
                          <h1>{self.name}</h1>
                          <h3>"by " {self.author} </h3>
                          </div>

            }
            .into_view(cx)
        }

        fn tags(&self) -> &[&'static str] {
            &self.tags
        }
    }
}
