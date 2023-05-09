use leptos::*;

#[derive(PartialEq, Copy, Clone, Eq)]
pub enum BioSection {
    Home,
    Musician,
    Composer,
    Coder,
    VisualArtist,
    InstrumentBuilder,
}
impl From<&str> for BioSection {
    fn from(name: &str) -> Self {
        match name {
            "musician" => Self::Musician,
            "composer" => Self::Composer,
            "coder" => Self::Coder,
            "visual_artist" => Self::VisualArtist,
            "instrument_builder" => Self::InstrumentBuilder,
            _ => Self::Home,
        }
    }
}
impl BioSection {
    fn text(&self) -> &'static str {
        match self {
            BioSection::Home => "Home text",
            BioSection::Musician => "Musician text!",
            BioSection::Composer => "Composer text!",
            BioSection::Coder => todo!(),
            BioSection::VisualArtist => todo!(),
            BioSection::InstrumentBuilder => todo!(),
        }
    }
}

#[component]
pub fn BioMasonry(cx: Scope) -> impl IntoView {
    let container_c = "columns-3 gap-8 h-full";
    let img_c = "w-full mb-8 rounded";
    let url = "https://vignette.wikia.nocookie.net/theunitedorganizationtoonsheroes/images/6/68/Link-0.png/revision/latest?cb=20171009143548";

    view! { cx,

        <div
            class=container_c
            >
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
             <img class=img_c src=url/>
         </div>
    }
}
