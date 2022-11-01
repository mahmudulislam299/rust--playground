use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct CustomerInfoStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
    pub mobile: Option<String>,

    pub customer_name: Option<String>,
    pub customer_address: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_city: Option<String>,
    pub customer_post_code: Option<String>,
    pub store_id: Option<String>,
    pub amount: Option<String>,
    pub order_id: Option<String>,
    pub currency: Option<String>,
    
    pub is_customerinfoenticated: bool,
}
