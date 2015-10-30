extern crate gsbrs;

use gsbrs::{GSBClient, Status};

fn main() {
    let key: String = "AIzaSyCOZpLyGR3gMKqrb5A9lGSsVKtr7".into();

    let gsb = GSBClient::new(key);
    let statuses = gsb.lookup("https://google.com").unwrap();

    if statuses.is_empty() {
        println!("Ok");
    } else {
        for status in statuses {
            match status {
                Status::Phishing => println!("Phishing"),
                Status::Malware => println!("Malware"),
                Status::Unwanted => println!("Unwanted"),
                // lookup only ever returns the above 3 statuses
                _ => unreachable!(),
            }
        }
    }

    let urls = vec!["https://google.com/", "http://exampleurl.org/"];

    let status_lines = gsb.lookup_all(urls.into_iter()).unwrap();

    if status_lines.is_empty() {
        println!("No matches for any url");
    } else {
        for statuses in status_lines {
            for status in statuses {
                match status {
                    Status::Phishing => println!("Phishing"),
                    Status::Malware => println!("Malware"),
                    Status::Unwanted => println!("Unwanted"),
                    Status::Ok => println!("Ok"),
                }
            }
        }
    }

}
