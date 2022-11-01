use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::customer_info::stores::customerinfo_store::CustomerInfoStore;

pub enum Msg {
    Store(Rc<CustomerInfoStore>),
}

pub struct DisplayCustomerInfo {
    dispatch: Dispatch<CustomerInfoStore>,
}

impl Component for DisplayCustomerInfo {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CustomerInfoStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let customerinfo_store = self.dispatch.get();
        let username = format!(
            "Username: {}",
            customerinfo_store.username.as_deref().unwrap_or_default()
        );
        let password = format!(
            "Password: {}",
            customerinfo_store.password.as_deref().unwrap_or_default()
        );
        let is_customerinfoenticated = format!("Is CustomerInfoenticated: {:?}", customerinfo_store.is_customerinfoenticated);


        

        html! {
            <div>
                <h2>{"CustomerInfo Store"}</h2>
                <div>{username}</div>
                <div>{password}</div>
                <div>{is_customerinfoenticated}</div>
            </div>
        }
    }
}
