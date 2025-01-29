use std::fs::File;
use std::io::{BufReader, Read};
use torrent::Torrent;

#[test]
fn torrent_parse_test() {
    let file = File::open("./tests/test.torrent").unwrap();
    let file = BufReader::new(file);
    let bytes: Vec<u8> = file.bytes().map(|it| it.unwrap()).collect();
    let torrent = Torrent::builder().set_content(bytes).unwrap().build();
    println!("{:?}", torrent);
}
