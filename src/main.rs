mod utils;
mod bundler;
mod parser;
mod parser_cjs_exports;
mod graph;
mod format;

use bundler::bundle_to_file;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rusty-bundler",
    version,
    about = "A simple JS bundler written in Rust"
)]
struct Cli {
    /// Path to the entry JavaScript file
    #[arg(short, long)]
    entry: PathBuf,

    /// Path to the output bundle file
    #[arg(short, long, default_value = "dist")]
    out_dir: PathBuf,

    /// Output module format: esm | cjs
    #[arg(long, default_value = "cjs")]
    format: String,
}

fn main() {
    let cli = Cli::parse();
    
    let formats_vec: Vec<String> = cli.format.split(",").map(|s| s.trim().to_string()).collect();
    
    let formats:&[String] = &formats_vec;

    match bundle_to_file(&cli.entry, &cli.out_dir, formats) {
        Ok(_) => println!("✅ 打包成功，输出文件: {:?}", cli.out_dir),
        Err(e) => eprintln!("❌ 打包失败: {}", e),
    }
}
