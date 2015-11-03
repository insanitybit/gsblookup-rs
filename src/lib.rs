#![deny(warnings)]
// #![feature(test)]

extern crate hyper;
extern crate url;
#[cfg(all(test))]
extern crate quickcheck;

pub mod gsberror;

use gsberror::GSBError;
use hyper::Client;
use hyper::status::StatusCode;
use std::io::prelude::*;
use url::Url;

#[allow(non_upper_case_globals)]
/// Indicates the maximum number of urls Google can process at a time
pub static url_limit: u32 = 500;

/// Status represents each list a URL may be found in as well as a value,
/// 'Ok', which is used as a placeholder when the URL is not found in any
/// list. 'Ok' is only used in bulk queries.
#[derive(Debug, Clone, PartialEq)]
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
    pver: String,
    client: hyper::client::Client
}

impl GSBClient {
    /// Creates a new GSBClient that will use 'key' as the GSB API key
    pub fn new(key: String) -> GSBClient {
        GSBClient {
            api_key: key,
            client_name: "gsbrs".to_owned(),
            app_ver: env!("CARGO_PKG_VERSION").to_owned(),
            pver: "3.1".to_owned(),
            client: Client::new()
        }
    }

    /// Sets the GSBClient client_name to 'client_name'
    /// GSBClient uses 'gsbrs' as the client_name by default.
    pub fn change_client_name(&mut self, client_name: String) {
        self.client_name = client_name;
    }

    /// Queries GSB API with 'url', returns Vec of Status for 'url'
    pub fn lookup(&self, url: &str) -> Result<Vec<Status>, GSBError> {
        let query = self.build_get_url(url.clone());

        let msg = {
            let mut s = String::new();
            let mut res = try!((&self).client.get(&query).send());
            try!((&self).check_res(&mut res));
            try!(res.read_to_string(&mut s));
            s
        };

        let msg: Vec<&str> = msg.split(",").collect();

        let statuses = try!(self.statuses_from_vec(&msg));
        Ok(statuses)
    }

    /// Build a queryable String with 'url'
    fn build_get_url(&self, url: &str) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();

        let v: Vec<(&str, &str)> = vec![("client", self.client_name.as_ref()),
                                        ("key", self.api_key.as_ref()),
                                        ("appver", self.app_ver.as_ref()),
                                        ("pver", self.pver.as_ref()),
                                        ("url", url)];

        base.set_query_from_pairs(v.into_iter());

        format!("{}", base)
    }

    /// Takes an iterator of &str and returns a single string and the number of items
    /// counted in the iterator. If there are > url_limit items, return GSBError::TooManyUrls
    fn url_list_from_iter<'a, I>(&self, urls: I) -> Result<(String, usize), GSBError>
        where I: Iterator<Item = &'a str>
    {
        let mut url_list = String::new();
        let mut length: usize = 0;

        for url in urls {
            length = length + 1;
            url_list.push_str(url);
            url_list.push('\n');
        }
        url_list.pop();
        let length = length;

        // GSB API only accepts 500 or fewer urls
        if length > url_limit as usize {
            return Err(GSBError::TooManyUrls);
        }

        Ok((url_list, length))
    }

    /// Returns GSBError::HTTPStatusCode if Response StatusCode is not 200
    fn check_res(&self, res: &mut hyper::client::response::Response) -> Result<(), GSBError> {

        if res.status != StatusCode::Ok {
            if res.status != StatusCode::NoContent {
                return Err(GSBError::HTTPStatusCode(res.status))
            }
        }
        Ok(())
    }

    /// Perform a bulk lookup on an iterable of urls.
    /// Returns a Vector of Vectors containing Statuses.
    /// Returns GSBError::TooManyUrls if > 500 urls are pased in
    pub fn lookup_all<'a, I>(&self, urls: I) -> Result<Vec<Vec<Status>>, GSBError>
        where I: Iterator<Item = &'a str>
    {
        let url = self.build_post_url();

        let message = {
            let (url_list, length) = match (&self).url_list_from_iter(urls) {
                Ok((u,l))   => (u, l.to_string()),
                Err(e)  => return Err(e)
            };
            // length of message is the length of url_li
            let mut message = String::with_capacity(length.len() + url_list.len());

            message.push_str(&length);
            message.push('\n');
            message.push_str(&url_list);
            message
        };

        let post = (&self).client.post(&url).body(&message);
        let mut res = try!(post.send());
        try!((&self).check_res(&mut res));
        let res = res;
        let msgs = try!(self.messages_from_response_post(res));

        Ok(msgs)
    }

    /// Takes a reponse from GSB and splits it into lines of Statuses
    fn messages_from_response_post<R: Read>(&self, mut res: R) -> Result<Vec<Vec<Status>>, GSBError> {
        let msgs = {
            let mut s = String::new();
            try!(res.read_to_string(&mut s));
            s
        };

        let msgs: Vec<&str> = msgs.split("\n").collect();
        let mut all_statuses = Vec::with_capacity(msgs.len());

        for status_line in msgs {
            let status_line: Vec<&str> = status_line.split(",").collect();
            let statuses = try!(self.statuses_from_vec(&status_line));
            if !statuses.is_empty() {
                all_statuses.push(statuses);
            }
        }

        Ok(all_statuses)
    }

    /// Builds a Vec<Status> from a slice of &str
    fn statuses_from_vec(&self, strstatuses: &[&str]) -> Result<Vec<Status>, GSBError> {
        let mut statuses = Vec::new();
        for status in strstatuses {
            let status = *status;
            match status {
                "phishing" => statuses.push(Status::Phishing),
                "malware" => statuses.push(Status::Malware),
                "unwanted" => statuses.push(Status::Unwanted),
                "ok" => statuses.push(Status::Ok),
                "" => (),
                _ => return Err(GSBError::MalformedMessage(status.to_owned())),
            }
        }
        Ok(statuses)
    }

    /// Builds a queryable string for POST requests
    fn build_post_url(&self) -> String {
        let mut base = Url::parse("https://sb-ssl.google.com/safebrowsing/api/lookup?").unwrap();

        let v: Vec<(&str, &str)> = vec![("client", self.client_name.as_ref()),
                                        ("key", self.api_key.as_ref()),
                                        ("appver", self.app_ver.as_ref()),
                                        ("pver", self.pver.as_ref())];

        base.set_query_from_pairs(v.into_iter());

        format!("{}", base)
    }

