use std::ops::Deref;

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::shared::model::{ResponseBody, UserDTO};

async fn process_signup(user_dto: &UserDTO) -> Result<ResponseBody<String>, ResponseBody<String>> {
    log::info!("process_signup");
    log::info!("{:?}", user_dto);
    let resp = Request::post("/api/auth/signup")
        .json(user_dto)
        .expect("Invalid request data!")
        .send()
        .await
        .unwrap();
    log::info!("{:?}", resp);

    if resp.ok() {
        return Ok(resp
            .json::<ResponseBody<String>>()
            .await
            .map_err(|e| ResponseBody::new(&format!("{:?}", e).to_string(), "".to_string())))?;
    }
    return Err(resp.json::<ResponseBody<String>>().await.unwrap());
}

#[function_component(Signup)]
pub fn signup() -> Html {
    let user_state = use_state(UserDTO::default);
    let state = {
        let user_state = user_state.clone();
        use_async(async move { process_signup(&(*user_state)).await })
    };

    let email_changed = {
        let user_state = user_state.clone();
        Callback::from(move |event: Event| {
            let mut user_dto = user_state.deref().clone();
            let email = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            user_dto.email = email;
            user_state.set(user_dto);
        })
    };

    let username_changed = {
        let user_state = user_state.clone();
        Callback::from(move |event: Event| {
            let mut user_dto = user_state.deref().clone();
            let username = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            user_dto.username = username;
            user_state.set(user_dto);
        })
    };

    let password_changed = {
        let user_state = user_state.clone();
        Callback::from(move |event: Event| {
            let mut user_dto = user_state.deref().clone();
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            user_dto.password = password;
            user_state.set(user_dto);
        })
    };

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.default_prevented();
            state.run();
        })
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
                    html! { <p>{ format!("Response: {}", data) }</p> }
                } else if let Some(error) = &state.error {
                    html! { <p>{ format!("Error: {}", error.message) }</p> }
                } else {
                    html! {
                        <div class="row d-flex align-items-center justify-content-center">
                            <div class="col-4">
                                <form {onsubmit}>
                                    <div class="form-group">
                                        <h2>{ "Please Sign Up" }</h2>
                                    </div>
                                    <div class="form-group">
                                        <label for="email" class="col-form-label">{ "Email" }</label>
                                        <input type="text" class="form-control" id="email" onchange={email_changed} placeholder="Email"/>
                                    </div>
                                    <div class="form-group">
                                        <label for="username" class="col-form-label">{ "Username" }</label>
                                        <input type="text" class="form-control" id="username" onchange={username_changed} placeholder="Username"/>
                                    </div>
                                    <div class="form-group">
                                        <label for="password" class="col-form-label">{ "Password" }</label>
                                        <input type="password" class="form-control" id="password" onchange={password_changed} placeholder="Password"/>
                                    </div>
                                    <div class="form-group mt-3">
                                        <button type="submit" class="btn btn-primary form-control" disabled={state.loading}>{ "Sign Up" }</button>
                                    </div>
                                    <div class="text-center mt-3">
                                      <span>{ "Already have an account? " }</span>
                                      <a href="/login">{ "Please sign in!" }</a>
                                    </div>
                                </form>
                            </div>
                        </div>
                    }
                }
            }
        </div>
    }
}
