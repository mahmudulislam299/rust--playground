use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

use crate::components::customer_info::customerinfo_form::CustomerInfoForm;
use crate::components::customer_info::display_customerinfo::DisplayCustomerInfo;

#[function_component(Showforminfopage)]
pub fn showforminfopage() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Home);
    });

    html! {
      <div>
        <h1>{"Showforminfopage"}</h1>
        <button {onclick}>{"Go Home"}</button>
				<DisplayCustomerInfo />
      </div>
    }
}
