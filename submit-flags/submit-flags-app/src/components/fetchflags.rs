use anyhow::{anyhow, Error};

use yew::callback::Callback;
use yew::format::{Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use super::common::LevelsInfo;

#[derive(Default)]
pub struct GetFlagsService {
    file_path: &'static str,
}

impl GetFlagsService {
    pub fn new(file_path: &'static str) -> Self {
        Self {
            file_path,
        }
    }

    pub fn get_response(&mut self, callback: Callback<Result<LevelsInfo, Error>>) -> FetchTask {
        let handler = move |response: Response<Result<String, Error>>| {
            let (head, body) = response.into_parts();
            if head.status.is_success() {
                log::debug!("Response is a success");
                let body_value = body.unwrap();
                log::debug!("here's the body: {}", body_value);
                let parsed: LevelsInfo = serde_json::from_str(&body_value).unwrap();
                log::debug!("JSON conversion went well! Found {} levels", parsed.levels.len());
                callback.emit(Ok(parsed))
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error getting levels from server",
                    head.status
                )))
            }
        };

        // Local server
        let url = format!("/{}", self.file_path);

        let request = Request::get(url.clone().as_str()).body(Nothing).unwrap();
        log::debug!("Created get request to URI {}", request.uri());
        FetchService::fetch(request, handler.into()).unwrap()
    }
}
