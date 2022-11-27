use anyhow::Result;
// use async_std::task;
// mod utils;

// #[tokio::main]
// #[async_std::main]
fn main() -> Result<()> {
    println!("hi from rust wasi module");

    // test accessing the network
    use std::collections::HashMap;
    let rt = tokio::runtime::Builder::new_current_thread().build()?;
    // let resp = rt.block_on(async {
    //     reqwest::get("https://httpbin.org/ip")
    //         .await?
    //         .json::<HashMap<String, String>>()
    //         .await
    // });

    // println!("response: {:#?}", resp);
    // console::debug_1(&JsValue::from(format!("{:#?}", resp)));
    Ok(())
}

// use wasm_bindgen::prelude::*;
// use web_sys::console;

// #[wasm_bindgen(module = "./fetch.js")]
// extern "C" {}

// #[wasm_bindgen(start)]
// pub async fn main_js() -> Result<(), JsError> {
//     console::debug_1(&JsValue::from("im a rust wasm github action!"));

//     use std::collections::HashMap;
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     console::debug_1(&JsValue::from(format!("{:#?}", resp)));

//     for entry in std::env::current_dir()?.read_dir()? {
//         console::debug_1(&JsValue::from(format!("entry: {:#?}", entry?)));
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
