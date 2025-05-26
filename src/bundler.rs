use crate::parser::{parse_imports, transform_es_to_commonjs};
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ModuleInfo {
    pub id: String,
    pub code: String,
    pub deps: Vec<String>,
}

pub type ModuleGraph = HashMap<String, ModuleInfo>;

fn resolve_module_path(base_path: &Path, dep: &str) -> PathBuf {
    let mut dep_path = base_path.join(dep);
    if dep_path.extension().is_none() {
        dep_path.set_extension("js");
    }

    dep_path
        .canonicalize()
        .unwrap_or_else(|_| panic!("无法解析模块路径: {:?}", dep_path))
}

fn to_relative_id(full_path: &Path, base_dir: &Path) -> String {
    let full_path = full_path.canonicalize().expect("无法规范化 full_path");
    let base_dir = base_dir.canonicalize().expect("无法规范化 base_dir");

    let rel = full_path
        .strip_prefix(base_dir)
        .unwrap()
        .to_string_lossy()
        .replace("\\", "/");

    if rel.starts_with("./") {
        rel
    } else {
        format!("./{}", rel)
    }
}

fn walk(
    path: &Path,
    base_dir: &Path,
    graph: &mut ModuleGraph,
    path_stack: &mut Vec<String>,
    format: &str,
) {
    let id = to_relative_id(path, base_dir);

    // 避免循环依赖
    if path_stack.contains(&id) {
        println!("检测到循环依赖: {}", id);
        return;
    }

    // 避免重复加载
    if graph.contains_key(&id) {
        return;
    }

    println!("walk 进入: {}, 当前 path_stack: {:?}", id, path_stack);
    path_stack.push(id.clone());

    let raw_code = fs::read_to_string(path).expect(&format!("读取文件失败: {:?}", path));
    let code = match format {
        "cjs" => transform_es_to_commonjs(&raw_code),
        _ => raw_code,
    };

    let imports = parse_imports(&code);

    let module = ModuleInfo {
        id: id.clone(),
        code,
        deps: imports.clone(),
    };

    graph.insert(id.clone(), module);

    for dep in imports {
        let full_dep_path = resolve_module_path(path.parent().unwrap(), &dep);

        walk(&full_dep_path, base_dir, graph, path_stack, format);
    }

    // 避免误报循环依赖
    path_stack.pop();

    println!("walk 离开: {}, 当前 path_stack: {:?}", id, path_stack);
}

pub fn build_module_graph(entry: &str, format: &str) -> HashMap<String, ModuleInfo> {
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();

    let mut graph = HashMap::new();

    let mut path_stack: Vec<String> = vec![];

    walk(&entry_path, &base_dir, &mut graph, &mut path_stack, format);

    graph
}

pub fn bundle(graph: &ModuleGraph, entry: &str, format: &str) -> String {
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();
    let entry_id = to_relative_id(&entry_path, &base_dir);
    let mut output = String::new();

    match format {
        "esm" => {
            for (id, module) in graph {
                output.push_str(&format!("// {}\n{}\n", id, module.code));
            }
            output.push_str(&format!("\nimport \"{}\";\n", entry_id));
        }
        _ => {
            // 1. 开始模块定义对象
            output.push_str("const modules = {\n");

            for (id, module) in graph {
                output.push_str(&format!(
                    " {:?}: function(require, module, exports) {{\n",
                    id
                ));
                output.push_str(&module.code);
                output.push_str("\n  },\n");
                // modules_code.push_str(&format!(
                //     "\"{}\": function(require, module, exports) {{\n{}\n}}, \n",
                //     id, module.code
                // ));
                // println!("打包模块: {}", id);
            }
            output.push_str("};\n\n");

            // 2. 定义require函数
            output.push_str(
                r#"
                    const cache = {};
                    function require(id) {
                        if(cache[id]){
                            return cache[id].exports;
                        }
                        const module = {exports: {}};
                        cache[id] = module;
                        modules[id](require, module, module.exports);
                        return module.exports;
                    }
                "#,
            );

            // 3. 执行入口模块
            output.push_str(&format!("\nrequire({:?});\n", entry_id));
        }
    }

    output
}

pub fn print_module_graph(graph: &ModuleGraph, entry_id: &str, indent: usize) {
    if let Some(module) = graph.get(entry_id) {
        let prefix = " ".repeat(indent);
        println!("{}- {}", prefix, module.id);
        for dep in &module.deps {
            print_module_graph(graph, dep, indent + 1);
        }
    }
}

pub fn bundle_to_file(
    entry_path: &Path,
    output_path: &PathBuf,
    format: &str,
) -> Result<(), String> {
    if !entry_path.exists() {
        return Err(format!("入口文件不存在: {:?}", entry_path));
    }

    if !entry_path.is_file() {
        return Err(format!("入口文件不是一个文件: {:?}", entry_path));
    }

    if output_path.exists() && !output_path.is_file() {
        return Err(format!("输出路径不是一个文件: {:?}", output_path));
    }

    if output_path.exists() {
        fs::remove_file(output_path).map_err(|e| format!("删除原输出文件失败: {}", e))?;
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建输出目录失败: {}", e))?;
    }

    let entry_str = entry_path.to_str().ok_or("入口路径包含非法字符")?;
    let graph = build_module_graph(entry_path.to_str().unwrap(), format);
    let code = bundle(&graph, entry_str, format);

    fs::write(output_path, code).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
