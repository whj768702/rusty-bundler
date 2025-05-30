use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::{to_relative_id};
use crate::graph::{walk, ModuleGraph, ModuleInfo};

pub fn build_module_graph(entry: &str, formats: &[String]) -> HashMap<String, ModuleInfo> {
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();

    let mut graph = HashMap::new();

    let mut path_stack: Vec<String> = vec![];

    walk(&entry_path, &base_dir, &mut graph, &mut path_stack, formats);

    graph
}

pub fn bundle(graph: &ModuleGraph, entry: &str, formats: &[String]) -> HashMap<String, String> {
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();
    let entry_id = to_relative_id(&entry_path, &base_dir);
    let mut result = HashMap::new();

    for format in formats {
        let mut output = String::new();
        for module in graph.values() {
            if let Some(code) = module.code.get(format) {
                output.push_str(code);
                output.push('\n');
            }
        }
        result.insert(format.clone(), output);
        // match format {
        //     "esm" => {
        //         for (id, module) in graph {
        //             output.push_str(&format!("// {}\n{}\n", id, module.code));
        //         }
        //         output.push_str(&format!("\nimport \"{}\";\n", entry_id));
        //         
        //         result.insert(format.to_string(), output.clone());
        //     }
        //     cjs => {
        //         // 1. 开始模块定义对象
        //         output.push_str("const modules = {\n");
        // 
        //         for (id, module) in graph {
        //             output.push_str(&format!(
        //                 " {:?}: function(require, module, exports) {{\n",
        //                 id
        //             ));
        //             output.push_str(&module.code);
        //             output.push_str("\n  },\n");
        //         }
        //         output.push_str("};\n\n");
        // 
        //         // 2. 定义require函数
        //         output.push_str(
        //             r#"
        //             const cache = {};
        //             function require(id) {
        //                 if(cache[id]){
        //                     return cache[id].exports;
        //                 }
        //                 const module = {exports: {}};
        //                 cache[id] = module;
        //                 modules[id](require, module, module.exports);
        //                 return module.exports;
        //             }
        //         "#,
        //         );
        // 
        //         // 3. 执行入口模块
        //         output.push_str(&format!("\nrequire({:?});\n", entry_id));
        //         
        //         result.insert(format.to_string(), output.clone());
        //     }
        // }
    }

    result
}

pub fn bundle_to_file(
    entry_path: &Path,
    output_dir: &PathBuf,
    formats: &[String],
) -> Result<(), String> {
    if !entry_path.exists() {
        return Err(format!("入口文件不存在: {:?}", entry_path));
    }

    if !entry_path.is_file() {
        return Err(format!("入口文件不是一个文件: {:?}", entry_path));
    }

    if output_dir.exists() && !output_dir.is_dir() {
        return Err(format!("输出路径不是一个目录: {:?}", output_dir));
    }

    if output_dir.exists() {
        fs::remove_dir_all(output_dir).map_err(|e| format!("删除原输出文件失败: {}", e))?;
    }

   /* if let Some(parent) = output_dir.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建输出目录失败: {}", e))?;
    }*/

    let entry_str = entry_path.to_str().ok_or("入口路径包含非法字符")?;
    let graph = build_module_graph(entry_path.to_str().unwrap(), formats);
    let code = bundle(&graph, entry_str, formats);
    
    for (fmt, code) in code {
        let filename = format!("bundle.{}.js", fmt);
        let output_path = output_dir.join(filename);
        
        // 确保每个文件的父级目录都存在
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建输出目录失败: {}", e))?;
        }
        println!("输出文件: {:?}", output_path);
        fs::write(output_path, code).map_err(|e| format!("写入文件失败: {}", e))?;
    }

    // fs::write(output_path, code).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
