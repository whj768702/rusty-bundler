mod parser;
mod bundler;

use std::env;
use std::fs;
use bundler::build_module_graph;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("用法: rusty-bundler <入口文件>");
        return;
    }

    let entry = &args[1];
    println!("entry: {}", entry);
    let graph = build_module_graph(entry);

    let mut output = String::new();
    output.push_str("// Bundled by rust-bundler\n");

    for(_id, module) in &graph {
        output.push_str(&format!(
            "\n// module: {}\n{}\n",
            module.id,
            module.code
        ));
    }

    fs::create_dir_all("dist").unwrap();
    fs::write("dist/bundle.js", output).unwrap();

    println!("✅ 打包完成，输出到 dist/bundle.js");
}
