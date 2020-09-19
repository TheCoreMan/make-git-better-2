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
            // This has a default value of `false`. Not passed from parent
            flag_correct: props.flag_correct,
        }
    }

    // See https://yew.rs/docs/en/concepts/components/#update
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Updating level {} component", self.name.clone());
        // Do something different depending on the update message.
        match msg {
            LevelMsg::CheckFlag => {
                log::debug!("In level {}, checking flag", self.name.clone());
                // TODO - Change this to hash instead of flag
                self.flag_correct = self.user_flag == self.flag;
                true  // Re-render
            }
            LevelMsg::UserFlagChanged(new_user_flag) => {
                log::debug!("In level {}, User flag changed from {} to {}", self.name.clone(), self.user_flag.clone(), new_user_flag.clone());
                self.user_flag = new_user_flag;
                self.update(LevelMsg::CheckFlag);
                true  // Re-render
            }
        }
    }

    // See https://yew.rs/docs/en/concepts/components/#change
    // We're not using "change"
    fn change(&mut self, _props: Self::Properties) -> ShouldRender{
        log::debug!("Changing level {} component", self.name.clone());
        false
    }

    // See https://yew.rs/docs/en/concepts/components/#view
    // In this method we're declaring what the element looks like. This is very reminiscent of JSX and React.
    fn view(&self) -> Html {
        log::debug!("Viewing level {} component", self.name.clone());

        // TODO - move to "create"
        let label_text = self.name.clone() + "'s flag goes here 🚩";
        let input_id = self.name.clone() + "-id";

        // Creating the element as variables makes it clearer - similar to functional elements in react

        // This element just prints the component info to make it easier to develop. Will delete soon :)
        let debug_info_element = html! { 
            <pre>
                { 
                    format!("DEBUG: I am a level component! Name: {} | Flag: {} | Status: {}", 
                        self.name.clone(),
                        self.flag.clone(),
                        self.flag_correct) 
                }
                <br/>
            </pre> 
        };

        // This element is the input for the flag.
        let input_element = html! {
            <div class="input-effect">
                <input 
                    id={ input_id.clone() } 
                    /* Change the background colour effect according to the status. If the flag is correct, the class will be "effect-8 effect-10-good",
                     * which paints the BG of the text box green (and stays). Otherwise, paint it in red (as long as it's in focus).
                     */
                    class={ format!("effect-8 effect-10-{}", if self.flag_correct { "good" } else { "bad" }) } 
                    type="text" 
                    placeholder={label_text.clone()} 
                    // Whenever the user inputs something into the box, notify this LevelComponent that the user flag has changed.
                    oninput=self.link.callback(|e: InputData| LevelMsg::UserFlagChanged(e.value))  // <-- important line!
                />
                // Cosmetics
                <span class="focus-bg"></span><span class="focus-border"><i></i></span>
            </div>
        };

        // This element is for a11y - don't indicate status with color only, but with an emoji as well.
        let status_element = html! {
            <pre class="status"> { get_correct_emoji(self.flag_correct) }</pre>
        };

        // This is the complete HTML component we're returning from `view`.
        html! {
            <span>
                // TODO - delete this
                { debug_info_element }
                <div>
                    { input_element }
                    { status_element }
                </div>
            </span>
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
