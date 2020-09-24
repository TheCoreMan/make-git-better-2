use anyhow::Error;
use yew::prelude::{Component, ComponentLink, html, Html, ShouldRender};
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
    error: String,

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
        Self {
            link: link.clone(),
            levels: None,
            all_flags_done: false,
            error: "".to_string(),

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
                self.error = format!("{:?}", err);
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
                </main>
            </>
        }
    }
}

// Extra, non-component functions for MainPage
impl MainPage {
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
