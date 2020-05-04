use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"King Tech & Dev"}</h1>
            </>
        }
    }
}
