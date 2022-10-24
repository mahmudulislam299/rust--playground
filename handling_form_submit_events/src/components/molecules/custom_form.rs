use std::clone;
use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub customer_name: String,
    pub customer_address: String,
    pub customer_phone: String,
    pub customer_city: String,
    pub cutomer_post_code: String,
    pub store_id: String,
    pub amount: String,
    pub oerder_id: String,
    pub currency: String,
    pub client_ip: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());

    // username
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    // customer name
    let cloned_state = state.clone();
    let customer_name_changed = Callback::from(move |customer_name| {
        let mut data = cloned_state.deref().clone();
        data.customer_name = customer_name;
        cloned_state.set(data);
    });

    // customer address
    let cloned_state = state.clone();
    let customer_address_changed = Callback::from(move |customer_address| {
        let mut data = cloned_state.deref().clone();
        data.customer_address = customer_address;
        cloned_state.set(data);
    });

    
    // customer_phone
    let cloned_state = state.clone();
    let customer_phone_changed = Callback::from(move |customer_phone| {
        let mut data = cloned_state.deref().clone();
        data.customer_phone = customer_phone;
        cloned_state.set(data);
    });
    
    // customer_city
    let cloned_state = state.clone();
    let customer_city_changed = Callback::from(move |customer_city| {
        let mut data = cloned_state.deref().clone();
        data.customer_city = customer_city;
        cloned_state.set(data);
    });

    // cutomer_post_code
    let cloned_state = state.clone();
    let cutomer_post_code_changed = Callback::from(move |cutomer_post_code| {
        let mut data = cloned_state.deref().clone();
        data.cutomer_post_code = cutomer_post_code;
        cloned_state.set(data);
    });
    
    // store_id
    let cloned_state = state.clone();
    let store_id_changed = Callback::from(move |store_id| {
        let mut data = cloned_state.deref().clone();
        data.store_id = store_id;
        cloned_state.set(data);
    });

    // amount
    let cloned_state = state.clone();
    let amount_changed = Callback::from(move |amount| {
        let mut data = cloned_state.deref().clone();
        data.amount = amount;
        cloned_state.set(data);
    });
    
    // oerder_id
    let cloned_state = state.clone();
    let oerder_id_changed = Callback::from(move |oerder_id| {
        let mut data = cloned_state.deref().clone();
        data.oerder_id = oerder_id;
        cloned_state.set(data);
    });

    // currency
    let cloned_state = state.clone();
    let currency_changed = Callback::from(move |currency| {
        let mut data = cloned_state.deref().clone();
        data.currency = currency;
        cloned_state.set(data);
    });
    
    // client_ip
    let cloned_state = state.clone();
    let client_ip_changed = Callback::from(move |client_ip| {
        let mut data = cloned_state.deref().clone();
        data.client_ip = client_ip;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
      <form onsubmit={onsubmit}>
        <label>{"Username:  "}</label> 
        <TextInput name="username" handle_onchange={username_changed} /> <br/>
        <label>{"customer_name:  "}</label> 
        <TextInput name="customer_name" handle_onchange={customer_name_changed} /><br/>
        <label>{"customer_address:  "}</label> 
        <TextInput name="customer_address" handle_onchange={customer_address_changed} /><br/>
        <label>{"customer_phone:  "}</label> 
        <TextInput name="customer_phone" handle_onchange={customer_phone_changed} /><br/>
        <label>{"customer_city:  "}</label> 
        <TextInput name="customer_city" handle_onchange={customer_city_changed} /><br/>
        <label>{"cutomer_post_code:  "}</label> 
        <TextInput name="cutomer_post_code" handle_onchange={cutomer_post_code_changed} /><br/>
        <label>{"store_id:  "}</label> 
        <TextInput name="store_id" handle_onchange={store_id_changed} /><br/>
        <label>{"amount:  "}</label> 
        <TextInput name="amount" handle_onchange={amount_changed} /><br/>
        <label>{"oerder_id:  "}</label> 
        <TextInput name="oerder_id" handle_onchange={oerder_id_changed} /><br/>
        <label>{"currency:  "}</label> 
        <TextInput name="currency" handle_onchange={currency_changed} /><br/>
        <label>{"client_ip:  "}</label> 
        <TextInput name="client_ip" handle_onchange={client_ip_changed} /><br/>
        
        <CustomButton label="Submit" />
      </form>
    }
}
