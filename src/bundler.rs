use crate::parser::{parse_imports, transform_es_to_commonjs};
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
    let raw_code = fs::read_to_string(path).expect(&format!("读取文件失败: {:?}", path));
    let code = transform_es_to_commonjs(&raw_code);
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
    let entry_path = Path::new(entry).to_path_buf();
    let base_dir = entry_path.parent().unwrap().to_path_buf();
    let entry_id = to_relative_id(&entry_path, &base_dir);
    let mut output = String::new();

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

    output

    // let entry_path = Path::new(entry).canonicalize().unwrap();
    // let entry_id = to_relative_id(&entry_path, entry_path.parent().unwrap());

    // format!(
    //     r#"(function(modules) {{
    //      const require = (id) => {{
    //        const fn = modules[id];
    //        const module = {{ exports: {{}} }};
    //        fn(require, module, module.exports);
    //        return module.exports;
    //      }};
    //      require("{}");
    //    }})({{
    //      {}
    //    }});"#,
    //     entry_id, modules_code
    // )
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
