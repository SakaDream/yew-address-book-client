use anyhow::Error;
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::shared::model::{Person, ResponseBody};

pub struct Dashboard {
    link: ComponentLink<Self>,
    has_error: bool,
    fetching: bool,
    response: ResponseBody<Vec<Person>>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<ResponseBody<Vec<Person>>, Error>),
    FetchMock,
    ShowError(Result<ResponseBody<Vec<Person>>, Error>),
}

impl Dashboard {
    pub fn get_people(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<ResponseBody<Vec<Person>>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                log::info!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::ShowError(data)
                }
            },
        );
        let request = Request::get("http://localhost:8000/api/address-book")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }

    pub fn get_mock(&mut self) -> Vec<Person> {
        let person1 = Person {
            id: 1,
            name: "Test".to_string(),
            gender: true,
            age: 20,
            address: "Test".to_string(),
            phone: "0123456789".to_string(),
            email: "test@email.com".to_string(),
        };
        let mut result = Vec::new();
        result.push(person1);

        result
    }
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            has_error: false,
            fetching: false,
            response: ResponseBody::new("", Vec::new()),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                self.has_error = false;
                let task = self.get_people();
                self.ft = Some(task);
                true
            },
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.has_error = false;
                match response.map(|res| res) {
                    Ok(res) => self.response = res,
                    Err(err) => self.response = ResponseBody::new(err.to_string().as_str(), Vec::new())
                };
                true
            },
            Msg::FetchMock => {
                self.fetching = false;
                self.has_error = false;
                self.response = ResponseBody::new("ok", self.get_mock());
                true
            },
            Msg::ShowError(response) => {
                self.fetching = false;
                self.has_error = true;
                match response.map(|res| res) {
                    Ok(res) => self.response = res,
                    Err(err) => self.response = ResponseBody::new(err.to_string().as_str(), Vec::new())
                };
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component

        html! {
            <div class="container">
                {
                    if self.fetching {
                        html! {
                            <div class="d-flex justify-content-center">
                                <div class="spinner-border" role="status">
                                    <span class="sr-only" />
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th scope="col">{ "#" }</th>
                                        <th scope="col">{ "Name" }</th>
                                        <th scope="col">{ "Gender" }</th>
                                        <th scope="col">{ "Age" }</th>
                                        <th scope="col">{ "Address" }</th>
                                        <th scope="col">{ "Phone" }</th>
                                        <th scope="col">{ "Email" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for self.response.data.iter().map(|item|
                                        html! {
                                            <tr>
                                                <th scope="row">{ &item.id }</th>
                                                <td>{ &item.name }</td>
                                                <td>{ &item.age }</td>
                                                <td>{ if item.gender { "Male" } else { "Female" } }</td>
                                                <td>{ &item.address }</td>
                                                <td>{ &item.phone }</td>
                                                <td>{ &item.email }</td>
                                            </tr>
                                        }
                                    ) }
                                </tbody>
                            </table>
                        }
                    }
                }
                <div class="btn-toolbar">
                    <button type="button" class="btn btn-primary m-1" onclick=self.link.callback(|_| Msg::FetchData)>{ "Show data from Rest API" }</button>
                    <button type="button" class="btn btn-secondary m-1" onclick=self.link.callback(|_| Msg::FetchMock)>{ "Show mock" }</button>
                </div>
                {
                    if self.has_error {
                        html! {
                            <p>{ format!("Error: {}", self.response.message) }</p>
                        } 
                    } else { 
                        html!{}
                    }
                }
            </div>
        }
    }
}
