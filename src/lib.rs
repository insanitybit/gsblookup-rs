#![feature(custom_derive)]
#![allow(dead_code, unused_attributes)]
extern crate url;
extern crate hyper;

pub mod gsberror;


use gsberror::GSBError;
use hyper::Client;
use url::Url;
use std::io::prelude::*;

/// Status represents each list a URL may be found in as well as a value,
/// 'Ok', which is used as a placeholder when the URL is not found in any
/// list. 'Ok' is only used in bulk queries.
#[derive(Display, Debug)]
pub enum Status {
    Ok,
    Phishing,
    Malware,
    Unwanted,
}

/// A client for interacting with the Google Safe Browsing Lookup API
pub struct GSBClient {
    api_key: String,
    client_name: String,
    app_ver: String,
    pver: String
}

impl GSBClient {
    /// Creates a new GSBClient that will use 'key' as the GSB API key
    pub fn new(key: String) -> GSBClient {
        GSBClient {
            api_key: key,
            client_name: "gsbrs".to_string(),
            app_ver: "0.0.1".to_string(),
            pver: "3.1".to_string()
        }
    }

    /// Sets the GSBClient client_name to 'client_name'
    /// GSBClient uses 'gsbrs' as the client_name by default.
    pub fn change_client_name(&mut self, client_name: String) {
        self.client_name = client_name;
    }

    /// Queries GSB API with 'url', returns Vec of Status for 'url'
    pub fn lookup(&self, url: &Url) -> Result<Vec<Status>, GSBError> {
        let query = self.build_get_url(url.clone());
        let mut statuses : Vec<Status> = Vec::new();

        let client = Client::new();
        let mut res = try!(client.get(&query).send());

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
                // Given a well formed response from GSB this should never happen
                _   => panic!()
            };
        }
        Ok(statuses)
    }

    /// Build a queryable String with 'url'
    fn build_get_url(&self, url: Url) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();
        let url = format!("{}", url);

        let v : Vec<(&str, &str)> =
                vec![("client", self.client_name.as_ref()),
                    ("key", self.api_key.as_ref()),
                    ("appver", self.app_ver.as_ref()),
                    ("pver", self.pver.as_ref()),
                    ("url", url.as_ref())];

        base.set_query_from_pairs(v.into_iter());

        format!("{}",base)
    }

    /// Perform a bulk lookup on an iterable of urls.
    /// Returns a Vector of Vectors containing Statuses.
    /// # Panics
    /// Panics if more than 500 urls are passed in.
    pub fn lookup_all<'a, I> (&self, urls: I) -> Result<Vec<Vec<Status>>, GSBError>
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

            // GSB API only accepts 500 or fewer urls
            if size > 500 {
                panic!("Can not lookup more than 500 urls");
            }

            all_statuses.reserve(size);
            let size = size.to_string();
            let mut message = String::with_capacity(furls.len() + size.len());

            message.push_str(&size);
            message.push('\n');
            message.push_str(&furls);
            message.pop();
            message
        };

        let client = Client::new();
        let client = client.post(&url).body(&message);
        let mut res = try!(client.send());

        all_statuses = self.messages_from_response_post(&mut res);

        Ok(all_statuses)
    }

    /// Takes a reponse from GSB and splits it into lines of Statuses
    fn messages_from_response_post(&self, res: &mut hyper::client::response::Response) -> Vec<Vec<Status>> {
        let msgs = {
            let mut s = String::new();
            let _ = res.read_to_string(&mut s);
            s
        };

        let msgs : Vec<&str> = msgs.split("\n").collect();
        let mut all_statuses = Vec::with_capacity(msgs.len());

        for status_line in msgs {
            let status_line : Vec<&str> = status_line.split(",").collect();
            let mut statuses = Vec::with_capacity(status_line.len());

            statuses.extend(self.statuses_from_vec(&status_line));
            if !statuses.is_empty() {
                all_statuses.push(statuses);
            }
        }

        all_statuses
    }

    /// Builds a Vec<Status> from a slice of &str
    fn statuses_from_vec(&self, strstatuses: &[&str]) -> Vec<Status> {
        let mut statuses = Vec::new();
        for status in strstatuses {
            let status = *status;
            match status {
                "phishing"  => statuses.push(Status::Phishing),
                "malware"   => statuses.push(Status::Malware),
                "unwanted"  => statuses.push(Status::Unwanted),
                "ok"        => statuses.push(Status::Ok),
                ""  => (),
                _   => unreachable!()
            }
        }
        statuses
    }

    /// Builds a queryable string for POST requests
    fn build_post_url(&self) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();

        let v : Vec<(&str, &str)> =
                vec![("client", self.client_name.as_ref()),
                    ("key", self.api_key.as_ref()),
                    ("appver", self.app_ver.as_ref()),
                    ("pver", self.pver.as_ref())];

        base.set_query_from_pairs(v.into_iter());

        format!("{}",base)
    }

    // pub fn canonicalize(url: Url) -> Url {
    //     unimplemented!()
    // }

}
