use std::collections::BTreeMap;

use regex::Regex;

pub struct ExportCollector {
    export_map: BTreeMap<String, String>,
}

impl ExportCollector {
    pub fn new() -> Self {
        Self {
            export_map: BTreeMap::new(),
        }
    }

    pub fn extract_from(&mut self, code: &mut String) {
        self.extract_export_default(code);
        self.extract_export_named(code);
    }

    fn extract_export_default(&mut self, code: &mut String) {
        let re = Regex::new(r"export\s+default\s+([^;]+);").unwrap();
        let mut to_export = None;

        // 提取default导出
        for cap in re.captures_iter(code) {
            let expr = cap[1].trim();
            to_export = Some(expr.to_string());
        }

        // 移除export default语句
        *code = re.replace_all(code, "").to_string();

        // 存入 export map
        if let Some(expr) = to_export {
            self.export_map.insert("default".to_string(), expr);
        }
    }

    fn extract_export_named(&mut self, code: &mut String) {
        let re = Regex::new(r"export\s+\{([^}]+)};").unwrap();
        let mut exports = Vec::new();

        // 提取具名导出
        for cap in re.captures_iter(code) {
            let inner = cap[1].trim();
            let parts = inner.split(',').map(|s| s.trim());

            for part in parts {
                if part.is_empty() {
                    continue;
                }

                if part.contains(" as ") {
                    let subpart: Vec<&str> = part.split(" as ").map(|s| s.trim()).collect();
                    if subpart.len() == 2 {
                        let original = subpart[0];
                        let alias = subpart[1];
                        exports.push((alias.to_string(), original.to_string()));
                    }
                } else {
                    exports.push((part.to_string(), part.to_string()));
                }
            }
        }
        // 移除原始 export 语句
        *code = re.replace_all(code, "").to_string();

        // 存入 export map
        for (k, v) in exports {
            self.export_map.insert(k, v);
        }
    }

    pub fn inject_module_exports(&self, code: &mut String) {
        let export_str = self
            .export_map
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        code.push_str(&format!("\nmodule.exports = {{ {export_str} }};\n"));
    }
}
