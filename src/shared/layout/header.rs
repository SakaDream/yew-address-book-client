use yew::{Component, ComponentLink, Html, html};
use yew_router::prelude::*;
use crate::{
    shared::constraint,
    routes::AppRoute,
};

use super::{Header, HeaderProps};

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header { _link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(route) = self.props.current_route.as_ref() {
            let classes = match(route) {
                AppRoute::Home => ("nav-link active", "nav-link", "nav-link"),
                AppRoute::HelloWorld => ("nav-link", "nav-link active", "nav-link"),
                AppRoute::Dashboard => ("nav-link", "nav-link", "nav-link active"),
                _ => ("nav-link", "nav-link", "nav-link"),
            };

            html! {
                <header>
                    <nav class="navbar navbar-expand navbar-dark bg-dark">
                        <div class="container-fluid">
                            <a class="navbar-brand" href="#">{ constraint::APP_NAME }</a>
                            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                                <span class="navbar-toggler-icon"></span>
                            </button>
                            <div class="collapse navbar-collapse" id="navbarSupportedContent">
                                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                    <li class="nav-item">
                                        <RouterAnchor<AppRoute> classes=classes.0 route=AppRoute::Home> { "Home" } </RouterAnchor<AppRoute>>
                                    </li>
                                    <li class="nav-item">
                                        <RouterAnchor<AppRoute> classes=classes.1 route=AppRoute::HelloWorld> { "Hello World" } </RouterAnchor<AppRoute>>
                                    </li>
                                    <li class="nav-item">
                                        <RouterAnchor<AppRoute> classes=classes.2 route=AppRoute::Dashboard> { "Dashboard" } </RouterAnchor<AppRoute>>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </nav>
                </header>
            }
        } else {
            html! {}
        }
    }
}