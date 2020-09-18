use log;

use yew::prelude::{Component, ComponentLink, Properties, html, Html, ShouldRender};
use yew::html::InputData;

pub struct LevelComponent {
    // The link enables us to interact (i.e. reqister callbacks and send messages) with the component itself. See https://yew.rs/docs/en/concepts/components/#create
    link: ComponentLink<Self>,
    // The level's name. This is so the user knows which flag belongs where
    name: String,
    // The flag itself. In the future this will become a hash so that the users can't get the flags using devtools.
    flag: String,
    // The user's guess for the flag, that they are typing
    user_flag: String,
    // Whether the correct flag has been entered.
    flag_correct: bool,
}

// These are the messages (think "events") that can happen in this component.
pub enum LevelMsg {
    // This message indicates that it's time to check the user flag to see if it's the correct one.
    CheckFlag,
    // This message indicates that the user changed the flg they're guessing (when they're typing). Since we need to pass a value, this message has a parameter - see the `view` and `update` methods to see how this is used.
    UserFlagChanged(String),
}

// See https://yew.rs/docs/en/concepts/components/properties/
// The properties allow enable child and parent components to communicate with each other.
// The parent of a level component is the page itself.
#[derive(Clone, PartialEq, Properties)]
pub struct LevelProps {
    // This prop is the level's name. Passed from parent and won't change
    pub name: String,
    // This prop is the level's flag. Passed from parent and won't change
    pub flag: String,
    // This prop indicates whether the user's flag is correct. Not passed from parent, but rather used to communicate back to it from the level.
    #[prop_or(false)]
    pub flag_correct: bool,
}

// See https://yew.rs/docs/en/concepts/components/
// `Component` is a Trait (see https://doc.rust-lang.org/book/ch10-02-traits.html), defined here: https://github.com/yewstack/yew/blob/master/yew/src/html/mod.rs#L30 
impl Component for LevelComponent {
    // Overriding properties since we have our own.
    type Properties = LevelProps;
    // Overriding `Message` since we have our own messages.
    type Message = LevelMsg;

    // See https://yew.rs/docs/en/concepts/components/#create
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::debug!("Creating level {} component", props.name.clone());
        Self {
            link: link,
            // Pass the name from the parent component
            name: props.name,
            // Pass the flag from the parent component
            flag: props.flag,
            // The initial user flag is empty
            user_flag: "".to_string(),
            // This is passed from the parent component as well, but has a default value of `false`.
            flag_correct: props.flag_correct,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LevelMsg::CheckFlag => {
                // TODO - hash
                self.flag_correct = self.user_flag == self.flag
            }
            LevelMsg::UserFlagChanged(value) => {
                log::debug!("In level {}, User flag changed from {} to {}", self.name.clone(), self.user_flag.clone(), value.clone());
                self.user_flag = value;
                self.update(LevelMsg::CheckFlag);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender{
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let label_text = self.name.clone() + "'s flag goes here 🚩";
        html! {
            <>
                <span>
                    <pre>
                        {"DEBUG: I am a level component! Name: "}{self.name.clone()}
                        {" | Flag: "}{self.flag.clone()}
                        {" | Status: "}{get_correct_emoji(self.flag_correct)}
                    </pre>
                    <br/>
                    <div>
                        <div class="input-effect">
                        <input class={ format!("effect-8 effect-10-{}", if self.flag_correct { "good" } else { "bad" }) } type="text" placeholder={label_text.clone()} oninput=self.link.callback(|e: InputData| LevelMsg::UserFlagChanged(e.value)) />
                            <span class="focus-bg"></span>    
                            <span class="focus-border">
                                <i></i>
                            </span>
                        </div>
                        // TODO - delete button
                        <button id="check-flag-btn" onclick=self.link.callback(|_| LevelMsg::CheckFlag)><span>{"Check flag"}</span></button>
                    </div>
                </span>
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
