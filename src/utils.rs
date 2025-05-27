use std::path::{Path, PathBuf};
use crate::graph::ModuleGraph;

pub fn resolve_module_path(base_path: &Path, dep: &str) -> PathBuf {
    let mut dep_path = base_path.join(dep);
    if dep_path.extension().is_none() {
        dep_path.set_extension("js");
    }

    dep_path
        .canonicalize()
        .unwrap_or_else(|_| panic!("无法解析模块路径: {:?}", dep_path))
}

pub fn to_relative_id(full_path: &Path, base_dir: &Path) -> String {
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

pub fn print_module_graph(graph: &ModuleGraph, entry_id: &str, indent: usize) {
    if let Some(module) = graph.get(entry_id) {
        let prefix = " ".repeat(indent);
        println!("{}- {}", prefix, module.id);
        for dep in &module.deps {
            print_module_graph(graph, dep, indent + 1);
        }
    }
}

