use crate::parser::parse_imports;
use std::collections::HashMap;
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
}

fn to_relative_id(full_path: &Path, base_dir: &Path) -> String {
    let rel = full_path
        .strip_prefix(base_dir)
        .unwrap()
        .to_string_lossy()
        .replace("\\", "/");

    if !rel.starts_with("./") {
        format!("./{}", rel)
    } else {
        rel
    }
}

fn walk(path: &Path, base_dir: &Path, graph: &mut ModuleGraph) {
    let code = fs::read_to_string(path).expect(&format!("读取文件失败: {:?}", path));
    println!("code: {}", code);
    let imports = parse_imports(&code);

    let id = to_relative_id(path, base_dir);

    if graph.contains_key(&id) {
        return;
    }

    let module = ModuleInfo {
        id: id.clone(),
        code,
        deps: imports.clone(),
    };

    graph.insert(id.clone(), module);

    for dep in imports {
        let full_dep_path = resolve_module_path(path.parent().unwrap(), &dep);

        walk(&full_dep_path, base_dir, graph);
    }
}

pub fn build_module_graph(entry: &str) -> HashMap<String, ModuleInfo> {
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();

    let mut graph = HashMap::new();

    walk(&entry_path, &base_dir, &mut graph);

    graph
}

pub fn bundle(graph: &ModuleGraph, entry: &str) -> String {
    let mut modules_code = String::new();

    for (id, module) in graph {
        modules_code.push_str(&format!(
            "\"{}\": function(require, module, exports) {{\n{}\n}}, \n",
            id, module.code
        ));
        println!("打包模块: {}", id);
    }

    let entry_path = Path::new(entry).canonicalize().unwrap();
    let entry_id = to_relative_id(&entry_path, entry_path.parent().unwrap());

    format!(
        r#"(function(modules) {{
         const require = (id) => {{
           const fn = modules[id];
           const module = {{ exports: {{}} }};
           fn(require, module, module.exports);
           return module.exports;
         }};
         require("{}");
       }})({{
         {}
       }});"#,
        entry_id, modules_code
    )
}
