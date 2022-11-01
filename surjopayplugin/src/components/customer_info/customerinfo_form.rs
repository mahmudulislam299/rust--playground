use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::customer_info::stores::customerinfo_store::CustomerInfoStore;

use stylist::{Style};
const STYLE_FILE: &str = include_str!("customerinfo_form_css.css");

pub enum Msg {
    Store(Rc<CustomerInfoStore>),
    Username(String),
    Password(String),
    Address(String),
    Mobile(String),

    CustomerName(String),
    CustomerAddress(String),
    CustomerPhone(String),
    CustomerCity(String),
    CustomerPostCode(String),
    StoreId(String),
    Amount(String),
    OrderId(String),
    Currency(String),

    Login,
}

pub struct CustomerInfoForm {
    dispatch: Dispatch<CustomerInfoStore>,
}

impl Component for CustomerInfoForm {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CustomerInfoStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::Username(username) => {
                self.dispatch
                    .reduce_mut(|store| store.username = Some(username));
                false
            }
            Msg::Password(password) => {
                self.dispatch
                    .reduce_mut(|store| store.password = Some(password));
                false
            }
            Msg::Address(address) => {
                self.dispatch
                    .reduce_mut(|store| store.address = Some(address));
                false
            }
            Msg::Mobile(mobile) => {
                self.dispatch
                    .reduce_mut(|store| store.mobile = Some(mobile));
                false
            }

            Msg::CustomerName(customer_name) => {
                self.dispatch
                    .reduce_mut(|store| store.customer_name = Some(customer_name));
                false
            }

            Msg::CustomerAddress(customer_address) => {
                self.dispatch
                    .reduce_mut(|store| store.customer_address = Some(customer_address));
                false
            }

            Msg::CustomerPhone(customer_phone) => {
                self.dispatch
                    .reduce_mut(|store| store.customer_phone = Some(customer_phone));
                false
            }

            Msg::CustomerCity(customer_city) => {
                self.dispatch
                    .reduce_mut(|store| store.customer_city = Some(customer_city));
                false
            }

            Msg::CustomerPostCode(customer_post_code) => {
                self.dispatch
                    .reduce_mut(|store| store.customer_post_code = Some(customer_post_code));
                false
            }

            Msg::StoreId(store_id) => {
                self.dispatch
                    .reduce_mut(|store| store.store_id = Some(store_id));
                false
            }

            Msg::Amount(amount) => {
                self.dispatch
                    .reduce_mut(|store| store.amount = Some(amount));
                false
            }

            Msg::OrderId(order_id) => {
                self.dispatch
                    .reduce_mut(|store| store.order_id = Some(order_id));
                false
            }

            Msg::Currency(currency) => {
                self.dispatch
                    .reduce_mut(|store| store.currency = Some(currency));
                false
            }

            Msg::Login => {
                self.dispatch.reduce_mut(|store| {
                    store.is_customerinfoenticated = store.password.is_some() && store.username.is_some();
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::Login
        });

        let username_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let username = target.value();
            Msg::Username(username)
        });

        let password_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let password = target.value();
            Msg::Password(password)
        });

        let address_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let address = target.value();
            Msg::Address(address)
        });


        let mobile_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let mobile = target.value();
            Msg::Mobile(mobile)
        });


        let customer_name_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let customer_name = target.value();
            Msg::CustomerName(customer_name)
        });

        let customer_address_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let customer_address = target.value();
            Msg::CustomerAddress(customer_address)
        });

        let customer_phone_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let customer_phone = target.value();
            Msg::CustomerPhone(customer_phone)
        });

        let customer_city_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let customer_city = target.value();
            Msg::CustomerCity(customer_city)
        });

        let customer_post_code_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let customer_post_code = target.value();
            Msg::CustomerPostCode(customer_post_code)
        });

        let store_id_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let store_id = target.value();
            Msg::StoreId(store_id)
        });
        
        let amount_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let amount = target.value();
            Msg::Amount(amount)
        });

        let order_id_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let order_id = target.value();
            Msg::OrderId(order_id)
        });

        let currency_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let currency = target.value();
            Msg::Currency(currency)
        });
        
        let stylesheet = Style::new(STYLE_FILE).unwrap();

        html! {
            <form {onsubmit}>
                <h2>{"Login"}</h2>
                <h3>{"test"}</h3>
                <table>
                    <tr>
                        <td>
                            <label for="username">{"Username"}</label>
                        </td>
                        <td>
                            <input type="text" id="username" placeholder="username" onchange={username_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="password">{"Password"}</label>
                        </td>
                        <td>
                            <input type="password" id="password" placeholder="password" onchange={password_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="address">{"Address"}</label>
                        </td>
                        <td>
                            <input type="address" id="address" placeholder="address" onchange={address_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                        <label for="mobile">{"Mobile"}</label>
                        </td>
                        <td>
                        <input type="text" id="mobile" placeholder="mobile" onchange={mobile_onchange} />  
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="customer_name">{"CustomerName"}</label>
                        </td>
                        <td>
                            <input type="text" id="customer_name" placeholder="customer_name" onchange={customer_name_onchange} />  
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="customer_address">{"CustomerAddress"}</label>
                        </td>
                        <td>
                            <input type="text" id="customer_address" placeholder="customer_address" onchange={customer_address_onchange} />   
                        </td>
                    </tr>
                    <tr>
                        <td>
                        <label for="customer_phone">{"CustomerPhone"}</label>
                        </td>
                        <td>
                        <input type="text" id="customer_phone" placeholder="customer_phone" onchange={customer_phone_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="customer_city">{"CustomerCity"}</label>
                        
                        </td>
                        <td>
                            <input type="text" id="customer_city" placeholder="customer_city" onchange={customer_city_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="customer_post_code">{"CustomerPostCode"}</label>                        
                        </td>
                        <td>
                        <input type="text" id="customer_post_code" placeholder="customer_post_code" onchange={customer_post_code_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                        <label for="store_id">{"StoreId"}</label>
                       
                        </td>
                        <td>
                        <input type="text" id="store_id" placeholder="store_id" onchange={store_id_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                        <label for="amount">{"Amount"}</label>
                        
                        </td>
                        <td>
                        <input type="text" id="amount" placeholder="amount" onchange={amount_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="order_id">{"OrderId"}</label>
                        
                        </td>
                        <td>
                        <   input type="text" id="order_id" placeholder="order_id" onchange={order_id_onchange} />
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <label for="currency">{"Currency"}</label>
                        
                        </td>
                        <td>
                            <input type="text" id="currency" placeholder="currency" onchange={currency_onchange} />
                        </td>
                    
                    </tr>
                    
                    <tr>
                    <td>
                    </td>
                    </tr>
                
                </table>
    
                
                <div>
                    <button>{"Login"}</button>
                </div>
            </form>
        }
    }
}
