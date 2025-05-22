mod bundler;
mod parser;
mod parser_cjs_exports;

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
    #[arg(short, long, default_value = "dist/bundle.js")]
    output: PathBuf,

    /// Ouput module format: esm | cjs
    #[arg(short, long, default_value = "cjs")]
    format: String,
}

fn main() {
    let cli = Cli::parse();

    match bundle_to_file(&cli.entry, &cli.output, &cli.format) {
        Ok(_) => println!("✅ 打包成功，输出文件: {:?}", cli.output),
        Err(e) => eprintln!("❌ 打包失败: {}", e),
    }
}
