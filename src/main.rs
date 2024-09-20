use bip_metainfo::Metainfo;
use clap::{Arg, Command};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 clap 解析命令行参数
    let matches = Command::new("Torrent Info Hash Extractor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Extracts the info hash from torrent files")
        .arg(
            Arg::new("file_paths")
                .help("The paths to the torrent files")
                .required(true)
                .value_name("FILE")
                .num_args(1..),
        )
        .get_matches();

    // 获取 file_paths 参数
    let file_paths: Vec<&String> = matches.get_many("file_paths").unwrap().collect();

    println!("File\tMagnet\tName");
    for file_path in file_paths {
        // 打开并读取 torrent 文件
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 解析 torrent 文件
        let metainfo = Metainfo::from_bytes(&buffer)?;

        // 获取 info_hash
        let info_hash = metainfo.info().info_hash();

        // 将 info_hash 转换为十六进制字符串
        let info_hash_hex = hex::encode(info_hash.as_ref());

        // 构建 magnet URI
        let magnet_uri = format!("magnet:?xt=urn:btih:{}", info_hash_hex);

        // 打印 magnet URI
        println!(
            "{}\t{}\t{}",
            Path::new(file_path).file_name().unwrap().to_string_lossy(),
            magnet_uri,
            metainfo.info().directory().unwrap().to_string_lossy(),
        );
    }

    Ok(())
}
