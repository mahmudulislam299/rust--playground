use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;


use super::super::molecules::custom_form::CustomForm;
use crate::components::molecules::custom_form::Data;
// use gloo::console::log;
use std::ops::Deref;
// use stylist::yew::styled_component;

// use yew::ContextProvider;



#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[function_component(Home)]
pub fn home() -> Html {
  let user_state = use_state(User::default);

  let custom_form_submit = {
    let user_state = user_state.clone();
    Callback::from(move |data: Data| {
        let mut user = user_state.deref().clone();
        user.username = data.username;
        user.favorite_language = data.favorite_language;
        user_state.set(user);
    })
  };

    html! {
      <div>
        <h1>{"Home"}</h1>
        <CustomForm onsubmit={custom_form_submit} />
        <div>
          <Link<Route> to={Route::Hello}>{ "To Hello" }</Link<Route>>
        </div>
      </div>
    }
}
