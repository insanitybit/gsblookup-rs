#![allow(dead_code, unused_variables)]
extern crate url;
extern crate hyper;

use hyper::Client;
use url::Url;
use std::io::prelude::*;

pub enum Status {
    Phishing,
    Malware,
    Unwanted,
}

pub struct GSBClient {
    api_key: String,
    name: String,
    app_ver: String,
    api_ver: String
}

impl GSBClient {
    pub fn new(key: String) -> GSBClient {
        GSBClient {
            api_key: key,
            name: "gsbrs".to_string(),
            app_ver: "0.0.1".to_string(),
            api_ver: "3.1".to_string()
        }
    }

    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }

    /// Takes a Url and queries the Google Safebrowsing Lookup API
    /// Returns a Vec of Status enums corresponding to the lists in which
    /// the Url is found. If the Url is not found in any list, the Vec is empty.
    pub fn lookup(&self, url: Url) -> Vec<Status> {
        let query = self.build_url(url);
        let mut statuses : Vec<Status> = Vec::new();
        println!("{}", query);

        let client = Client::new();
        let res = client.get(&query).send();

        match res {
            Ok(mut res) => {
                let msg = {
                    let mut s = String::new();
                    let _ = res.read_to_string(&mut s);
                    let s : Vec<String> = s.split(",").map(|s| s.to_string()).collect();
                    s
                };

                for status in msg {
                    match status.as_ref() {
                        ""          => break,
                        "phishing"  => statuses.push(Status::Phishing),
                        "malware"   => statuses.push(Status::Malware),
                        "unwanted"  => statuses.push(Status::Unwanted),
                        _   => unreachable!()
                    }
                }
            },
            Err(e)  => println!("Request to {} failed with: {}", query, e)
        };
        statuses
    }

    pub fn build_url(&self, url: Url) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();
        let url = format!("{}", url);

        let v = vec![("client", &self.name),
                    ("key", &self.api_key),
                    ("appver", &self.app_ver),
                    ("pver", &self.api_ver),
                    ("url", &url)];

        base.set_query_from_pairs(&v);

        format!("{}",base)
    }

    // pub fn canonicalize(url: Url) -> Url {
    //     unimplemented!()
    // }

}
