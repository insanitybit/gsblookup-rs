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
gsbrs = "0.5.0"
```

# Example

Looking up a single URL.

```rust
let key: String = "AIzaSyCOZpyGR3gMKqrb5A9lGSsVKtr7".into();

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
            // lookup_all can return Status::Ok as well
            _ => unreachable!(),
        }
    }
}
```

See [examples/](https://github.com/insanitybit/gsblookup-rs/tree/master/examples) for more.

This library does not use any 'unsafe' blocks.
