use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::parser::parse_imports;

pub struct ModuleInfo {
    pub id: String,
    pub code: String,
    pub deps: Vec<String>,
}

pub type ModuleGraph = HashMap<String, ModuleInfo>;

pub fn build_module_graph(entry: &str)->HashMap<String, ModuleInfo> {
    let mut graph = HashMap::new();
    let entry_path = Path::new(entry).to_path_buf();

    fn walk(path:&Path, graph:&mut ModuleGraph) {
        let code = fs::read_to_string(path).unwrap_or_else(|e| panic!("读取文件失败: {}, path={}", e, path.display()));
        let imports = parse_imports(&code);

        let id = path.to_string_lossy().to_string();

        let module = ModuleInfo {
            id: id.clone(),
            code,
            deps: imports.clone(),
        };

        graph.insert(id.clone(), module);

        for dep in imports {
            let mut dep_path = path.parent().unwrap().join(&dep);

            if dep_path.extension().is_none(){
                dep_path.set_extension("js");
            }
            let dep_id = dep_path.to_string_lossy().to_string();
            if !graph.contains_key(&dep_id) {
                walk(&dep_path, graph);
            }
        }
    }

    walk(&entry_path, &mut graph);

    graph
}
