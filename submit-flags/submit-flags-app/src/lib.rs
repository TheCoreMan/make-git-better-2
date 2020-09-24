// See https://github.com/yewstack/yew/issues/97
#![recursion_limit="256"]
use yew::App;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use std::panic;
use wasm_logger;

mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    // Logging to console - see https://yew.rs/docs/en/more/debugging/
    wasm_logger::init(wasm_logger::Config::default());

    // Stacktrace to console on panics - see https://yew.rs/docs/en/more/debugging/
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Actually start the wasm app
    App::<components::mainpage::MainPage>::new().mount_to_body();
}
