extern crate hyper;
extern crate hyper_tls;

use std::env;
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::rt::{self, lazy, Future, Stream};

fn main() {
    rt::run(lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let token = "Bearer ".to_owned() + &env::var("token").unwrap().to_owned();

        let request = Request::builder()
            .method("GET")
            .uri("https://api.digitalocean.com/v2/regions")
            .header("Authorization", token)
            .body(Body::empty())
            .unwrap();

        client.request(request)
            // And then, if the request gets a response...
            .and_then(|res| {
                println!("status: {}", res.status());

                // Concatenate the body stream into a single buffer...
                // This returns a new future, since we must stream body.
                res.into_body().concat2()
            })

            // And then, if reading the full body succeeds...
            .and_then(|body| {
                // The body is just bytes, but let's print a string...
                let s = ::std::str::from_utf8(&body)
                    .expect("httpbin sends utf-8 JSON");

                println!("body: {}", s);

                // and_then requires we return a new Future, and it turns
                // out that Result is a Future that is ready immediately.
                Ok(())
            })

            // Map any errors that might have happened...
            .map_err(|err| {
                println!("error: {}", err);
            })
    }));
}
