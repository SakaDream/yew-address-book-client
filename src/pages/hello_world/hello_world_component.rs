use yew::{Component, ComponentLink, Html, html};
use crate::pages::hello_world::HelloWorld;

impl Component for HelloWorld {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        HelloWorld {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
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