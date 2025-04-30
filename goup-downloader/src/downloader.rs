use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time;
use std::{fs, fs::File};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::blocking;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use sha2::{Digest, Sha256};

use goup_version::Dir;

use crate::archived::Unpack;
use crate::utils;

pub fn install_go_version(version: &str) -> anyhow::Result<()> {
    let goup_home = Dir::goup_home()?;
    let version_dest_dir = goup_home.version(version);

    let mp = MultiProgress::new();
    let sp1 = mp.add(ProgressBar::new_spinner());
    sp1.enable_steady_tick(time::Duration::from_millis(100));
    sp1.set_message(format!("Installing {}", version));
    let sp2 = mp.add(ProgressBar::new_spinner());
    sp2.enable_steady_tick(time::Duration::from_millis(100));

    // 是否已解压成功并且存在
    if goup_home.is_dot_unpacked_success_file_exists(version) {
        sp2.finish_with_message(format!("Already installed {}", version,));
        return Ok(());
    }

    // download directory
    let dl_dest_dir = goup_home.cache();
    // 压缩包文件名称
    let archive_filename = utils::go_version_archive(version);
    // 压缩包sha256文件名称
    let archive_sha256_filename = utils::archive_sha256(&archive_filename);
    // 压缩包url
    let (archive_url, archive_sha256_url) = utils::archive_url(&archive_filename);

    if !dl_dest_dir.exists() {
        log::debug!("Create download directory");
        fs::create_dir_all(&dl_dest_dir)?
    }

    // 压缩包文件
    let archive_file = dl_dest_dir.join_path(archive_filename);
    let archive_sha256_file = dl_dest_dir.join_path(archive_sha256_filename);

    if !archive_file.exists() || !archive_sha256_file.exists() {
        // 下载压缩包
        sp2.set_message(format!("Downloading {}", archive_url));
        download_archive(&mp, &archive_file, &archive_url)?;

        // 下载压缩包sha256
        sp2.set_message(format!("Downloading {}", archive_sha256_url));
        download_archive_sha256(&archive_sha256_file, &archive_sha256_url)?;
    }

    // 校验压缩包sha256
    sp2.set_message(format!("Verifying {}", archive_sha256_file.display()));
    let ok = verify_archive_file_sha256(&archive_file, &archive_sha256_file)?;
    if !ok {
        // TODO: here should remove the bad archive_file.
        anyhow::bail!("Hashsum NOT match {}", archive_sha256_file.display());
    }

    // 解压
    sp2.set_message(format!("Unpacking {}", archive_file.display()));
    if !version_dest_dir.exists() {
        log::debug!("Create version directory: {}", version_dest_dir.display());
        fs::create_dir_all(&version_dest_dir)?
    }
    archive_file
        .to_string_lossy()
        .parse::<Unpack>()?
        .unpack(&version_dest_dir, &archive_file)?;
    sp2.finish_and_clear();

    // 设置解压成功
    goup_home.create_dot_unpacked_success_file(version)?;
    sp1.finish_with_message(format!("Installed {}", version));

    Ok(())
}

/// download_archive 下载压缩包
fn download_archive<P: AsRef<Path>>(
    mp: &MultiProgress,
    dest: P,
    archive_url: &str,
) -> anyhow::Result<()> {
    let client = Client::new();

    let resp = client
        .head(archive_url)
        .header("User-Agent", "GOUP Client")
        .timeout(time::Duration::from_secs(10))
        .send()?;
    let headers = resp.headers();
    let content_length = headers
        .get(CONTENT_LENGTH)
        .unwrap()
        .to_str()?
        .parse::<u64>()?;

    let pb = mp.add(ProgressBar::new(content_length));
    pb.set_style(
            ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));
    pb.enable_steady_tick(time::Duration::from_millis(100));

    let mut cache_file = fs::File::create(dest)?;

    let mut start = 0;
    const CHUNK_SIZE: u64 = 1024 * 1024;
    while start < content_length {
        let end = start + CHUNK_SIZE;
        let range = format!("bytes={}-{}", start, end);
        let buf = client
            .get(archive_url)
            .header("User-Agent", "GOUP Client")
            .header("Range", range)
            .timeout(time::Duration::from_secs(30))
            .send()?
            .bytes()?;
        cache_file.write_all(&buf)?;
        pb.inc(buf.len() as u64);
        start = end + 1;
    }

    pb.finish_and_clear();
    mp.remove(&pb);

    Ok(())
}

/// download_archive_sha256 下载压缩包sha256
fn download_archive_sha256<P: AsRef<Path>>(
    dest: P,
    archive_sha256_url: &str,
) -> anyhow::Result<()> {
    let mut response = blocking::get(archive_sha256_url)?;
    if !response.status().is_success() {
        anyhow::bail!("Downloading archive failure");
    }
    let mut file = File::create(dest)?;
    response.copy_to(&mut file)?;
    Ok(())
}

/// compute_file_sha256 计算文件的sha256
fn compute_file_sha256<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let mut context = Sha256::new();
    let mut file = File::open(path)?;
    let mut buffer = [0; 4096]; // 定义一个缓冲区来处理字节流数据
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", context.finalize()))
}

/// verify_archive_file_sha256 校验文件压缩包的sha256
fn verify_archive_file_sha256<P1, P2>(
    archive_file: P1,
    archive_sha256_file: P2,
) -> anyhow::Result<bool>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let expect_sha256 = fs::read_to_string(archive_sha256_file)?;
    let expect_sha256 = expect_sha256.trim();
    let got_sha256 = compute_file_sha256(&archive_file)?;

    Ok(expect_sha256 == got_sha256)
}
