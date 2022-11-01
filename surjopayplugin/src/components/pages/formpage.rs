use yew::prelude::*;
use yew_router::prelude::*;



use crate::router::Route;

use crate::components::customer_info::customerinfo_form::CustomerInfoForm;
use crate::components::customer_info::display_customerinfo::DisplayCustomerInfo;

#[function_component(Formpage)]
pub fn formpage() -> Html {
    let history = use_history().unwrap();
    let onclick1 = Callback::from(move |_| {
        history.push(Route::Home);
    });

    let history = use_history().unwrap();
    let onclick2 = Callback::from(move |_| {
        history.push(Route::Showforminfopage);
    });

    html! {
      <div>
        <h1>{"formpage"}</h1>
        <button onclick = {onclick1}>{"Go Home"}</button>
        <button onclick = {onclick2}>{"show from info page"}</button>
				<CustomerInfoForm />
				<DisplayCustomerInfo />
      </div>
    }
}
