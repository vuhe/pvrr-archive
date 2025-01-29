use anyhow::{ensure, Result};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// 创建文件硬链接（需要两个路径在同一挂载卷）
fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let file = from.as_ref();
    ensure!(file.is_file(), "路径 {} 非文件.", file.display());
    std::fs::hard_link(from, to)?;
    Ok(())
}

/// 拷贝文件
fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let file = from.as_ref();
    ensure!(file.is_file(), "路径 {} 非文件.", file.display());
    std::fs::copy(from, to)?;
    Ok(())
}

/// 修改文件路径（需要两个路径在同一挂载卷）
fn modify_file_path<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    std::fs::rename(from.as_ref(), to.as_ref())?;
    Ok(())
}

/// 先拷贝文件再删除文件
fn copy_delete_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let file = from.as_ref();
    ensure!(file.is_file(), "路径 {} 非文件.", file.display());
    std::fs::copy(from.as_ref(), to)?;
    std::fs::remove_file(from.as_ref())?;
    Ok(())
}

/// 列出文件夹下所有的文件，
/// 如果路径本身是文件，则返回文件本身
fn tree<P: AsRef<Path>>(dir: P) -> Vec<FileEntry> {
    WalkDir::new(dir.as_ref())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|it| it.file_type().is_file())
        .map(|it| FileEntry::new(dir.as_ref(), it))
        .collect()
}

/// 文件信息
struct FileEntry {
    /// 存储实际路径和查找信息
    entry: DirEntry,
    /// 子路径 path，用于快速获取子路径，
    /// 仅在创建时构建，entry 中 path 的一部分
    sub_path: Box<Path>,
}

impl FileEntry {
    fn new(root: &Path, entry: DirEntry) -> Self {
        let full_path = entry.path();
        let depth = entry.depth();
        let sub_path = match depth {
            // depth == 0 说明本身为文件，要取文件名，因此需要将 root 上移一级
            0 => match root.parent() {
                // 如果目录没有上级，那么直接使用本级即可
                None => full_path,
                // 如果有上级，但上级为 "" (常见于没有以 / 开始的路径)，那么直接使用本级即可
                Some(parent) if parent == Path::new("") => full_path,
                // 如果有上级，使用上级目录将本目录截断
                Some(parent) => full_path.strip_prefix(parent).unwrap(),
            },
            // depth > 0 说明本身为文件夹，直接使用 root 将本目录截断
            _ => full_path.strip_prefix(root).unwrap(),
        }
        .into();
        Self { entry, sub_path }
    }

    /// 基于所给文件路径的子目录路径
    /// - 如果提供的根路径为文件，则会返回文件名
    /// - 如果提供的根路径为文件夹，则会返回子目录开始的路径名
    fn components(&self) -> impl Iterator<Item = Option<&str>> {
        let path = self.sub_path.as_ref();
        path.components().map(|it| it.as_os_str().to_str())
    }
}
