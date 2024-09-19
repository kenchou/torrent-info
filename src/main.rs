use bip_metainfo::Metainfo;
use clap::{Arg, Command};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 clap 解析命令行参数
    let matches = Command::new("Torrent Info Hash Extractor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Extracts the info hash from a torrent file")
        .arg(
            Arg::new("file_path")
                .help("The path to the torrent file")
                .required(true)
                .index(1),
        )
        .get_matches();

    // 获取 file_path 参数
    let file_path = matches.get_one::<String>("file_path").expect("file_path is required");

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
    println!("Magnet URI: {}", magnet_uri);

    Ok(())
}