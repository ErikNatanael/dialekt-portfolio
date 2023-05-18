use leptos::*;

const IMAGES_ROOT: &'static str = "assets/images/";

pub struct Project {
    main_image: &'static str,
    name: &'static str,
}

impl Project {
    pub fn thumbnail(&self, cx: Scope, i: usize) -> impl IntoView {
        let img_c = "w-full mb-8 rounded";
        let mut url = String::from(IMAGES_ROOT);
        url.push_str(self.main_image);
        let height = ["h-2/5", "h-1/5", "h-2/5", "h-1/5", "h-3/5", "h-1/5", "h-3/5", "h-1/5", "h-1/5"][i%9];
        view! { cx,
            <div class=format!("bg-cover bg-center w-full mb-8 rounded hover:opacity-60 {height}") style={format!("background-image: url({url})")}>
                <h2>{self.name}</h2>
                </div>
        }
    }
}

pub fn all_projects() -> Vec<Project> {
    vec![
        Project {
            main_image: "Nature_in_the_Anthropocene1.jpg",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20200605_111748.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20220704_174152.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2021-09-28-17h54m39s766.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2022-10-10-14h49m51s992.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Nature_in_the_Anthropocene1.jpg",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20200605_111748.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20220704_174152.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2021-09-28-17h54m39s766.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2022-10-10-14h49m51s992.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Nature_in_the_Anthropocene1.jpg",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20200605_111748.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20220704_174152.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2021-09-28-17h54m39s766.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2022-10-10-14h49m51s992.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Nature_in_the_Anthropocene1.jpg",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20200605_111748.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "Screenshot_20220704_174152.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2021-09-28-17h54m39s766.png",
            name: "Nature in the Anthropocene",
        },
        Project {
            main_image: "vlcsnap-2022-10-10-14h49m51s992.png",
            name: "Nature in the Anthropocene",
        },
    ]
}

#[component]
pub fn ProjectMasonry(cx: Scope) -> impl IntoView {
    let container_c = "columns-3 gap-8 h-full";
    let projects = all_projects();

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
