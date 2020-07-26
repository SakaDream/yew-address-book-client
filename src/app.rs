use yew::{agent::Bridged, Bridge, Component, ComponentLink, Html, html};
use yew_router::prelude::*;
use crate::{
    shared::layout::{Header, Footer},
    pages::{
        hello_world::HelloWorld,
        page_not_found::PageNotFound,
    },
    routes::*,
};

pub struct App {
    #[allow(unused)]
    link: ComponentLink<Self>,
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    UpdateRoute(Route)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::UpdateRoute));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        App {
            link,
            current_route: Some(switch(&mut route)),
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(mut route) => {
                self.current_route = Some(switch(&mut route))
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <main class="flex-shrink-0 mt-3">
                    <div class="container-fluid">
                        {
                            if let Some(route) = &self.current_route {
                                match route {
                                    AppRoute::Index => html!{<HelloWorld />},
                                    AppRoute::HelloWorld => html!{<HelloWorld />},
                                    AppRoute::PageNotFound | _ => html!{<PageNotFound />},
                                }
                            } else {
                                html!{<PageNotFound />}
                            }
                        }
                    </div>
                </main>
                <Footer />
            </>
        }
    }
}