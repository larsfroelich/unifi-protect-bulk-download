# UniFi-Protect Rust Library

[![Crates.io](https://img.shields.io/crates/v/unifi-protect.svg)](https://crates.io/crates/unifi-protect)
[![Docs.rs](https://docs.rs/unifi_protect/badge.svg)](https://docs.rs/unifi_protect)
[![Build Status](https://travis-ci.org/xlfpx/unifi_protect_rust.svg?branch=main)](https://travis-ci.org/xlfpx/unifi_protect_rust)

This library enables interacting with a unifi protect server (such as the one running on a ubiquiti cloud key gen2).
It aims to eventually be a complete implementation of the unifi protect api in Rust, closely following the example of the javascript-based  https://github.com/hjdhjd/unifi-protect/
It is also used in the [unifi-protect-bulk-download](https://github.com/xlfpx/unifi-protect-bulk-download) tool for downloading footage from a unifi protect system.

To add the library to an existing cargo project: `cargo add unifi-protect`

Basic usage:
```rust
use unifi_protect::UnifiProtectServer;

// [ ...]
// within an async context:

// create a new instance using the base uri of the unifi protect server (same uri you would use to reach the system's web portal)
let mut server = UnifiProtectServer::new("BASE_URI"); // ( e.g. "https://192.168.1.28")
// login with username + password credentials (same credentials you would use to login to the system's web portal)
server
    .login("USERNAME", "PASSWORD")
    .await
    .expect("Failed to login");
server
    .fetch_cameras()
    .await
    .expect("Failed to fetch cameras");
println!("Found {} cameras", server.cameras.len());
```