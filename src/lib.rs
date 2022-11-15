use anyhow::Result;
use anyhow::Context;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use serde::{Deserialize,Serialize};
//use serde_json::json;

/// Send an HTTP request and return the response.
#[http_component]
fn send_request(_req: Request) -> Result<Response> {

    let mut _res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
        .method("GET")
        .uri("https://some-random-api.ml/facts/dog")
        .body(None)
        .unwrap(),

        )?;

    let req2 = spin_sdk::outbound_http::send_request(
        http::Request::builder()
        .method("GET")
        .uri("https://some-random-api.ml/facts/cat")
        .body(None)
        .unwrap(),

        )?;

    let req3 = spin_sdk::outbound_http::send_request(
        http::Request::builder()
        .method("GET")
        .uri("https://some-random-api.ml/facts/bird")
        .body(None)
        .unwrap(),

        )?;

    #[derive(Deserialize, Serialize)]
    struct Data {
        fact: String,
    }

    #[derive(Deserialize, Serialize)]
    struct Fdata {
        f1: String,
        f2: String,
        f3: String,

    }


    //let body = _res.body().clone().unwrap_or_default();
    //println!("{:?}",body);
    let d: Data = serde_json::from_slice(&_res.body().clone().unwrap_or_default())?;
    let f: Data = serde_json::from_slice(&req2.body().clone().unwrap_or_default())?;
    let e: Data = serde_json::from_slice(&req3.body().clone().unwrap_or_default())?;
    let g = Fdata {
    f1: d.fact.clone(),
    f2: f.fact.clone(),
    f3: e.fact.clone(),
    };
    println!("{} {}", g.f1,g.f2);


    //    let c: u32 = serde_json::from_slice(&_res.body().clone().unwrap_or_default())?;
    //println!("{c}");
    //println!("{}",d.fact);
    http::Response::builder()
        .header("Foo", "Bar")
        .header("Content-Type","application/json")
        .status(200)
        //.body(Some(serde_json::to_vec(&g)?.into())).with_context(|| format!("Failed to create response"))
        .body(Some(serde_json::to_vec(&g)?.into())).with_context(|| format!("Failed to create response"))

}
