use std::ops::Deref;

use gloo_net::http::Request;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::use_async;

use crate::shared::model::{LoginDTO, ResponseBody, TokenBodyResponse};

async fn request_login(
    username: String,
    password: String,
) -> Result<ResponseBody<TokenBodyResponse>, ResponseBody<String>> {
    let resp = Request::post("/server/login")
        .json(&LoginDTO {
            username_or_email: username,
            password: password,
        })
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
    let username_or_email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let username_or_email_state = use_state(|| String::from(""));
    let password_state = use_state(|| String::from(""));

    let username_or_email = username_or_email_state.deref().clone();
    let password = password_state.deref().clone();

    let state = use_async(async move { request_login(username_or_email, password).await });

    let submit_login = {
        let username_or_email_ref = username_or_email_ref.clone();
        let password_ref = password_ref.clone();
        let username_or_email_state = username_or_email_state.clone();
        let password_state = password_state.clone();
        let state = state.clone();
        move |_| {
            if let Some(input) = username_or_email_ref.cast::<HtmlInputElement>() {
                username_or_email_state.set(input.value());
            }
            if let Some(input) = password_ref.cast::<HtmlInputElement>() {
                password_state.set(input.value());
            }
            let state = state.clone();
            state.run();
        }
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
                    html! { <p>{ format!("Login Response: {}", data) }</p> }
                } else if let Some(error) = &state.error {
                    html! { <p>{ format!("Error: {}", error.message) }</p> }
                } else {
                    html!{
                        <div class="row d-flex align-items-center justify-content-center">
                            <div class="col-4">
                                <form action="">
                                    <div class="form-group">
                                        <h2>{ "Please Sign In" }</h2>
                                    </div>
                                    <div class="form-group">
                                        <label for="username-or-email" class="col-form-label">{ "Username or Email" }</label>
                                        <input ref={username_or_email_ref} type="text" class="form-control" id="username-or-email" placeholder="Username or Email" required=true/>
                                    </div>
                                    <div class="form-group">
                                        <label for="password" class="col-form-label">{ "Password" }</label>
                                        <input ref={password_ref} type="password" class="form-control" id="password" placeholder="Password" required=true/>
                                    </div>
                                    <div class="form-group mt-3">
                                        <button type="submit" class="btn btn-primary form-control" disabled={state.loading} onclick={submit_login}>{ "Sign In" }</button>
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
