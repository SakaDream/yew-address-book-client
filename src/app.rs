use yew::{Component, ComponentLink, Html, html};

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Address Book" }</h1>
                <p>{ "Hello, world!" }</p>
            </div>
        }
    }
}