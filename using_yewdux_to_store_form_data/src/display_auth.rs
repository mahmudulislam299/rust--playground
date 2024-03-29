use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::ustomerInfo_store::AuthStore;

pub enum Msg {
    Store(Rc<AuthStore>),
}

pub struct DisplayAuth {
    dispatch: Dispatch<AuthStore>,
}

impl Component for DisplayAuth {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let ustomerInfo_store = self.dispatch.get();
        let username = format!(
            "Username: {}",
            ustomerInfo_store.username.as_deref().unwrap_or_default()
        );
        let password = format!(
            "Password: {}",
            ustomerInfo_store.password.as_deref().unwrap_or_default()
        );
        let is_ustomerInfoenticated = format!("Is Authenticated: {:?}", ustomerInfo_store.is_ustomerInfoenticated);

        html! {
            <div>
                <h2>{"Auth Store"}</h2>
                <div>{username}</div>
                <div>{password}</div>
                <div>{is_ustomerInfoenticated}</div>
            </div>
        }
    }
}
