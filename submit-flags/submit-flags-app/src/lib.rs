// See https://github.com/yewstack/yew/issues/97
#![recursion_limit="256"]

mod level;

use wasm_bindgen::prelude::{wasm_bindgen};
use yew::prelude::{Component, ComponentLink, ShouldRender, Html, html, App};
use level::LevelComponent;
use wasm_logger;

struct LevelInfo {
    name: String, 
    flag: String,
}

struct SubmitFlagsPage {
    link: ComponentLink<Self>,
    levels: Vec<LevelInfo>,
    all_flags_done: bool,
}

enum Msg {
    CheckAllFlags,
}

impl Component for SubmitFlagsPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO change to read from file
        let const_level_1 = LevelInfo {name: "name1".to_string(), flag: "flag1".to_string()};
        let const_level_2 = LevelInfo {name: "name2".to_string(), flag: "flag2".to_string()};

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

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<SubmitFlagsPage>::new().mount_to_body();
}
