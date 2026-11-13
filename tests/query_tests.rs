mod harness;

use std::io;
use std::io::{Read, Write};
use gag::BufferRedirect;
use harness::query_harness;

#[test]
fn query_count_unconditional() {
    const QUERY: &str = "SELECT count(name) FROM Playlist(testing);";

    const EXPECTED: &str = "|COUNT(name)|
|-----------|
|10         |
";

    let output_res =  query_harness(QUERY.to_string());

    assert!(output_res.is_ok());

    let output = output_res.unwrap();

    assert_eq!(output, EXPECTED)
}

#[test]
fn query_count_artist_arcticmonkeys() {
    const QUERY: &str = "SELECT count(name) FROM Playlist(testing) WHERE \"Arctic Monkeys\" IN artists;";

    const EXPECTED: &str = "|COUNT(name)|
|-----------|
|4          |
";

    let output_res =  query_harness(QUERY.to_string());

    assert!(output_res.is_ok());

    let output = output_res.unwrap();

    assert_eq!(output, EXPECTED)
}