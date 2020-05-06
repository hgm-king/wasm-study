#[macro_use]
mod utils;
use crate::utils::*;

mod components;
pub use crate::components::{header::Header as Head};

use web_sys::Window;
use yew::prelude::*;
use std::error::Error;

pub mod context;
pub mod job;
pub mod native_worker;

use yew::agent::Threaded;

use yew::worker::{Bridge, Bridged};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use wasm_bindgen::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    worker: Box<dyn Bridge<native_worker::Worker>>,
    job: Box<dyn Bridge<job::Worker>>,
    context: Box<dyn Bridge<context::Worker>>,
    context_2: Box<dyn Bridge<context::Worker>>,
}

enum Msg {
    SendToWorker,
    SendToJob,
    SendToContext,
    DataReceived,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::DataReceived);
        let worker = native_worker::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Self {
            link,
            worker,
            job,
            context,
            context_2,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWorker => {
                console_log!("Sending to worker!");
                self.worker.send(native_worker::Request::GetDataFromServer);
            }
            Msg::SendToJob => {
                console_log!("Sending to worker!");
                self.job.send(job::Request::GetDataFromServer);
            }
            Msg::SendToContext => {
                console_log!("Sending to worker!");
                self.context.send(context::Request::GetDataFromServer);
                self.context_2.send(context::Request::GetDataFromServer);
            }
            Msg::DataReceived => {
                console_log!("DataReceived");
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
        html! {
            <div class={"main"}>
                <Head />
                <div>
                    <nav class="menu">
                        <button onclick=self.link.callback(|_| Msg::SendToWorker)>{ "Send to Thread" }</button>
                        <button onclick=self.link.callback(|_| Msg::SendToJob)>{ "Send to Job" }</button>
                        <button onclick=self.link.callback(|_| Msg::SendToContext)>{ "Send to Context" }</button>
                    </nav>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();

    native_worker::Worker::register();

    console_log!("Hello, World");
    // yew::initialize();

}
