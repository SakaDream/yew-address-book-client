use std::ops::Deref;

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::use_async;

use crate::shared::model::{LoginDTO, ResponseBody, TokenBodyResponse};

async fn process_login(
    login_dto: &LoginDTO,
) -> Result<ResponseBody<TokenBodyResponse>, ResponseBody<String>> {
    let resp = Request::post("/api/auth/login")
        .json(login_dto)
        .expect("Invalid request data!")
        .send()
        .await
        .unwrap();

    if resp.ok() {
        return Ok(resp
            .json::<ResponseBody<TokenBodyResponse>>()
            .await
            .map_err(|e| ResponseBody::new(&format!("{:?}", e).to_string(), "".to_string())))?;
    }
    return Err(resp.json::<ResponseBody<String>>().await.unwrap());
}

#[function_component(Login)]
pub fn login() -> Html {
    let login_state = use_state(LoginDTO::default);
    let state = {
        let login_state = login_state.clone();
        use_async(async move {
            process_login(&(*login_state)).await
        })
    };

    let username_or_email_changed = {
        let login_state = login_state.clone();
        Callback::from(move |event: Event| {
            let mut login_dto = login_state.deref().clone();
            let username_or_email = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            login_dto.username_or_email = username_or_email;
            login_state.set(login_dto);
        })
    };

    let password_changed = {
        let login_state = login_state.clone();
        Callback::from(move |event: Event| {
            let mut login_dto = login_state.deref().clone();
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            login_dto.password = password;
            login_state.set(login_dto);
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
                    html!{
                        <div class="row d-flex align-items-center justify-content-center">
                            <div class="col-4">
                                <form {onsubmit}>
                                    <div class="form-group">
                                        <h2>{ "Please Sign In" }</h2>
                                    </div>
                                    <div class="form-group">
                                        <label for="username-or-email" class="col-form-label">{ "Username or Email" }</label>
                                        <input type="text" class="form-control" id="username-or-email" onchange={username_or_email_changed} placeholder="Username or Email" required=true/>
                                    </div>
                                    <div class="form-group">
                                        <label for="password" class="col-form-label">{ "Password" }</label>
                                        <input type="password" class="form-control" id="password" onchange={password_changed}  placeholder="Password" required=true/>
                                    </div>
                                    <div class="form-group mt-3">
                                        <button type="submit" class="btn btn-primary form-control" disabled={state.loading}>{ "Sign In" }</button>
                                    </div>
                                    <div class="text-center mt-3">
                                      <span>{ "Do not have account? " }</span>
                                      <a href="/signup">{ "Create new one!" }</a>
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
