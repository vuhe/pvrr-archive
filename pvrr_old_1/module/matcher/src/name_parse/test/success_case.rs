use super::{FilmBaseInfo, FilmNameParser};

#[test]
fn case_1() {
    let title = "【爪爪字幕组】★7月新番[欢迎来到实力至上主义的教室 第二季/Youkoso Jitsuryoku Shijou Shugi no Kyoushitsu e S2][11][1080p][HEVC][GB][MP4][招募翻译校对]";
    let item = FilmBaseInfo {
        title: vec![String::from("欢迎来到实力至上主义的教室")],
        year: None,
        season: Some(2),
        episode: Some(11),
        tag: Some(String::from("爪爪字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_2() {
    let title = "National.Parks.Adventure.AKA.America.Wild:.National.Parks.Adventure.3D.2016.1080p.Blu-ray.AVC.TrueHD.7.1";
    let item = FilmBaseInfo {
        title: vec![
            String::from("National Parks Adventure"),
            String::from("America Wild: National Parks Adventure"),
        ],
        year: Some(2016),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("Blu-ray")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_3() {
    let title =
        "[秋叶原冥途战争][Akiba Maid Sensou][2022][WEB-DL][1080][TV Series][第01话][LeagueWEB]";
    let item = FilmBaseInfo {
        title: vec![String::from("Akiba Maid Sensou")],
        year: Some(2022),
        season: None,
        episode: Some(1),
        tag: Some(String::from("秋叶原冥途战争")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_4() {
    let title = "哆啦A梦：大雄的宇宙小战争 2021 (2022) - 1080p.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("哆啦A梦：大雄的宇宙小战争 2021")],
        year: Some(2022),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_5() {
    let title = "新精武门1991 (1991).mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("新精武门1991")],
        year: Some(1991),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: None,
        streaming: None,
        resolution: None,
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_6() {
    let title = "24 S01 1080p WEB-DL AAC2.0 H.264-BTN";
    let item = FilmBaseInfo {
        title: vec![String::from("24")],
        year: None,
        season: Some(1),
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_7() {
    let title =
        "Qi Refining for 3000 Years S01E06 2022 1080p B-Blobal WEB-DL X264 AAC-AnimeS@AdWeb";
    let item = FilmBaseInfo {
        title: vec![String::from("Qi Refining for 3000 Years")],
        year: Some(2022),
        season: Some(1),
        episode: Some(6),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_8() {
    let title = "Noumin Kanren no Skill Bakka Agetetara Naze ka Tsuyoku Natta S01E02 2022 1080p B-Global WEB-DL X264 AAC-AnimeS@ADWeb[2022年10月新番]";
    let item = FilmBaseInfo {
        title: vec![String::from("Noumin Kanren no Skill Bakka Agetetara Naze ka Tsuyoku Natta")],
        year: Some(2022),
        season: Some(1),
        episode: Some(2),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: Some(String::from("B-Global")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_9() {
    let title = "dou luo da lu S01E229 2018 2160p WEB-DL H265 AAC-ADWeb[[国漫连载] 斗罗大陆 第229集 4k | 国语中字]";
    let item = FilmBaseInfo {
        title: vec![String::from("dou luo da lu")],
        year: Some(2018),
        season: Some(1),
        episode: Some(229),
        tag: Some(String::from("连载")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("4k")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_10() {
    let title = "Thor Love and Thunder (2022) [1080p] [WEBRip] [5.1]";
    let item = FilmBaseInfo {
        title: vec![String::from("Thor Love and Thunder")],
        year: Some(2022),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEBRip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_11() {
    let title = "[Animations(动画片)][[诛仙][Jade Dynasty][2022][WEB-DL][2160][TV Series][TV 08][LeagueWEB]][诛仙/诛仙动画 第一季 第08集 | 类型:动画 [国语中字]][680.12 MB]";
    let item = FilmBaseInfo {
        title: vec![String::from("诛仙")],
        year: Some(2022),
        season: Some(1),
        episode: Some(8),
        tag: Some(String::from("动画片")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("2160")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_12() {
    let title = "钢铁侠2 (2010) 1080p AC3.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("钢铁侠2")],
        year: Some(2010),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_13() {
    let title = "Wonder Woman 1984 2020 BluRay 1080p Atmos TrueHD 7.1 X264-EPiC";
    let item = FilmBaseInfo {
        title: vec![String::from("Wonder Woman 1984")],
        year: Some(2020),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("BluRay")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_14() {
    let title = "9-1-1 - S04E03 - Future Tense WEBDL-1080p.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("9-1-1")],
        year: None,
        season: Some(4),
        episode: Some(3),
        tag: None,
        version: None,
        source: Some(String::from("WEBDL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_15() {
    let title = "【幻月字幕组】【22年日剧】【据幸存的六人所说】【04】【1080P】【中日双语】";
    let item = FilmBaseInfo {
        title: vec![String::from("据幸存的六人所说")],
        year: None,
        season: None,
        episode: Some(4),
        tag: Some(String::from("幻月字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_16() {
    let title = "【爪爪字幕组】★7月新番[即使如此依旧步步进逼/Soredemo Ayumu wa Yosetekuru][09][1080p][HEVC][GB][MP4][招募翻译校对]";
    let item = FilmBaseInfo {
        title: vec![
            String::from("即使如此依旧步步进逼"),
            String::from("Soredemo Ayumu wa Yosetekuru"),
        ],
        year: None,
        season: None,
        episode: Some(9),
        tag: Some(String::from("爪爪字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_17() {
    let title =
        "[猎户不鸽发布组] 不死者之王 第四季 OVERLORD Ⅳ [02] [1080p] [简中内封] [2022年7月番]";
    let item = FilmBaseInfo {
        title: vec![String::from("不死者之王")],
        year: None,
        season: Some(4),
        episode: Some(2),
        tag: Some(String::from("猎户不鸽发布组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_18() {
    let title = "[GM-Team][国漫][寻剑 第1季][Sword Quest Season 1][2002][02][AVC][GB][1080P]";
    let item = FilmBaseInfo {
        title: vec![String::from("寻剑")],
        year: Some(2002),
        season: Some(1),
        episode: Some(2),
        tag: Some(String::from("GM-Team")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_19() {
    let title = " [猎户不鸽发布组] 组长女儿与照料专员 / 组长女儿与保姆 Kumichou Musume to Sewagakari [09] [1080p+] [简中内嵌] [2022年7月番]";
    let item = FilmBaseInfo {
        title: vec![
            String::from("组长女儿与照料专员"),
            String::from("组长女儿与保姆 Kumichou Musume to Sewagakari"),
        ],
        year: None,
        season: None,
        episode: Some(9),
        tag: Some(String::from("猎户不鸽发布组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_20() {
    let title = "Nande Koko ni Sensei ga!? 2019 Blu-ray Remux 1080p AVC LPCM-7³ ACG";
    let item = FilmBaseInfo {
        title: vec![String::from("Nande Koko ni Sensei ga!?")],
        year: Some(2019),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("Remux")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_21() {
    let title = "30.Rock.S02E01.1080p.BluRay.X264-BORDURE.mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("30 Rock")],
        year: None,
        season: Some(2),
        episode: Some(1),
        tag: None,
        version: None,
        source: Some(String::from("BluRay")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_22() {
    let title = "[Gal to Kyouryuu][02][BDRIP][1080P][H264_FLAC].mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Gal to Kyouryuu")],
        year: None,
        season: None,
        episode: Some(2),
        tag: None,
        version: None,
        source: Some(String::from("BDRIP")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_23() {
    let title = "[AI-Raws] 逆境無頼カイジ #13 (BD HEVC 1920x1080 yuv444p10le FLAC)[7CFEE642].mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("逆境無頼カイジ")],
        year: None,
        season: None,
        episode: Some(13),
        tag: Some(String::from("AI-Raws")),
        version: None,
        source: Some(String::from("BD")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_24() {
    let title = "Mr. Robot - S02E06 - eps2.4_m4ster-s1ave.aes SDTV.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("Mr  Robot")],
        year: None,
        season: Some(2),
        episode: Some(6),
        tag: None,
        version: None,
        source: Some(String::from("SDTV")),
        streaming: None,
        resolution: None,
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_25() {
    let title = "[神印王座][Throne of Seal][2022][WEB-DL][2160][TV Series][TV 22][LeagueWEB] 神印王座 第一季 第22集 | 类型:动画 [国语中字][967.44 MB]";
    let item = FilmBaseInfo {
        title: vec![String::from("Throne of Seal")],
        year: Some(2022),
        season: Some(1),
        episode: Some(22),
        tag: Some(String::from("神印王座")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("2160")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_26() {
    let title = "S02E1000.mkv";
    let item = FilmBaseInfo {
        title: vec![],
        year: None,
        season: Some(2),
        episode: Some(1000),
        tag: None,
        version: None,
        source: None,
        streaming: None,
        resolution: None,
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_27() {
    let title =
        "[梦蓝字幕组]New Doraemon 哆啦A梦新番[754][2023.04.15][AVC][1080P][GB_JP][MP4] [574.6MB]";
    let item = FilmBaseInfo {
        title: vec![String::from("New Doraemon 哆啦A梦新番")],
        year: None,
        season: None,
        episode: Some(754),
        tag: Some(String::from("梦蓝字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_28() {
    let title = "[ANi] OVERLORD 第四季 - 04 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("OVERLORD")],
        year: None,
        season: Some(4),
        episode: Some(4),
        tag: Some(String::from("ANi")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: Some(String::from("Baha")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_29() {
    let title =
        "[SweetSub&LoliHouse] Made in Abyss S2 - 03v2 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Made in Abyss")],
        year: None,
        season: Some(2),
        episode: Some(3),
        tag: Some(String::from("SweetSub&LoliHouse")),
        version: Some(String::from("v2")),
        source: Some(String::from("WebRip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_30() {
    let title = "[GM-Team][国漫][斗破苍穹 第5季][Fights Break Sphere V][2022][05][HEVC][GB][4K]";
    let item = FilmBaseInfo {
        title: vec![String::from("斗破苍穹")],
        year: Some(2022),
        season: Some(5),
        episode: Some(5),
        tag: Some(String::from("GM-Team")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("4K")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_31() {
    let title = "Ousama Ranking S01E02-[1080p][BDRIP][X265.FLAC].mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Ousama Ranking")],
        year: None,
        season: Some(1),
        episode: Some(2),
        tag: None,
        version: None,
        source: Some(String::from("BDRIP")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_32() {
    let title = "[Nekomoe kissaten&LoliHouse] Soredemo Ayumu wa Yosetekuru - 01v2 [WebRip 1080p HEVC-10bit EAC3 ASSx2].mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Soredemo Ayumu wa Yosetekuru")],
        year: None,
        season: None,
        episode: Some(1),
        tag: Some(String::from("Nekomoe kissaten&LoliHouse")),
        version: Some(String::from("v2")),
        source: Some(String::from("WebRip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_33() {
    let title = "[喵萌奶茶屋&LoliHouse] 金装的薇尔梅 / Kinsou no Vermeil - 01 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕]";
    let item = FilmBaseInfo {
        title: vec![String::from("金装的薇尔梅"), String::from("Kinsou no Vermeil")],
        year: None,
        season: None,
        episode: Some(1),
        tag: Some(String::from("喵萌奶茶屋&LoliHouse")),
        version: None,
        source: Some(String::from("WebRip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_34() {
    let title = "Hataraku.Maou-sama.S02E05.2022.1080p.CR.WEB-DL.X264.AAC-ADWeb.mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Hataraku Maou-sama")],
        year: Some(2022),
        season: Some(2),
        episode: Some(5),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_35() {
    let title = "The Witch Part 2：The Other One 2022 1080p WEB-DL AAC5.1 H264-tG1R0";
    let item = FilmBaseInfo {
        title: vec![String::from("The Witch Part 2：The Other One")],
        year: Some(2022),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_36() {
    let title = "一夜新娘 - S02E07 - 第 7 集.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("一夜新娘")],
        year: None,
        season: Some(2),
        episode: Some(7),
        tag: None,
        version: None,
        source: None,
        streaming: None,
        resolution: None,
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_37() {
    let title = "[ANi] 處刑少女的生存之道 - 07 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("處刑少女的生存之道")],
        year: None,
        season: None,
        episode: Some(7),
        tag: Some(String::from("ANi")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: Some(String::from("Baha")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_38() {
    let title = "Stand-up.Comedy.S01E01.PartA.2022.1080p.WEB-DL.H264.AAC-TJUPT.mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("Stand-up Comedy")],
        year: Some(2022),
        season: Some(1),
        episode: Some(1),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_39() {
    let title = "教父3.The.Godfather.Part.III.1990.1080p.NF.WEBRip.H264.DDP5.1-PTerWEB.mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("教父3 The Godfather Part III")],
        year: Some(1990),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEBRip")),
        streaming: Some(String::from("NF")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_40() {
    let title = "A.Quiet.Place.Part.II.2020.1080p.UHD.BluRay.DD+7.1.DoVi.X265-PuTao";
    let item = FilmBaseInfo {
        title: vec![String::from("A Quiet Place Part II")],
        year: Some(2020),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("BluRay")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_41() {
    let title = "Childhood.In.A.Capsule.S01E16.2022.1080p.KKTV.WEB-DL.X264.AAC-ADWeb.mkv";
    let item = FilmBaseInfo {
        title: vec![String::from("Childhood In A Capsule")],
        year: Some(2022),
        season: Some(1),
        episode: Some(16),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_42() {
    let title = "[桜都字幕组] 异世界归来的舅舅 / Isekai Ojisan [01][1080p][简体内嵌]";
    let item = FilmBaseInfo {
        title: vec![String::from("异世界归来的舅舅"), String::from("Isekai Ojisan")],
        year: None,
        season: None,
        episode: Some(1),
        tag: Some(String::from("桜都字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_43() {
    let title = "【喵萌奶茶屋】★04月新番★[夏日重現/Summer Time Rendering][15][720p][繁日雙語][招募翻譯片源]";
    let item = FilmBaseInfo {
        title: vec![String::from("夏日重現"), String::from("Summer Time Rendering")],
        year: None,
        season: None,
        episode: Some(15),
        tag: Some(String::from("喵萌奶茶屋")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("720")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_44() {
    let title = "[NC-Raws] 打工吧！魔王大人 第二季 / Hataraku Maou-sama!! - 02 (B-Global 1920x1080 HEVC AAC MKV)";
    let item = FilmBaseInfo {
        title: vec![String::from("打工吧！魔王大人")],
        year: None,
        season: Some(2),
        episode: Some(2),
        tag: Some(String::from("NC-Raws")),
        version: None,
        source: None,
        streaming: Some(String::from("B-Global")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_45() {
    let title = "The Witch Part 2 The Other One 2022 1080p WEB-DL AAC5.1 H.264-tG1R0";
    let item = FilmBaseInfo {
        title: vec![String::from("The Witch Part 2 The Other One")],
        year: Some(2022),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_46() {
    let title = "The 355 2022 BluRay 1080p DTS-HD MA5.1 X265.10bit-BeiTai";
    let item = FilmBaseInfo {
        title: vec![String::from("The 355")],
        year: Some(2022),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("BluRay")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

/// 多季忽略
#[test]
fn case_47() {
    let title = "Sense8 s01-s02 2015-2017 1080P WEB-DL X265 AC3￡cXcY@FRDS";
    let item = FilmBaseInfo {
        title: vec![String::from("Sense8")],
        year: Some(2015),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

/// 单季多集仅忽略集，因为季信息可能参与后续匹配
#[test]
fn case_48() {
    let title = "The Heart of Genius S01 13-14 2022 1080p WEB-DL H264 AAC";
    let item = FilmBaseInfo {
        title: vec![String::from("The Heart of Genius")],
        year: Some(2022),
        season: Some(1),
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_49() {
    let title = "2022.8.2.Twelve.Monkeys.1995.GBR.4K.REMASTERED.BluRay.1080p.X264.DTS [3.4 GB]";
    let item = FilmBaseInfo {
        title: vec![String::from("Twelve Monkeys")],
        year: Some(1995),
        season: None,
        episode: None,
        tag: None,
        version: None,
        source: Some(String::from("BluRay")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_50() {
    let title = "[NC-Raws] 王者天下 第四季 - 17 (Baha 1920x1080 AVC AAC MP4) [3B1AA7BB].mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("王者天下")],
        year: None,
        season: Some(4),
        episode: Some(17),
        tag: Some(String::from("NC-Raws")),
        version: None,
        source: None,
        streaming: Some(String::from("Baha")),
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_51() {
    let title = "Sense8 S2E1 2015-2017 1080P WEB-DL X265 AC3￡cXcY@FRDS";
    let item = FilmBaseInfo {
        title: vec![String::from("Sense8")],
        year: Some(2015),
        season: Some(2),
        episode: Some(1),
        tag: None,
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

/// 多集忽略
#[test]
fn case_52() {
    let title = "[xyx98]传颂之物/Utawarerumono/うたわれるもの[BDrip][1920x1080][TV 01-26 Fin][hevc-yuv420p10 flac_ac3][ENG PGS]";
    let item = FilmBaseInfo {
        title: vec![
            String::from("传颂之物"),
            String::from("Utawarerumono"),
            String::from("うたわれるもの"),
        ],
        year: None,
        season: None,
        episode: None,
        tag: Some(String::from("xyx98")),
        version: None,
        source: Some(String::from("BDrip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_53() {
    let title = "[云歌字幕组][7月新番][欢迎来到实力至上主义的教室 第二季][01][X264 10bit][1080p][简体中文].mp4";
    let item = FilmBaseInfo {
        title: vec![String::from("欢迎来到实力至上主义的教室")],
        year: None,
        season: Some(2),
        episode: Some(1),
        tag: Some(String::from("云歌字幕组")),
        version: None,
        source: None,
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_54() {
    let title = "[诛仙][Jade Dynasty][2022][WEB-DL][2160][TV Series][TV 04][LeagueWEB]";
    let item = FilmBaseInfo {
        title: vec![String::from("Jade Dynasty")],
        year: Some(2022),
        season: None,
        episode: Some(4),
        tag: Some(String::from("诛仙")),
        version: None,
        source: Some(String::from("WEB-DL")),
        streaming: None,
        resolution: Some(String::from("2160")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}

#[test]
fn case_55() {
    let title = "Rick and Morty.S06E06.JuRicksic.Mort.1080p.HMAX.WEBRip.DD5.1.X264-NTb[rartv]";
    let item = FilmBaseInfo {
        title: vec![String::from("Rick and Morty")],
        year: None,
        season: Some(6),
        episode: Some(6),
        tag: Some(String::from("rartv")),
        version: None,
        source: Some(String::from("WEBRip")),
        streaming: None,
        resolution: Some(String::from("1080")),
    };
    assert_eq!(FilmNameParser::parse(title), item);
}
