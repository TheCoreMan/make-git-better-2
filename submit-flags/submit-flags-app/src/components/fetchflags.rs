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
                let parsed = try_to_parse_levels_json(&body_value);
                match parsed {
                    Ok(v) => {
                        log::debug!("JSON conversion went well! Found {} levels", v.levels.len());
                        callback.emit(Ok(v))
                    }
                    Err(e) => {
                       callback.emit(Err(anyhow!("{:?}", e)));
                    }
                }
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error getting levels from server",
                    head.status
                )))
            }
        };

        // Local server
        let url = format!("/{}", self.file_path);

        let request = Request::get(url.clone().as_str()).header("Cache-Control", "no-cache").body(Nothing).unwrap();
        log::debug!("Created get request to URI {}", request.uri());
        FetchService::fetch(request, handler.into()).unwrap()
    }
}

fn try_to_parse_levels_json(data: &str) -> Result<LevelsInfo, serde_json::Error> {
    let parsed: LevelsInfo = serde_json::from_str(data)?;
    Ok(parsed)
}
