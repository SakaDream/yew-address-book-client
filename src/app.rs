use yew::{agent::Bridged, Bridge, Component, ComponentLink, Html, html};
use yew_router::prelude::*;
use crate::{pages::{
        dashboard::Dashboard,
        hello_world::HelloWorld,
        page_not_found::PageNotFound,
    }, routes::{AppRoute, fix_fragment_routes}, shared::layout::{Header, Footer}};

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
        fix_fragment_routes(&mut route);
        App {
            link,
            current_route: AppRoute::switch(route),
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(mut route) => {
                fix_fragment_routes(&mut route);
                self.current_route = AppRoute::switch(route);
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
                <Header current_route={&self.current_route}/>
                <main class="flex-shrink-0 mt-3">
                    <div class="container-fluid">
                        {
                            if let Some(route) = &self.current_route {
                                log::info!("{:#?}", self.current_route);
                                match route {
                                    AppRoute::Index => html!{<HelloWorld />},
                                    AppRoute::Home => html!{<HelloWorld />},
                                    AppRoute::HelloWorld => html!{<HelloWorld />},
                                    AppRoute::Dashboard => html!{<Dashboard />},
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