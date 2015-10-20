extern crate gsbrs;
extern crate url;

use url::Url;
use gsbrs::{GSBClient, Status};

fn main() {
    let key : String = "API KEY HERE".into();

    let gsb = GSBClient::new(key);

    let url = Url::parse("http://exampleurl.org/").unwrap();

    let statuses = gsb.lookup(url);
    for status in statuses {
        match status {
            Status::Phishing    => println!("Phishing"),
            Status::Malware     => println!("Malware"),
            Status::Unwanted    => println!("Unwanted")
        }
    }
}
