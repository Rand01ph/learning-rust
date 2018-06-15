extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::rt::{self, lazy, Future, Stream};
use serde_json::{Value, Error};

#[derive(Serialize, Deserialize, Debug)]
struct Regions {
    name: String,
    slug: String,
    sizes: Vec<String>,
    features: Vec<String>,
    available: bool,
}


#[derive(Serialize, Deserialize, Debug)]
struct Results {
    regions: Vec<Regions>
}

fn main() {
    rt::run(lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let request = Request::builder()
            .method("GET")
            .uri("https://api.digitalocean.com/v2/regions")
            .header("Authorization", &*format!("Bearer {}", env::var("TOKEN").unwrap()))
            .body(Body::empty())
            .unwrap();

        client.request(request)
            .and_then(|res| {
                println!("status: {}", res.status());
                res.into_body().concat2()
            })
            .and_then(|body| {
                let s = ::std::str::from_utf8(&body)
                    .expect("digitalocean sends utf-8 JSON");
                let v: Results = serde_json::from_str(&s).unwrap();
                let all_slug: Vec<String> = v.regions.into_iter().map(|x| x.slug).collect();
                println!("all_available_slug is {:?}", all_slug);
                Ok(())
            })
            .map_err(|err| {
                println!("error: {}", err);
            })
    }));
}
