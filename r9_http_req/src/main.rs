use std::error::Error;

fn main() {
    
    let _ok = http_func();
    // println!("{:#?}", ok);

}

fn http_func() -> Result<(), Box<dyn Error>> {
    
    let resp = reqwest::blocking::get("http://iot.pranisheba.com.bd/api/v1/devices/c_time")?.text()?;
    println!("{:#?}", resp);

    // if resp.status().is_success() {
    //     println!("success!");
    // } else if resp.status().is_server_error() {
    //  println!("server error!");
    // } else {
    //  println!("Something else happened. Status: {:?}", resp.status());
    // }
    Ok(())

}


// fn main() {
//     let resp = reqwest::blocking::get("http://iot.pranisheba.com.bd/api/v1/devices/c_time")?.text()?;
//     println!("{:#?}", resp);
//     Ok(())


//     // let client = reqwest::Client::new();
//     }
    