use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::{use_async, use_latest};

use crate::shared::model::{Person, ResponseBody};

async fn fetch_data(
    use_mock_data: bool,
) -> Result<ResponseBody<Vec<Person>>, ResponseBody<String>> {
    if use_mock_data {
        Ok(ResponseBody::new(
            "OK",
            vec![Person {
                id: 1,
                name: "Test".to_string(),
                gender: true,
                age: 20,
                address: "Test".to_string(),
                phone: "0123456789".to_string(),
                email: "test@email.com".to_string(),
            }],
        ))
    } else {
        let resp = Request::get("/api/address-book")
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await
            .unwrap();

        if resp.ok() {
            return Ok(resp
                .json::<ResponseBody<Vec<Person>>>()
                .await
                .map_err(|e| ResponseBody::new(&format!("{:?}", e).to_string(), "".to_string())))?;
        }
        return Err(resp.json::<ResponseBody<String>>().await.unwrap());
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let use_mock_data = use_state(|| false);
    let use_mock_data_latest = use_latest(use_mock_data.clone());

    let state = use_async(async move { fetch_data(**use_mock_data_latest.current()).await });

    let on_fetch_api_click = {
        let use_mock_data = use_mock_data.clone();
        let state = state.clone();
        Callback::from(move |_| {
            let use_mock_data = use_mock_data.clone();
            use_mock_data.set(false);
            let state = state.clone();
            state.run();
        })
    };

    let on_fetch_mock_click = {
        let use_mock_data = use_mock_data.clone();
        let state = state.clone();
        Callback::from(move |_| {
            let use_mock_data = use_mock_data.clone();
            use_mock_data.set(true);
            let state = state.clone();
            state.run();
        })
    };

    let table_header = html! {
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
    };

    html! {
        <div class="container">
            {
                if state.loading {
                    html! {
                        <div class="d-flex justify-content-center">
                            <div class="spinner-border" role="status">
                                <span class="sr-only" />
                            </div>
                        </div>
                    }
                } else if let Some(data) = &state.data {
                    html! {
                        <table class="table" disabled={state.loading}>
                            { table_header }
                            <tbody>
                                { for data.data.iter().map(|item|
                                    html! {
                                        <tr>
                                            <th scope="row">{ &item.id }</th>
                                            <td>{ &item.name }</td>
                                            <td>{ if item.gender { "Male" } else { "Female" } }</td>
                                            <td>{ &item.age }</td>
                                            <td>{ &item.address }</td>
                                            <td>{ &item.phone }</td>
                                            <td>{ &item.email }</td>
                                        </tr>
                                    }
                                ) }
                            </tbody>
                        </table>
                    }
                } else if let Some(error) = &state.error {
                    html! { <p>{ format!("Error: {}", error.message) }</p> }
                } else {
                    html! {
                        <table class="table" disabled={state.loading}>
                            { table_header }
                            <tbody></tbody>
                        </table>
                    }
                }
            }
            <div class="btn-toolbar">
                <button type="button" class="btn btn-primary m-1" disabled={state.loading} onclick={on_fetch_api_click}>{ "Show data from Rest API" }</button>
                <button type="button" class="btn btn-primary m-1" disabled={state.loading} onclick={on_fetch_mock_click}>{ "Show data from Mock" }</button>
            </div>
        </div>
    }
}
