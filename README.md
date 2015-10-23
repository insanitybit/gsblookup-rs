[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/insanitybit/gsblookup-rs/blob/master/LICENSE) [![Build Status](https://travis-ci.org/insanitybit/gsblookup-rs.png)](https://travis-ci.org/insanitybit/gsblookup-rs)
[![](https://meritbadge.herokuapp.com/gsbrs)](https://crates.io/crates/gsbrs)
[![Coverage Status](https://coveralls.io/repos/insanitybit/gsblookup-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/insanitybit/gsblookup-rs?branch=master)

# gsblookup-rs
Rust interface to [Google Safe Browsing Lookup API](https://developers.google.com/safe-browsing/lookup_guide)

[Documentation](https://insanitybit.github.io/gsblookup-rs/gsbrs/)

# Usage

Available on crates.io

Add this to your Cargo.toml

```toml
[dependencies]
gsbrs = "*"
```

# Example

Looking up a single URL.

```rust
  let key : String = "API KEY HERE".into();
  let url = Url::parse("http://exampleurl.org/").unwrap();

  let gsb = GSBClient::new(key);
  let statuses = gsb.lookup(url).unwrap();

  if statuses.is_empty() {
    println!("Url not found in any of Google's lists");
  } else {
    for status in statuses {
        match status {
            Status::Phishing    => println!("Phishing"),
            Status::Malware     => println!("Malware"),
            Status::Unwanted    => println!("Unwanted"),
            _                   => ()
        }
    }
  }
```

See [examples/](https://github.com/insanitybit/gsblookup-rs/tree/master/examples) for more.
