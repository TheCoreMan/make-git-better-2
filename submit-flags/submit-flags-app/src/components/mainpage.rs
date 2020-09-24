use yew::prelude::{Component, ComponentLink, html, Html, ShouldRender};

use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

use super::level::LevelComponent;
use super::common::{LevelInfo, LevelsInfo};

use super::fetchflags::GetFlagsService;

pub struct MainPage {
    link: ComponentLink<Self>,
    levels: Option<Vec<LevelInfo>>,
    all_flags_done: bool,

    // Fetch-related members
    flags_service: GetFlagsService,
    flags_service_response: Option<LevelsInfo>,
    flags_service_callback: Callback<Result<LevelsInfo, Error>>,
    flags_service_task: Option<FetchTask>,
    requested_flags: bool,
}

#[derive(Debug)]
pub enum MainPageMsg {
    CheckAllFlags,
    // Fetch-related messages
    GetFlagsResponse,
    FlagsResponseReady(Result<LevelsInfo, Error>),
}

impl Component for MainPage {
    type Message = MainPageMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        /*let const_level_1 = LevelInfo {name: "name1".to_string(), flag: "bfebba9e53b0108063c9c9e5828c0907337aeeed4363b1aac4da791d9593cec2".to_string()};
        let const_level_2 = LevelInfo {name: "name2".to_string(), flag: "e647a1ad81540b0c4e11048cba1eeae8a9993052a1186a6dd9acf575c834ba83".to_string()};
        let levels_info_vector = vec![const_level_1, const_level_2];

        let levels = LevelsInfo { levels: levels_info_vector };*/

        Self {
            link: link.clone(),
            levels: None,
            all_flags_done: false,

            flags_service: GetFlagsService::new("levels_info.json"),
            flags_service_response: None,
            flags_service_callback: link.callback(MainPageMsg::FlagsResponseReady),
            flags_service_task: None,
            requested_flags: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("MainPage: Update message {:?}", msg);
        match msg {
            MainPageMsg::CheckAllFlags => {
                // TODO: impl this
                self.all_flags_done = !self.all_flags_done;
            }
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
            }
            MainPageMsg::FlagsResponseReady(Err(err)) => {
                log::error!("Error while trying to fetch flags: {:?}", err);
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
                    { self.levels_comp() }
                </main>
            </>
        }
    }
}

// Extra, non-component functions for MainPage
impl MainPage {
    fn levels_comp(&self) -> Html {
        match &self.levels {
            None => {
                html! {
                    <div><p>{ "No levels yet. :( Check console logs." }</p></div>
                }
            }
            Some(levels) => {
                html! {
                    <div id="level-checkers" class="content">
                        {
                            for levels.iter().map(create_component_from_level_info)
                        }
                    </div>
                }
            }
        }
    }
}

fn create_component_from_level_info(level_info: &LevelInfo) -> Html {
    html! {
        <LevelComponent name=level_info.name.clone() flag=level_info.flag.clone() />
    }
}
