use std::cmp::{max, min};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[rustfmt::skip]
fn sim_jaro(s1: &str, s2: &str) -> f64 {
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    if s1.len() == 0 && s2.len() == 0 { return 1.0; }

    let match_distance: isize = max(max(s1.len(), s2.len()) as isize / 2 - 1, 0);
    let mut s1_matches = vec![false; s1.len()];
    let mut s2_matches = vec![false; s2.len()];
    let mut m: isize = 0;
    for i in 0..s1.len() {
        let start = max(0, i as isize - match_distance) as usize;
        let end = min(i + match_distance as usize + 1, s2.len());
        for j in start..end {
            if !s2_matches[j] && s1.get(i) == s2.get(j) {
                s1_matches[i] = true;
                s2_matches[j] = true;
                m += 1;
                break;
            }
        }
    }
    if m == 0 { return 0.0; }
    let mut t = 0.0;
    let mut k = 0;
    for i in 0..s1.len() {
        if s1_matches[i] {
            while !s2_matches[k] { k += 1; }
            if s1.get(i) != s2.get(k) { t += 0.5; }
            k += 1;
        }
    }

    let m = m as f64;
    (m / s1.len() as f64 + m / s2.len() as f64 + (m  - t) / m) / 3.0
}

#[derive(Debug)]
pub(super) struct PathEntry {
    pub(super) path: PathBuf,
    pub(super) name: String,
    distance: f64,
}

impl PathEntry {
    fn new(dir: DirEntry, key_word: &str, prefix: &HashSet<&OsStr>) -> Self {
        let path = dir.into_path();
        let (distance, name) = path
            .components()
            .map(|it| it.as_os_str())
            .filter(|it| !prefix.contains(it))
            .map(|it| it.to_str())
            .filter(|it| it.is_some())
            .map(|it| it.unwrap())
            .map(|it| (sim_jaro(key_word, it), it))
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap_or((0.0, ""));
        let name = name.to_owned();
        Self { path, name, distance }
    }
}

/// 搜索路径下所有文件并和 key_word 进行对比，返回排序后的文件路径
pub(super) fn search(path: &Path, key_word: &str) -> Vec<PathEntry> {
    let prefix: HashSet<&OsStr> = path.components().map(|it| it.as_os_str()).collect();
    let mut paths: Vec<PathEntry> = WalkDir::new(path)
        .into_iter()
        .filter(|it| it.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| it.path().is_file())
        .map(|it| PathEntry::new(it, key_word, &prefix))
        .filter(|it| 0.05 < it.distance)
        .collect();
    paths.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
    paths
}

#[cfg(test)]
mod test {
    use super::search;
    use std::path::Path;

    #[test]
    fn search_test() {
        let ans = search(Path::new("./"), "torznab");
        println!("{:#?}", ans);
    }
}
