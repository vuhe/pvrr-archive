use base_tool::text::Text;
use matcher::static_match_title as match_title;
use matcher::{FilmEpisode, FilmResolution, FilmSource, FilmStreaming, MatchedItem};

#[test]
fn case_1() {
    let title = "【爪爪字幕组】★7月新番[欢迎来到实力至上主义的教室 第二季/Youkoso Jitsuryoku Shijou Shugi no Kyoushitsu e S2][11][1080p][HEVC][GB][MP4][招募翻译校对]";
    let item = MatchedItem {
        title: Text::from("欢迎来到实力至上主义的教室"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 11 },
        year: None,
        tags: vec![Text::from("爪爪字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_2() {
    let title = "National.Parks.Adventure.AKA.America.Wild:.National.Parks.Adventure.3D.2016.1080p.Blu-ray.AVC.TrueHD.7.1";
    let item = MatchedItem {
        title: Text::from("America Wild: National Parks Adventure"),
        episode: FilmEpisode::Movie,
        year: Some(2016),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_3() {
    let title =
        "[秋叶原冥途战争][Akiba Maid Sensou][2022][WEB-DL][1080][TV Series][第01话][LeagueWEB]";
    let item = MatchedItem {
        title: Text::from("Akiba Maid Sensou"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 1 },
        year: Some(2022),
        tags: vec![Text::from("秋叶原冥途战争")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_4() {
    let title = "哆啦A梦：大雄的宇宙小战争 2021 (2022) - 1080p.mp4";
    let item = MatchedItem {
        title: Text::from("哆啦A梦：大雄的宇宙小战争 2021"),
        episode: FilmEpisode::Movie,
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_5() {
    let title = "新精武门1991 (1991).mkv";
    let item = MatchedItem {
        title: Text::from("新精武门1991"),
        episode: FilmEpisode::Movie,
        year: Some(1991),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::Unknown,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_6() {
    let title = "24 S01 1080p WEB-DL AAC2.0 H.264-BTN";
    let item = MatchedItem {
        title: Text::from("24"),
        episode: FilmEpisode::OneSeason(1),
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_7() {
    let title =
        "Qi Refining for 3000 Years S01E06 2022 1080p B-Blobal WEB-DL X264 AAC-AnimeS@AdWeb";
    let item = MatchedItem {
        title: Text::from("Qi Refining for 3000 Years"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 6 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_8() {
    let title = "Noumin Kanren no Skill Bakka Agetetara Naze ka Tsuyoku Natta S01E02 2022 1080p B-Global WEB-DL X264 AAC-AnimeS@ADWeb[2022年10月新番]";
    let item = MatchedItem {
        title: Text::from("Noumin Kanren no Skill Bakka Agetetara Naze ka Tsuyoku Natta"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 2 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Bilibili,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_9() {
    let title = "dou luo da lu S01E229 2018 2160p WEB-DL H265 AAC-ADWeb[[国漫连载] 斗罗大陆 第229集 4k | 国语中字]";
    let item = MatchedItem {
        title: Text::from("dou luo da lu"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 229 },
        year: Some(2018),
        tags: vec![Text::from("连载")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R2160,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_10() {
    let title = "Thor Love and Thunder (2022) [1080p] [WEBRip] [5.1]";
    let item = MatchedItem {
        title: Text::from("Thor Love and Thunder"),
        episode: FilmEpisode::Movie,
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_11() {
    let title = "[Animations(动画片)][[诛仙][Jade Dynasty][2022][WEB-DL][2160][TV Series][TV 08][LeagueWEB]][诛仙/诛仙动画 第一季 第08集 | 类型:动画 [国语中字]][680.12 MB]";
    let item = MatchedItem {
        title: Text::from("诛仙"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 8 },
        year: Some(2022),
        tags: vec![Text::from("动画片")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R2160,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_12() {
    let title = "钢铁侠2 (2010) 1080p AC3.mp4";
    let item = MatchedItem {
        title: Text::from("钢铁侠2"),
        episode: FilmEpisode::Movie,
        year: Some(2010),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_13() {
    let title = "Wonder Woman 1984 2020 BluRay 1080p Atmos TrueHD 7.1 X264-EPiC";
    let item = MatchedItem {
        title: Text::from("Wonder Woman 1984"),
        episode: FilmEpisode::Movie,
        year: Some(2020),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_14() {
    let title = "9-1-1 - S04E03 - Future Tense WEBDL-1080p.mp4";
    let item = MatchedItem {
        title: Text::from("9-1-1"),
        episode: FilmEpisode::SingleEpisode { season: 4, episode: 3 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_15() {
    let title = "【幻月字幕组】【22年日剧】【据幸存的六人所说】【04】【1080P】【中日双语】";
    let item = MatchedItem {
        title: Text::from("据幸存的六人所说"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 4 },
        year: None,
        tags: vec![Text::from("幻月字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_16() {
    let title = "【爪爪字幕组】★7月新番[即使如此依旧步步进逼/Soredemo Ayumu wa Yosetekuru][09][1080p][HEVC][GB][MP4][招募翻译校对]";
    let item = MatchedItem {
        title: Text::from("即使如此依旧步步进逼"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 9 },
        year: None,
        tags: vec![Text::from("爪爪字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_17() {
    let title =
        "[猎户不鸽发布组] 不死者之王 第四季 OVERLORD Ⅳ [02] [1080p] [简中内封] [2022年7月番]";
    let item = MatchedItem {
        title: Text::from("不死者之王"),
        episode: FilmEpisode::SingleEpisode { season: 4, episode: 2 },
        year: None,
        tags: vec![Text::from("猎户不鸽发布组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_18() {
    let title = "[GM-Team][国漫][寻剑 第1季][Sword Quest Season 1][2002][02][AVC][GB][1080P]";
    let item = MatchedItem {
        title: Text::from("寻剑"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 2 },
        year: Some(2002),
        tags: vec![Text::from("GM-Team")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_19() {
    let title = " [猎户不鸽发布组] 组长女儿与照料专员 / 组长女儿与保姆 Kumichou Musume to Sewagakari [09] [1080p+] [简中内嵌] [2022年7月番]";
    let item = MatchedItem {
        title: Text::from("组长女儿与照料专员"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 9 },
        year: None,
        tags: vec![Text::from("猎户不鸽发布组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_20() {
    let title = "Nande Koko ni Sensei ga!? 2019 Blu-ray Remux 1080p AVC LPCM-7³ ACG";
    let item = MatchedItem {
        title: Text::from("Nande Koko ni Sensei ga!?"),
        episode: FilmEpisode::Movie,
        year: Some(2019),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_21() {
    let title = "30.Rock.S02E01.1080p.BluRay.X264-BORDURE.mkv";
    let item = MatchedItem {
        title: Text::from("30 Rock"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 1 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_22() {
    let title = "[Gal to Kyouryuu][02][BDRIP][1080P][H264_FLAC].mkv";
    let item = MatchedItem {
        title: Text::from("Gal to Kyouryuu"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 2 },
        year: None,
        tags: vec![Text::from("Gal to Kyouryuu")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_23() {
    let title = "[AI-Raws] 逆境無頼カイジ #13 (BD HEVC 1920x1080 yuv444p10le FLAC)[7CFEE642].mkv";
    let item = MatchedItem {
        title: Text::from("逆境無頼カイジ"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 13 },
        year: None,
        tags: vec![Text::from("AI-Raws")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_24() {
    let title = "Mr. Robot - S02E06 - eps2.4_m4ster-s1ave.aes SDTV.mp4";
    let item = MatchedItem {
        title: Text::from("Mr  Robot"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 6 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::SDTV,
        resolution: FilmResolution::Unknown,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_25() {
    let title = "[神印王座][Throne of Seal][2022][WEB-DL][2160][TV Series][TV 22][LeagueWEB] 神印王座 第一季 第22集 | 类型:动画 [国语中字][967.44 MB]";
    let item = MatchedItem {
        title: Text::from("Throne of Seal"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 22 },
        year: Some(2022),
        tags: vec![Text::from("神印王座")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R2160,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_26() {
    let title = "S02E1000.mkv";
    let item = MatchedItem {
        title: Text::default(),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 1000 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::Unknown,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_27() {
    let title =
        "[梦蓝字幕组]New Doraemon 哆啦A梦新番[754][2023.04.15][AVC][1080P][GB_JP][MP4] [574.6MB]";
    let item = MatchedItem {
        title: Text::from("New Doraemon 哆啦A梦新番"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 754 },
        year: None,
        tags: vec![Text::from("梦蓝字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_28() {
    let title = "[ANi] OVERLORD 第四季 - 04 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4";
    let item = MatchedItem {
        title: Text::from("OVERLORD"),
        episode: FilmEpisode::SingleEpisode { season: 4, episode: 4 },
        year: None,
        tags: vec![Text::from("ANi")],
        streaming: FilmStreaming::Baha,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_29() {
    let title =
        "[SweetSub&LoliHouse] Made in Abyss S2 - 03v2 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv";
    let item = MatchedItem {
        title: Text::from("Made in Abyss"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 3 },
        year: None,
        tags: vec![Text::from("SweetSub&LoliHouse"), Text::from("v2")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_30() {
    let title = "[GM-Team][国漫][斗破苍穹 第5季][Fights Break Sphere V][2022][05][HEVC][GB][4K]";
    let item = MatchedItem {
        title: Text::from("斗破苍穹"),
        episode: FilmEpisode::SingleEpisode { season: 5, episode: 5 },
        year: Some(2022),
        tags: vec![Text::from("GM-Team")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R2160,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_31() {
    let title = "Ousama Ranking S01E02-[1080p][BDRIP][X265.FLAC].mkv";
    let item = MatchedItem {
        title: Text::from("Ousama Ranking"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 2 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_32() {
    let title = "[Nekomoe kissaten&LoliHouse] Soredemo Ayumu wa Yosetekuru - 01v2 [WebRip 1080p HEVC-10bit EAC3 ASSx2].mkv";
    let item = MatchedItem {
        title: Text::from("Soredemo Ayumu wa Yosetekuru"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 1 },
        year: None,
        tags: vec![Text::from("Nekomoe kissaten&LoliHouse"), Text::from("v2")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_33() {
    let title = "[喵萌奶茶屋&LoliHouse] 金装的薇尔梅 / Kinsou no Vermeil - 01 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕]";
    let item = MatchedItem {
        title: Text::from("金装的薇尔梅"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 1 },
        year: None,
        tags: vec![Text::from("喵萌奶茶屋&LoliHouse")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_34() {
    let title = "Hataraku.Maou-sama.S02E05.2022.1080p.CR.WEB-DL.X264.AAC-ADWeb.mkv";
    let item = MatchedItem {
        title: Text::from("Hataraku Maou-sama"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 5 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_35() {
    let title = "The Witch Part 2：The Other One 2022 1080p WEB-DL AAC5.1 H264-tG1R0";
    let item = MatchedItem {
        title: Text::from("The Witch Part 2：The Other One"),
        episode: FilmEpisode::Movie,
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_36() {
    let title = "一夜新娘 - S02E07 - 第 7 集.mp4";
    let item = MatchedItem {
        title: Text::from("一夜新娘"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 7 },
        year: None,
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::Unknown,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_37() {
    let title = "[ANi] 處刑少女的生存之道 - 07 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4";
    let item = MatchedItem {
        title: Text::from("處刑少女的生存之道"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 7 },
        year: None,
        tags: vec![Text::from("ANi")],
        streaming: FilmStreaming::Baha,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_38() {
    let title = "Stand-up.Comedy.S01E01.PartA.2022.1080p.WEB-DL.H264.AAC-TJUPT.mp4";
    let item = MatchedItem {
        title: Text::from("Stand-up Comedy"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 1 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_39() {
    let title = "教父3.The.Godfather.Part.III.1990.1080p.NF.WEBRip.H264.DDP5.1-PTerWEB.mkv";
    let item = MatchedItem {
        title: Text::from("教父3 The Godfather Part III"),
        episode: FilmEpisode::Movie,
        year: Some(1990),
        tags: vec![],
        streaming: FilmStreaming::Netflix,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_40() {
    let title = "A.Quiet.Place.Part.II.2020.1080p.UHD.BluRay.DD+7.1.DoVi.X265-PuTao";
    let item = MatchedItem {
        title: Text::from("A Quiet Place Part II"),
        episode: FilmEpisode::Movie,
        year: Some(2020),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_41() {
    let title = "Childhood.In.A.Capsule.S01E16.2022.1080p.KKTV.WEB-DL.X264.AAC-ADWeb.mkv";
    let item = MatchedItem {
        title: Text::from("Childhood In A Capsule"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 16 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_42() {
    let title = "[桜都字幕组] 异世界归来的舅舅 / Isekai Ojisan [01][1080p][简体内嵌]";
    let item = MatchedItem {
        title: Text::from("异世界归来的舅舅"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 1 },
        year: None,
        tags: vec![Text::from("桜都字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_43() {
    let title = "【喵萌奶茶屋】★04月新番★[夏日重現/Summer Time Rendering][15][720p][繁日雙語][招募翻譯片源]";
    let item = MatchedItem {
        title: Text::from("夏日重現"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 15 },
        year: None,
        tags: vec![Text::from("喵萌奶茶屋")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R720,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_44() {
    let title = "[NC-Raws] 打工吧！魔王大人 第二季 / Hataraku Maou-sama!! - 02 (B-Global 1920x1080 HEVC AAC MKV)";
    let item = MatchedItem {
        title: Text::from("打工吧！魔王大人"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 2 },
        year: None,
        tags: vec![Text::from("NC-Raws")],
        streaming: FilmStreaming::Bilibili,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_45() {
    let title = "The Witch Part 2 The Other One 2022 1080p WEB-DL AAC5.1 H.264-tG1R0";
    let item = MatchedItem {
        title: Text::from("The Witch Part 2 The Other One"),
        episode: FilmEpisode::Movie,
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_46() {
    let title = "The 355 2022 BluRay 1080p DTS-HD MA5.1 X265.10bit-BeiTai";
    let item = MatchedItem {
        title: Text::from("The 355"),
        episode: FilmEpisode::Movie,
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_47() {
    let title = "Sense8 s01-s02 2015-2017 1080P WEB-DL X265 AC3￡cXcY@FRDS";
    let item = MatchedItem {
        title: Text::from("Sense8"),
        episode: FilmEpisode::MultiSeason(1..3),
        year: Some(2015),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_48() {
    let title = "The Heart of Genius S01 13-14 2022 1080p WEB-DL H264 AAC";
    let item = MatchedItem {
        title: Text::from("The Heart of Genius"),
        episode: FilmEpisode::MultiEpisode { season: 1, episode: 13..15 },
        year: Some(2022),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_49() {
    let title = "2022.8.2.Twelve.Monkeys.1995.GBR.4K.REMASTERED.BluRay.1080p.X264.DTS [3.4 GB]";
    let item = MatchedItem {
        title: Text::from("Twelve Monkeys"),
        episode: FilmEpisode::Movie,
        year: Some(1995),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_50() {
    let title = "[NC-Raws] 王者天下 第四季 - 17 (Baha 1920x1080 AVC AAC MP4) [3B1AA7BB].mp4";
    let item = MatchedItem {
        title: Text::from("王者天下"),
        episode: FilmEpisode::SingleEpisode { season: 4, episode: 17 },
        year: None,
        tags: vec![Text::from("NC-Raws")],
        streaming: FilmStreaming::Baha,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_51() {
    let title = "Sense8 S2E1 2015-2017 1080P WEB-DL X265 AC3￡cXcY@FRDS";
    let item = MatchedItem {
        title: Text::from("Sense8"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 1 },
        year: Some(2015),
        tags: vec![],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_52() {
    let title = "[xyx98]传颂之物/Utawarerumono/うたわれるもの[BDrip][1920x1080][TV 01-26 Fin][hevc-yuv420p10 flac_ac3][ENG PGS]";
    let item = MatchedItem {
        title: Text::from("传颂之物"),
        episode: FilmEpisode::MultiEpisode { season: 1, episode: 1..27 },
        year: None,
        tags: vec![Text::from("xyx98")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Bluray,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_53() {
    let title = "[云歌字幕组][7月新番][欢迎来到实力至上主义的教室 第二季][01][X264 10bit][1080p][简体中文].mp4";
    let item = MatchedItem {
        title: Text::from("欢迎来到实力至上主义的教室"),
        episode: FilmEpisode::SingleEpisode { season: 2, episode: 1 },
        year: None,
        tags: vec![Text::from("云歌字幕组")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::Unknown,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_54() {
    let title = "[诛仙][Jade Dynasty][2022][WEB-DL][2160][TV Series][TV 04][LeagueWEB]";
    let item = MatchedItem {
        title: Text::from("Jade Dynasty"),
        episode: FilmEpisode::SingleEpisode { season: 1, episode: 4 },
        year: Some(2022),
        tags: vec![Text::from("诛仙")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebDL,
        resolution: FilmResolution::R2160,
    };
    assert_eq!(match_title(Text::from(title)), item);
}

#[test]
fn case_56() {
    let title = "Rick and Morty.S06E06.JuRicksic.Mort.1080p.HMAX.WEBRip.DD5.1.X264-NTb[rartv]";
    let item = MatchedItem {
        title: Text::from("Rick and Morty"),
        episode: FilmEpisode::SingleEpisode { season: 6, episode: 6 },
        year: None,
        tags: vec![Text::from("rartv")],
        streaming: FilmStreaming::Unknown,
        source: FilmSource::WebRip,
        resolution: FilmResolution::R1080,
    };
    assert_eq!(match_title(Text::from(title)), item);
}
