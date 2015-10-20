#![allow(dead_code)]
extern crate url;
extern crate hyper;

use hyper::Client;
use url::Url;
use std::io::prelude::*;

pub enum Status {
    Ok,
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
    pub fn lookup(&self, url: &Url) -> Vec<Status> {
        let query = self.build_get_url(url.clone());
        let mut statuses : Vec<Status> = Vec::new();
        println!("{}", query);

        let client = Client::new();
        let res = client.get(&query).send();

        match res {
            Ok(mut res) => {
                let msg = {
                    let mut s = String::new();
                    let _ = res.read_to_string(&mut s);
                    s
                };

                let msg : Vec<&str> = msg.split(",").collect();

                for status in msg {
                    match status {
                        "phishing"  => statuses.push(Status::Phishing),
                        "malware"   => statuses.push(Status::Malware),
                        "unwanted"  => statuses.push(Status::Unwanted),
                        ""          => (),
                        _   => unreachable!()
                    }
                }
            },
            Err(e)  => println!("Request to {} failed with: {}", query, e)
        };
        statuses
    }

    fn build_get_url(&self, url: Url) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();
        let url = format!("{}", url);

        let v : Vec<(&str, &str)> =
                vec![("client", self.name.as_ref()),
                    ("key", self.api_key.as_ref()),
                    ("appver", self.app_ver.as_ref()),
                    ("pver", self.api_ver.as_ref()),
                    ("url", url.as_ref())];

        base.set_query_from_pairs(v.into_iter());

        format!("{}",base)
    }

    pub fn lookup_all<'a, I> (&self, urls: I) -> Vec<Vec<Status>>
    where I: Iterator<Item=&'a str>
    {
        let url = self.build_post_url();
        let mut all_statuses = Vec::new();

        let message = {
            let mut furls = String::new();
            let mut size : usize = 0;

            for url in urls {
                size = size + 1;
                furls.push_str(url);
                furls.push('\n');
            }
            if size > 200 {
                panic!("Can not lookup more than 200 urls");
            }
            let size = size.to_string();
            let mut message = String::with_capacity(furls.len() + size.len());

            message.push_str(&size);
            message.push_str(&furls);
            message
        };

        let client = Client::new();

        let res = client.post(&url)
            .body(&message)
            .send();

            match res {
                Ok(mut res) => {
                    let msg = {
                        let mut s = String::new();
                        let _ = res.read_to_string(&mut s);
                        s
                    };

                    let msg : Vec<&str> = msg.split("\n").collect();

                    for status_line in msg {
                        let mut statuses = Vec::new();
                        let status_line : Vec<&str> = status_line.split(",").collect();

                        for status in status_line {
                            match status {
                                "ok"        => statuses.push(Status::Ok),
                                "phishing"  => statuses.push(Status::Phishing),
                                "malware"   => statuses.push(Status::Malware),
                                "unwanted"  => statuses.push(Status::Unwanted),
                                ""  => (),
                                _   => unreachable!()
                            }
                        }
                        all_statuses.push(statuses);
                    }
                },
                Err(e)  => println!("Request failed with: {}", e)
            };


        all_statuses
    }

    fn build_post_url(&self) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();

        let v : Vec<(&str, &str)> =
                vec![("client", self.name.as_ref()),
                    ("key", self.api_key.as_ref()),
                    ("appver", self.app_ver.as_ref()),
                    ("pver", self.api_ver.as_ref())];

        base.set_query_from_pairs(v.into_iter());

        format!("{}",base)
    }

    // pub fn canonicalize(url: Url) -> Url {
    //     unimplemented!()
    // }

}
