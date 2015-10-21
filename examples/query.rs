extern crate gsbrs;
extern crate url;

use url::Url;
use gsbrs::{GSBClient, Status};

fn main() {
    let key : String = "API KEY HERE".into();

    let gsb = GSBClient::new(key);

    let url = Url::parse("http://exampleurl.org/").unwrap();

    let response = gsb.lookup(&url);

    match response {
        Ok(statuses) =>
        {
            for status in statuses {
                match status {
                    Status::Phishing    => println!("Phishing"),
                    Status::Malware     => println!("Malware"),
                    Status::Unwanted    => println!("Unwanted"),
                    Status::Ok          => println!("Ok")
                }
            }
        },
        Err(e) => println!("{}", e)
    }



    let urls = vec!["https://google.com/", "http://exampleurl.org/"];

    let _ = gsb.lookup_all(urls.into_iter());
}
