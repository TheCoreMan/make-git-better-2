use std::collections::HashMap;
use anyhow::Error;
use yew::prelude::{Component, ComponentLink, html, Html, ShouldRender};
use yew::callback::Callback;
use yew::format::Json;
use yew::services::fetch::FetchTask;

use super::level::LevelComponent;
use super::common::{LevelInfo, LevelsInfo, MainPageMsg, CheckFlagCallback};

use super::fetchflags::GetFlagsService;

pub struct MainPage {
    link: ComponentLink<Self>,

    // All the level information - used to create the level components
    levels: Option<Vec<LevelInfo>>,

    // The status of each level component (name to is_correct)
    levels_status: HashMap<String, bool>,

    // If there's an error in any of the states, shove it here so the user can see it.
    error: String,

    // Fetch-related members
    flags_service: GetFlagsService,
    flags_service_response: Option<LevelsInfo>,
    flags_service_callback: Callback<Result<LevelsInfo, Error>>,
    flags_service_task: Option<FetchTask>,
    requested_flags: bool,
}

impl Component for MainPage {
    type Message = MainPageMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            levels: None,
            levels_status: HashMap::new(),
            error: "".to_string(),

            flags_service: GetFlagsService::new("levels_info.json"),
            flags_service_response: None,
            flags_service_callback: link.callback(MainPageMsg::FlagsResponseReady),
            flags_service_task: None,
            requested_flags: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("MainPage: Update message {:?}", msg);
        match msg {
            // Fetch Messages
            MainPageMsg::GetFlagsResponse => {
                log::debug!("Sending a get response");
                let task = self.flags_service.get_response(self.flags_service_callback.clone());
                log::debug!("Sent a get response");
                self.flags_service_task = Some(task);
                self.requested_flags = true;
            }
            MainPageMsg::FlagsResponseReady(Ok(response)) => {
                self.flags_service_response = Some(response);
                log::debug!("Got response: {:?}", Json(self.flags_service_response.clone()));
                // Finally, get the levels from the response. Phew!
                self.levels = Some(self.flags_service_response.as_mut().unwrap().levels.clone());
                for level in self.levels.as_ref().unwrap().iter() {
                    self.levels_status.insert(level.name.clone(), false);
                }
            }
            MainPageMsg::FlagsResponseReady(Err(err)) => {
                log::error!("Error while trying to fetch flags: {:?}", err);
                self.error = format!("{:?}", err);
            }
            // Callback for level component
            MainPageMsg::CheckSingleFlag(status) => {
                *self.levels_status.get_mut(&status.level_name).unwrap() = status.is_correct;
                log::debug!("map {:?}", self.levels_status);
            }
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
        // If you didn't request flags yet, try to.
        if !self.requested_flags {
            log::debug!("Requesting flags for the first time.");
            self.link.send_message(MainPageMsg::GetFlagsResponse);
        }

        html! {
            <>
                <main class="site-main section-inner thin animated fadeIn">
                    <h1 id="home-title">{ "Make Git Better CTF - Submit Flags" }</h1>
                    { self.get_levels_comp() }
                    { self.get_totals_comp() }
                </main>
            </>
        }
    }
}

// Extra, non-component methods for MainPage
impl MainPage {
    fn get_totals_comp(&self) -> Html {
        if self.levels_status.is_empty() {
            html! {}
        } else {
            let mut len = 0;
            let mut counter = 0;
            for (_, is_correct) in self.levels_status.iter() {
                len += 1;
                if *is_correct { 
                    counter += 1; 
                }
            };

            let victory_comp: Html;

            if len == counter {
                victory_comp = html! {
                    <>
                        // fireworks! 
                        <div class="pyro"><div class="before"></div><div class="after"></div></div>
                        <div class="content">
                            <h1>{ "You win! 🏆" }</h1>
                            <p>
                                { "Screenshot this and send it to me to get into the " } 
                                <a target="_blank" rel="noopener noreferrer" href="https://www.mrnice.dev/about/#nc-shay-nehmad-443">{"make-git-better Hall of Fame"}</a>{"! "}
                                <a target="_blank" rel="noopener noreferrer" href="https://www.mrnice.dev/about/#nc-shay-nehmad-443">{"Here's a list of ways to contact me."}</a>
                            </p>
                            <h2>{ "Thanks for playing! 😀" }</h2>
                        </div>
                    </>
                };
            } else { victory_comp = html! { }; }

            html! {
                <div id="totals">
                    <pre>{ format!("{} / {}", counter, len) }</pre>
                    { victory_comp }
                </div>
            }
        }
    }

    fn get_levels_comp(&self) -> Html {
        match &self.levels {
            None => {
                // Check if still laoding, or an actual error
                if self.error.is_empty() {  // Still loading
                    html! {
                        <>
                            <div class="spinner"></div>
                            <p>{ "No levels yet. Loading from server. If this is taking more than a few seconds, check the console logs." }</p>
                        </>
                    }
                } else {  // Error state
                    html! {
                        <>
                            <div class="spinner"></div>
                            <p>{ format!("An error has occured! Details:") }</p>
                            <pre> { self.error.clone() } </pre>
                            <p>{ "Please reach out to Shay Nehmad (@ShayNehmad on Twitter) with the details!" }</p>
                        </>
                    }
                }
            }
            Some(levels) => {
                let render_level_component = |level| {
                    self.create_component_from_level_info(level)
                };
                
                html! {
                    <div id="level-checkers" class="content">
                        { 
                            for levels.iter().map(render_level_component)
                        }
                    </div>
                }
            }
        }
    }

    fn create_component_from_level_info(&self, level_info: &LevelInfo) -> Html {
        let callback: CheckFlagCallback = self.link.clone().callback(MainPageMsg::CheckSingleFlag);
        html! {
            <LevelComponent name=level_info.name.clone() flag=level_info.flag.clone() check_callback=callback />
        }
    }
}
