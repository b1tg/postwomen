extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

extern crate subprocess;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use std::io::{self, Write};
use std::env;
use hyper::{Client, Body, Method, Request};
use hyper::rt::{self, Future, Stream};
fn main() {
    pretty_env_logger::init();

    let url = match env::args().nth(2) {
        Some(url) => url,
        None => {
            println!("Usage: client <option> <url>");
            return;
        }
    };

    let method = match env::args().nth(1) {
        Some(m) => m,
        None => {
            println!("Usage: client <option> <url>");
            return;
        }
    };
    let url = url.parse::<hyper::Uri>().unwrap();

    if method == "post" {
        rt::run(post_url(url));
    } else {
        rt::run(fetch_url(url));
    }
    
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, Body>(https);

    client.get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}",e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}",err);
        })

}

fn post_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, Body>(https);

    subprocess::Exec::cmd("vim").arg("#temp.json").join().unwrap();

    let path = Path::new("#temp.json");
    let mut file = File::open(&path).unwrap();
    let mut json_s = String::new();
    file.read_to_string(&mut json_s).unwrap();   
    //let json = r#"{"library":"hyper"}"#;

    let mut req = Request::new(Body::from(json_s));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = url.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json")
    );
    
    client.request(req)
        .and_then(|res| {
            println!("Post: {}", res.status());
            res.into_body().for_each(|chunk|{
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expect stdout is open. err: {}",e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

 

