use core::torrent::Torrent;
use std::fs::File;
use std::io::Read;
use tokio_test::block_on;

#[test]
fn magnet_v1_test() {
    let url = "magnet:?xt=urn:btih:1ec0dbd01cfd4150b113bd95c4f02435e3a4d270";
    let torrent = block_on(Torrent::from_magnet(url)).unwrap();
    println!("{torrent:#?}");
}

#[test]
#[should_panic]
fn magnet_v2_test() {
    let url = "magnet:?xt=urn:btmh:1220caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e&dn=bittorrent-v2-test";
    let torrent = block_on(Torrent::from_magnet(url)).unwrap();
    println!("{torrent:#?}");
}

#[test]
fn magnet_hybrid_test() {
    let url = "magnet:?xt=urn:btih:631a31dd0a46257d5078c0dee4e66e26f73e42ac&xt=urn:btmh:1220d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb&dn=bittorrent-v1-v2-hybrid-test";
    let torrent = block_on(Torrent::from_magnet(url)).unwrap();
    println!("{torrent:#?}");
}

#[test]
fn torrent_v1_test() {
    let mut file = File::open("tests/bittorrent-v1-test.torrent").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let torrent = Torrent::from_bytes(&bytes).unwrap();
    println!("{torrent:#?}");
}

#[test]
fn torrent_v2_test() {
    let mut file = File::open("tests/bittorrent-v2-test.torrent").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let torrent = Torrent::from_bytes(&bytes).unwrap();
    println!("{torrent:#?}");
}

#[test]
fn torrent_hybrid_test() {
    let mut file = File::open("tests/bittorrent-v2-hybrid-test.torrent").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let torrent = Torrent::from_bytes(&bytes).unwrap();
    println!("{torrent:#?}");
}