// pub fn canonicalize(url: Url) -> Url {
//     unimplemented!()
// }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_post_url() {
        let g = GSBClient::new("testkey".to_owned());

        let s = format!("https://sb-ssl.google.\
                 com/safebrowsing/api/lookup?client=gsbrs&key=testkey&appver={}&pver=3.1",
                 env!("CARGO_PKG_VERSION"));
        assert_eq!(g.build_post_url(), s.to_owned());
    }


    #[test]
    fn test_build_get_url() {
        let g = GSBClient::new("testkey".to_owned());
        let u = "https://google.com/".to_owned();
        let s = format!("https://sb-ssl.google.com/safebrowsing/api/lookup?\
                client=gsbrs&key=testkey&appver={}&pver=3.1\
                &url=https%3A%2F%2Fgoogle.com%2F", env!("CARGO_PKG_VERSION"));
        assert_eq!(g.build_get_url(&u), s.to_owned());
    }

    #[test]
    fn test_statuses_from_vec() {
        let g = GSBClient::new("testkey".to_owned());
        let statuses = vec!["phishing", "malware", "unwanted", "ok"];
        let statuses= g.statuses_from_vec(&statuses).ok().expect("");
        assert_eq!(vec![Status::Phishing, Status::Malware, Status::Unwanted, Status::Ok], statuses);

        let statuses = vec!["", "", "", ""];
        let statuses= g.statuses_from_vec(&statuses).ok().expect("");
        assert!(statuses.is_empty());

        let statuses = vec!["malformed"];
        let statuses = g.statuses_from_vec(&statuses).unwrap_err();
        match statuses {
            gsberror::GSBError::MalformedMessage(msg) => {
                assert_eq!(msg, "malformed");
            },
            _  =>  panic!()
        }


    }
}

#[cfg(all(test))]
mod quicktests {
    use super::*;

    use quickcheck::quickcheck;

    fn quickcheck_build_get_url(url: String) {
        let g = GSBClient::new("testkey".to_owned());
            g.build_get_url(&url);
    }



    fn quickcheck_client_new(key: String) {
        let _ = GSBClient::new(key);
    }

    fn test_statuses_from_vec(strstatuses: Vec<String>) {
        let g = GSBClient::new("testkey".to_owned());
        let strstatuses : Vec<&str> = strstatuses.iter().map(|s| s.as_ref()).collect();
        let _ = g.statuses_from_vec(&strstatuses);
    }


    fn quickcheck_messages_from_response_post(cursor: String) {
        let g = GSBClient::new("testkey".to_owned());
        let cursor = cursor.as_bytes();
        let _ = g.messages_from_response_post(cursor);
    }

    fn quickcheck_set_name(name: String) {
        let mut g = GSBClient::new("testkey".to_owned());
        g.change_client_name(name);
    }

    #[test]
    fn test() {
        quickcheck(quickcheck_set_name as fn(String));

        quickcheck(quickcheck_messages_from_response_post as fn(String));
        quickcheck(test_statuses_from_vec as fn(Vec<String>));

        quickcheck(quickcheck_build_get_url as fn(String));
        quickcheck(quickcheck_client_new as fn(String));
    }

}
//
// #[cfg(test)]
// mod bench {
//     use super::*;
//     extern crate test;
//     use self::test::Bencher;
//
//     #[bench]
//     fn bench_build_get_url(b: &mut Bencher) {
//         let gsb = GSBClient::new("test".to_owned());
//         b.iter(|| {
//             gsb.build_get_url("https://google.com/");
//         });
//     }
//
//     #[bench]
//     fn bench_build_post_url(b: &mut Bencher) {
//         let gsb = GSBClient::new("test".to_owned());
//         b.iter(|| {
//             gsb.build_post_url();
//         });
//     }
//
//     #[bench]
//     fn bench_lookup(b: &mut Bencher) {
//         let count = test::black_box(1000);
//         let mut bstatuses = Vec::with_capacity(count );
//         for _ in 0..count {
//             bstatuses.push(test::black_box(Status::Phishing));
//         }
//
//         b.iter(|| {
//             let key: String = "AIzaSyCOZpyGR3gMKqrb5A9lGSsVKtr7".into();
//             let gsb = GSBClient::new(key);
//             let statuses = match gsb.lookup("https://google.com") {
//                 _  => bstatuses.clone()
//             };
//
//             if statuses.is_empty() {
//                 println!("Ok");
//             } else {
//                 for status in statuses {
//                     match status {
//                         Status::Phishing => test::black_box(()),
//                         Status::Malware => test::black_box(()),
//                         Status::Unwanted => test::black_box(()),
//                         // lookup only ever returns the above 3 statuses
//                         _ => unreachable!(),
//                     }
//                 }
//             }
//         });
//     }
//
// }
