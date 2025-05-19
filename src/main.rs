mod bundler;
mod parser;

use bundler::{build_module_graph, bundle, print_module_graph};
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

    println!("Module Graph: ");
    print_module_graph(&graph, "./index.js", 0);

    fs::create_dir_all("dist").unwrap();
    fs::write("dist/bundle.js", bundled_code).unwrap();

    println!("✅ 打包完成，输出到 dist/bundle.js");
}
