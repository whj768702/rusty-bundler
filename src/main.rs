mod bundler;
mod parser;

use bundler::{build_module_graph, bundle};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: rusty-bundler <入口文件>");
        return;
    }

    let entry = &args[1];
    let graph = build_module_graph(entry);

    let bundled_code = bundle(&graph, entry);

    fs::create_dir_all("dist").unwrap();
    fs::write("dist/bundle.js", bundled_code).unwrap();

    println!("✅ 打包完成，输出到 dist/bundle.js");
}
