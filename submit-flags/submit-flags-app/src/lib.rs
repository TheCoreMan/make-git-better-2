mod Level;

use wasm_bindgen::prelude::{wasm_bindgen};
use yew::prelude::{Component, ComponentLink, ShouldRender, Html, html, App};
use Level::LevelComponent;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
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
                    <h1>{ "Make Git Better CTF - Submit Flags" }</h1>
                    <div class="content">
                        <LevelComponent name="levelname" flag="levelflag"/>
                    </div>
                </main>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
