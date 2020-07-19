use yew::{
    Component, ComponentLink, html,
    virtual_dom::VNode,
};
use yew_router::{
    prelude::*,
    Switch,
    switch::Permissive,
};
use crate::layout::{Header, Footer};
use crate::components::{
    hello_world::HelloWorld,
};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    HelloWorld,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
            <>
                <Header />
                <main class="flex-shrink-0 mt-3">
                    <div class="container-fluid">
                        <Router<AppRoute>
                            render = Router::render(|switch: AppRoute| {
                                match switch {
                                    AppRoute::HelloWorld => html!{<HelloWorld />},
                                    AppRoute::PageNotFound(Permissive(None)) => html!{<h1>{ "404 Not Found" }</h1>},
                                    AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{<h1>{ "404 Not Found" }</h1>},
                                    _ => html!{<h1>{ "404 Not Found" }</h1>},
                                }
                            })
                        />
                    </div>
                </main>
                <Footer />
            </>
        }
    }
}