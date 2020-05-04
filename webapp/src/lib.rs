
mod components;
pub use crate::components::header;

use web_sys::Window;
use yew::prelude::*;
use std::error::Error;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 1,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        let window: web_sys::Window = web_sys::window().expect("window not available");
        window.alert_with_message("hello from wasm!").expect("alert failed");
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
                <components::header::Header />
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    alert(&format!("Hello, World!!"));
    yew::initialize();
    App::<Model>::new().mount_to_body();
}
