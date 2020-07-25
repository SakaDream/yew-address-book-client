use yew::{agent::Bridged, Bridge, Component, ComponentLink, Html, html};
use yew_router::prelude::*;
use crate::{
    shared::layout::{Header, Footer},
    pages::hello_world::HelloWorld,
};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "#/hello-world"]
    HelloWorld,
    #[to = "#/page-not-found"]
    PageNotFound,
    #[to = "#/"]
    Index,
}

/// Fix fragment handling problem for yew_router
fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/".to_string();
    }
}

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    link: ComponentLink<Self>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        fix_fragment_routes(&mut route);

        let current_path = route.route.clone();
        
        let mut current_route = AppRoute::switch(route);
        
        // Fix yew router auto redirect to home instead of PageNotFound
        if "#/" != current_path && current_route == Some(AppRoute::Index) {
            current_route = Some(AppRoute::PageNotFound);
        }
        
        App {
            current_route,
            link,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Route(mut route) => {
                fix_fragment_routes(&mut route);
                // Fix yew router auto redirect to home instead of PageNotFound
                if "#/" != &route.route && self.current_route == Some(AppRoute::Index) {
                    self.current_route = Some(AppRoute::PageNotFound);
                }
                self.current_route = AppRoute::switch(route)
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
                                    _ => html!{<h1>{ "404 Not Found" }</h1>},
                                }
                            } else {
                                html!{<h1>{ "404 Not Found" }</h1>}
                            }
                        }
                    </div>
                </main>
                <Footer />
            </>
        }
    }
}