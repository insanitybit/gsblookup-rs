[![License](http://img.shields.io/badge/license-MIT-blue.svg)]

[![Build Status](https://travis-ci.org/insanitybit/gsblookup-rs.png)](https://travis-ci.org/insanitybit/gsblookup-rs)

# gsblookup-rs
Rust interface to Google Safe Browsing Lookup API

# Example

```rust
  let key : String = "API KEY HERE".into();
  let url = Url::parse("http://exampleurl.org/").unwrap();

  let gsb = GSBClient::new(key);
  let statuses = gsb.lookup(url);

  if statuses.is_empty() {
    println!("Url not found in any of Google's lists");
  } else {
    for status in statuses {
        match status {
            Status::Phishing    => println!("Phishing"),
            Status::Malware     => println!("Malware"),
            Status::Unwanted    => println!("Unwanted")
        }
    }
  }
```

See examples/ for more.
