use yew::{Component, ComponentLink, Html, html};
use crate::{
    common::constraint,
    layout::Header,
};

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <nav class="navbar navbar-expand navbar-dark bg-dark">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="#">{ constraint::APP_NAME }</a>
                    </div>
                </nav>
            </header>
        }
    }
}