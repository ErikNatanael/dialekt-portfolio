use leptos::{IntoView, Scope, view};

pub struct PortfolioItem {
    title: String,
    description: String,
    my_role: String,
    relevance: String,
    media: Vec<PortfolioMedia>,
}

impl PortfolioItem {
  pub fn view(&self, cx: Scope) -> impl IntoView {
    let media: Vec<_> = self.media.iter().map(|m| match m {
      PortfolioMedia::Embed(s) => view!{ cx, <div class="pl-4" inner_html={s}></div>}.into_view(cx),
      PortfolioMedia::Image(s) => view!{ cx, <img src={s} />}.into_view(cx),
        PortfolioMedia::Sound(filename) => {
          let mut s = String::from("assets/sounds/"); 
          s.push_str(filename);
          view!{ cx, 
          <audio controls class="w-full pl-4">
            <source src={s} type="audio/mpeg" />
            //"Your browser does not support the audio tag."
          </audio> 
        }.into_view(cx)
      },
    }).collect();
    let text_div = "py-2";
    view! { cx,
      <div
      class="m-4 flex flex-col lg:flex-row border border-emerald-800 p-4"
      >
        <div class="basis-full lg:basis-1/2">
          <h3 class="text-3xl mb-4">{&self.title}</h3>
          
          <div class=text_div><span class="font-bold">"Description: "</span><span>{&self.description}</span></div>
          
          <div class=text_div><span class="font-bold">"My role: "</span><span>{&self.my_role}</span></div>

          <div class=text_div><span class="font-bold">"Relevance: "</span><span>{&self.relevance}</span></div>

        </div>
        <div class="basis-full lg:basis-1/2 grid place-items-center">
        {media}
        </div>
      </div>

    }
  }
}

pub enum PortfolioMedia {
    Embed(String),
    Image(String),
    Sound(String),
}

pub fn all_portfolio_items() -> Vec<PortfolioItem> {
    vec![
    PortfolioItem { 
      title: "sys|calls".to_string(), 
      description: "Excerpt of an 18 minute piece. Audiovisual performance work with quad channel audio and floor projection. System calls, the communication between programs and the operating system, are traced and this massive data stream directs sonic and visual processes. The live performance consists of normal computer work: browsing the internet, checking and writing emails etc. A hidden side of software brought to light.".to_string(), 
      my_role: "Composition, sound synthesis, system call data collection, network infrastructure".to_string(), 
      relevance: "The latest audiovisual performance I was involved in. This is representative for the digital mode of composition/performance.".to_string(),
      media: vec![PortfolioMedia::Embed("<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/KmKgrimCLrw\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>".to_string())] 
    },
    // PortfolioItem { 
    //   title: "sys|calls - images".to_string(), 
    //   description: "".to_string(), 
    //   my_role: "".to_string(), 
    //   relevance: "".to_string(),
    //   media: vec![PortfolioMedia::Embed("".to_string())] 
    // },
    PortfolioItem { 
      title: "Glänt".to_string(), 
      description: "Intro to my yet to be released album featuring 53edo music on the neod, inspired by old Scandinavian microtonality.".to_string(), 
      my_role: "Composition, performance, mixing, instrument".to_string(), 
      relevance: "This soft and fragile melodic style is representative of the musical direction I want to explore during the residency, in combination with the noisier music as heard in sys|calls above.".to_string(),
      media: vec![PortfolioMedia::Sound("Glant.mp3".to_string())] 
    },
    PortfolioItem { 
      title: "Master work - excerpt".to_string(), 
      description: "For my master's degree, I investigated the metaphor of fragility as the basis for an audiovisual live performance system. This is an excerpt of a longer work.".to_string(), 
      my_role: "Instrument design, composition, sound synthesis, visual synthesis, performance".to_string(), 
      relevance: "Audiovisual live performance on my own instrument.".to_string(),
      media: vec![PortfolioMedia::Embed("<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/6i4FmrIRZeA\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>".to_string())] 
    },
    PortfolioItem { 
      title: "ergia - excerpt".to_string(), 
      description: "ergia for solo viola and live electronics. The viola draws out whispers and evokes an organic soundscape of small digital noises. Electronic sounds and processes are triggered using machine listening.".to_string(), 
      my_role: "Performer, composition, sound design, programming".to_string(), 
      relevance: "Documentation of live performance music and tech experience.".to_string(),
      media: vec![PortfolioMedia::Embed("<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/xI1SG42B1iY\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>".to_string())] 
    },
    PortfolioItem { 
      title: "Match".to_string(), 
      description: "Intro to Match. Dolores Catherino and I were invited to collaborate on the intro to this Hear Between The Lines song. She plays in 72edo, I in 53edo.".to_string(), 
      my_role: "Co-composition, playing the neod".to_string(), 
      relevance: "A more microtonally developed composition, showing off what the neod is capable of in that regard.".to_string(),
      media: vec![PortfolioMedia::Sound("Match_intro.mp3".to_string())] 
    },
    // PortfolioItem { 
    //   title: "Brytpunkt - excerpt".to_string(), 
    //   description: "Audiovisual fixed media work around liminality and chaos".to_string(), 
    //   my_role: "Composition, sound design, visual design, programming".to_string(), 
    //   relevance: "".to_string(),
    //   media: vec![PortfolioMedia::Embed("".to_string())] 
    // },
    PortfolioItem { 
      title: "Deviance - selected clips".to_string(), 
      description: "Deviance is an audiovisual commission from Zubin Kanga set to premier 30 September. The piece consists of a piano part, a visual part and an electronic music part. It is based on EEG data taken while listening to two previous musical works. Both the original music and the brain activity data are used to compose the 60 visual events distributed over a 15 minute piece. These clips are not for dissemination.".to_string(), 
      my_role: "Visual design and programming".to_string(), 
      relevance: "This is the latest finished work using the visual engine I will base the proposed project on. The visuals can be generated live at 4k60.".to_string(),
      media: vec![
        PortfolioMedia::Embed("<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/dtvu2qQQcuI\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>".to_string()),
        PortfolioMedia::Embed("<iframe width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/ESZgxPF-C3k\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>".to_string()),
        
        ] 
    },
  ]
}

pub struct Image {
  name: String,
  description: String,
}

impl Image {
  pub fn view(&self, cx: Scope) -> impl IntoView {
    let mut url = String::from("/assets/images/");
    url.push_str(&self.name);
    
    let img_c = "w-full mb-8 rounded";
    view! { cx,
      <div
      class=""
      >
        <img class=img_c src=url/>
        <p>{&self.description}</p>
      </div>
    }
  }
}

pub fn graphics_images() -> Vec<Image> {
  vec![
    Image {name:"first_particle.jpg".to_string(), description: "".to_string() },
    Image {name:"particles1.jpg".to_string(), description: "".to_string() },
    Image {name:"particles2.jpg".to_string(), description: "".to_string() },
    Image {name:"particles3.jpg".to_string(), description: "".to_string() },
  ]
}


pub fn me_images() -> Vec<Image> {
  vec![
    Image {name:"GLIDEPHOTOS-5074.jpg".to_string(), description: "".to_string() },
    Image {name:"GLIDEPHOTOS-5081.jpg".to_string(), description: "".to_string() },
    Image {name:"neod.jpg".to_string(), description: "".to_string() },
    Image {name:"neod_turkos_1920_compressed.jpg".to_string(), description: "".to_string() },
    Image {name:"scarda.jpg".to_string(), description: "".to_string() },
    Image {name:"octarion.JPG".to_string(), description: "".to_string() },
    

  ]
}