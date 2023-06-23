use leptos::*;

use super::{Project, ProjectImage};

struct Thought {
    title: &'static str,
    text: &'static str,
}
impl Thought {
    pub fn view(&self, cx: Scope) -> impl IntoView {
        view! { cx,
            <div class="mt-12 mb-24 ">
                <h3 class="text-3xl max-w-prose">{self.title}</h3>
                <p>{self.text}</p>
                </div>
        }
    }
}

pub struct NeodProject {
    embedded_media: Vec<&'static str>,
    thoughts: Vec<Thought>,
    images: Vec<ProjectImage>,
    description: &'static str,
}

impl NeodProject {
    pub fn new() -> Self {
        let thoughts = vec![
            Thought {
                title: "Genesis",
                text: "Lorem ipsum.....",
            },
            Thought {
                title: "Genesis",
                text: "Lorem ipsum.....",
            },
            Thought {
                title: "Genesis",
                text: "Lorem ipsum.....",
            },
            Thought {
                title: "Genesis",
                text: "Lorem ipsum.....",
            },
            Thought {
                title: "Genesis",
                text: "Lorem ipsum.....",
            },
        ];
        let embedded_media = vec![];
        let photos = vec![ProjectImage {
            url: "neod_turkos_1920_compressed.jpg",
            title: "neod.1",
            description: "The cover for the neod sound diary, neod.1.anortosit",
            alt_text: "",
        }];
        let description = "
Neoden är inte bara objekt. Den är fortsättningen på ett kosmiskt garnnystan av ideer långt äldre än jag själv. Den är en knopp på livets träd. Den är ett musikinstrument, ett sinnesting som låter oss nå våra inte världar och röra vid det gudomliga; en praktik lika gammal som mänskligheten själv.

 Objektet neod är ett snäckskal på stranden, en stenig geologisk lämning av en levande _fantastisk_ miljö. I sin fysiska form får det fäste i världen; kraft att länka samman inre världar; mjukhet att bli ett kärl för andra idéer.
";
        Self {
            embedded_media,
            thoughts,
            images: photos,
            description,
        }
    }
    pub fn main_window(&self, cx: Scope) -> View {
        let column = "basis-1/2 md:basis-1/5 shrink-0";
        let num_images = self.images.len();
        let images_column_1 = self.images[0..num_images / 2]
            .iter()
            .map(|p| p.full_view(cx))
            .collect::<Vec<_>>();
        let images_column_2 = self.images[(num_images / 2 + 1)..]
            .iter()
            .map(|p| p.full_view(cx))
            .collect::<Vec<_>>();
        let thoughts = self.thoughts.iter().map(|t| t.view(cx)).collect::<Vec<_>>();
        let desc = self
            .description
            .split("\n")
            .map(move |segment| view! { cx, <p class="my-4">{segment}</p>})
            .collect::<Vec<_>>();
        view! { cx,
            <div class="grid md:grid-cols-5 w-full p-4">
                <div class={column}>{images_column_1}</div>
                <div class={column}></div>
                <div class={column}>
                    <h1 class="text-5xl">"neod"</h1>
                    {desc}
                </div>
                <div class={column}>{thoughts}</div>
                <div class={column}>{images_column_2}</div>
            </div>
        }
        .into_view(cx)
    }
}

impl Project for NeodProject {
    fn main_image(&self) -> ProjectImage {
        self.images[0]
    }

    fn name(&self) -> &'static str {
        "The Neod"
    }

    fn short_description(&self) -> &'static str {
        "A digital musical instrument"
    }

    fn page_view(&self, cx: Scope) -> View {
        self.main_window(cx)
    }

    fn tags(&self) -> &[&'static str] {
        &["music", "neod", "instrument", "dmi"]
    }
}
