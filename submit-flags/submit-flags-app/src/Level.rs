use yew::prelude::{Component, ComponentLink, Properties, html, Html, ShouldRender};

pub struct LevelComponent {
    link: ComponentLink<Self>,
    name: String,
    flag: String,
    flagCorrect: bool,
}

pub enum LevelMsg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LevelProps {
    #[prop_or_default]
    pub name: String,
    pub flag: String,
}

impl Component for LevelComponent {
    type Properties = LevelProps;
    type Message = LevelMsg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            name: props.name,
            flag: props.flag,
            flagCorrect: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender{
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Level Component is here!" }</h1>
                <p>{"Name: "}{self.name.clone()}{" | Flag: "}{self.flag.clone()}{" | Status: "}{get_correct_emoji(self.flagCorrect)}</p>
                <button class="menu-btn">{"Check flag"}</button>
            </>
        }
    }
}

fn get_correct_emoji(correct: bool) -> String {
    if correct {
        return "✔".to_string();
    } else {
        return "❌".to_string();
    }
}
