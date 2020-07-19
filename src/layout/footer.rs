use yew::{Component, ComponentLink, Html, html};
use crate::{
    common::constraint,
    layout::Footer,
};

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Footer { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer mt-auto py-3 bg-light">
                <div class="container">
                    <span class="text-muted">{ constraint::COPYRIGHT }</span>
                </div>
            </footer>
        }
    }
}