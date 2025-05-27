use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::parser::{parse_imports};
use crate::utils::{resolve_module_path, to_relative_id};
use crate::format::transform_es_to_commonjs;

pub struct ModuleInfo {
    pub id: String,
    pub code: String,
    pub deps: Vec<String>,
}

pub type ModuleGraph = HashMap<String, ModuleInfo>;

pub fn walk(
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
