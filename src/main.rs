mod bundler;
mod parser;

use bundler::bundle_to_file;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rusty-bundler")]
#[command(about = "A simple JS bundler written in Rust")]
struct Args {
    /// Entry js file path
    entry: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "dist/bundle.js")]
    out: PathBuf,
}

fn main() {
    let args = Args::parse();

    match bundle_to_file(&args.entry, &args.out) {
        Ok(_) => println!("✅ Bundle complete!"),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // if args.len() < 2 {
    //     println!("用法: rusty-bundler <入口文件>");
    //     return;
    // }

    // let entry = &args[1];
    // let graph = build_module_graph(entry);

    // let bundled_code = bundle(&graph, entry);

    // println!("Module Graph: ");
    // print_module_graph(&graph, "./index.js", 0);

    // fs::create_dir_all("dist").unwrap();
    // fs::write("dist/bundle.js", bundled_code).unwrap();

    println!("✅ 打包完成，输出到 dist/bundle.js");
}
