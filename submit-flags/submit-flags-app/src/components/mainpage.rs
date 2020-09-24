use yew::prelude::{Component, ComponentLink, Properties, html, Html, ShouldRender};
use serde::{Serialize, Deserialize};

use super::level::LevelComponent;

// todo move to same lib as scripts
#[derive(Serialize, Deserialize, Debug)]
struct LevelInfo {
    name: String, 
    flag: String,
}

pub struct MainPage {
    link: ComponentLink<Self>,
    levels: Vec<LevelInfo>,
    all_flags_done: bool,
}

pub enum Msg {
    CheckAllFlags,
}

// TODO mainpage props

impl Component for MainPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let const_level_1 = LevelInfo {name: "name1".to_string(), flag: "bfebba9e53b0108063c9c9e5828c0907337aeeed4363b1aac4da791d9593cec2".to_string()};
        let const_level_2 = LevelInfo {name: "name2".to_string(), flag: "e647a1ad81540b0c4e11048cba1eeae8a9993052a1186a6dd9acf575c834ba83".to_string()};
        let levels_info_vector = vec![const_level_1, const_level_2];

        Self {
            link: link,
            levels: levels_info_vector,
            all_flags_done: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CheckAllFlags => self.all_flags_done = !self.all_flags_done
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main class="site-main section-inner thin animated fadeIn">
                    <h1 id="home-title">{ "Make Git Better CTF - Submit Flags" }</h1>
                    <div id="level-checkers" class="content">
                        { for self.levels.iter().map(create_component_from_level_info) }
                    </div>
                </main>
            </>
        }
    }
}

fn create_component_from_level_info(level_info: &LevelInfo) -> Html {
    html! {
        <LevelComponent name=level_info.name.clone() flag=level_info.flag.clone() />
    }
}
