#[macro_use]
use crate::utils::*;

use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DataFetched,
}

pub enum Msg {
    Updating,
}

pub struct Worker {
    link: AgentLink<Worker>,
    _task: Box<dyn Task>,
}

impl Agent for Worker {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        console_log!("created context");
        let duration = Duration::from_secs(3);
        let callback = link.callback(|_| Msg::Updating);
        let task = IntervalService::new().spawn(duration, callback);
        Worker {
            link,
            _task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                console_log!("Tick...context");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        console_log!("context Request: {:?} {:?}", msg, who);
        match msg {
            Request::GetDataFromServer => {
                // TODO fetch actual data
                self.link.respond(who, Response::DataFetched);
            }
        }
    }
}
