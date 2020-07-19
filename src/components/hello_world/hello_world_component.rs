use yew::{Component, ComponentLink, Html, html};
use crate::components::hello_world::HelloWorld;

impl Component for HelloWorld {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HelloWorld { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Hello World!" }</h1>
            </>
        }
    }
}