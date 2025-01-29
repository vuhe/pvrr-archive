use super::{FilmBaseInfo, FilmNameParser};

#[test]
fn case_1() {
    let title = "[ANi] 带著智慧型手机闯荡异世界。2 - 05 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4] [375.12 MB]";
    let item = FilmBaseInfo {
        title: vec![String::from("带著智慧型手机闯荡异世界")],
        year: None,
        season: Some(2),
        episode: Some(5),
        tag: Some(String::from("ANi")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: Some(String::from("Baha")),
        resolution: Some(String::from("1080")),
    };
    assert_ne!(FilmNameParser::parse(title), item, "标题会被识别为: 带著智慧型手机闯荡异世界。2");
}
