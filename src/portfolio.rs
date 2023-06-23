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
    view! { cx,
      <div>
      <h3>{&self.title}</h3>
      // media
      <span></span>
      <span>{&self.description}</span>
      <br/>
      </div>

    }
  }
}

pub enum PortfolioMedia {
    Embed(String),
    Image(String),
}

pub fn all_portfolio_items() -> Vec<PortfolioItem> {
    vec![
    PortfolioItem { 
      title: "sys|calls - excerpt".to_string(), 
      description: "Audiovisual performance work with quad channel audio and floor projection. System calls, the communication between programs and the operating system, are traced and this massive data stream directs sonic and visual processes. The live performance consists of normal computer work: browsing the internet, checking and writing emails etc. A hidden side of software brought to light.".to_string(), 
      my_role: "Composition, sound synthesis, system call data collection, network infrastructure".to_string(), 
      relevance: "".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "sys|calls - images".to_string(), 
      description: "".to_string(), 
      my_role: "".to_string(), 
      relevance: "".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "Master work - excerpt".to_string(), 
      description: "For my master's degree, I investigated the metaphor of fragility as the basis for an audiovisual live performance system. This is an excerpt of a longer work.".to_string(), 
      my_role: "Instrument design, composition, sound synthesis, visual synthesis, performance".to_string(), 
      relevance: "".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "ergia - excerpt".to_string(), 
      description: "".to_string(), 
      my_role: "Performer, composition, sound design, programming".to_string(), 
      relevance: "".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "Brytpunkt - excerpt".to_string(), 
      description: "Audiovisual fixed media work around liminality and chaos".to_string(), 
      my_role: "Composition, sound design, visual design, programming".to_string(), 
      relevance: "".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "Deviance - selected clips".to_string(), 
      description: "Deviance is an audiovisual commission from Zubin Kanga set to premier 30 September. The piece consists of a piano part, a visual part and an electronic music part. It is based on EEG data taken while listening to two previous musical works. Both the original music and the brain activity data are used to compose the 60 visual events distributed over a 15 minute piece. These clips are not for dissemination.".to_string(), 
      my_role: "Visual design and programming".to_string(), 
      relevance: "This is the latest finished work using the visual engine I will base the proposed project on.".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
    PortfolioItem { 
      title: "Neodalbumintro".to_string(), 
      description: "Intro to my yet to be released album featuring 53edo music on the neod, inspired by old Scandinavian microtonality.".to_string(), 
      my_role: "Composition, performance, mixing, instrument".to_string(), 
      relevance: "This soft and fragile melodic style is representative of the musical direction I want to explore during the residency, in combination with the noisier music as heard in sys|calls above.".to_string(),
      media: vec![PortfolioMedia::Embed("".to_string())] 
    },
  ]
}
