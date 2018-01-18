#![warn(missing_docs)]
//! Small binary that serves a small HTML to favorite select few
//! internet radio stations and have them (remotely) played via
//! MPD 
//!
//! # Examples

extern crate mpd;

fn main() {
    println!("Hello, world!");

    use mpd::Client;

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    conn.volume(100).unwrap();
    conn.load("My Lounge Playlist", ..).unwrap();
    conn.play().unwrap();

    println!("Status: {:?}", conn.status());
}