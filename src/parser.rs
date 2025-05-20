use std::fmt::format;

use regex::Regex;

pub fn parse_imports(code: &str) -> Vec<String> {
    let mut imports = Vec::new();

    let import_re = Regex::new(r#"import\s+.*?["'](.*?)["'];?"#).unwrap();
    let require_re = Regex::new(r#"require\(["'](.*?)["']\)"#).unwrap();

    for cap in import_re
        .captures_iter(code)
        .chain(require_re.captures_iter(code))
    {
        let mut dep = cap[1].to_string();

        if !dep.starts_with("./") && !dep.starts_with("../") && !dep.starts_with("/") {
            dep = format!("./{}", dep);
        }
        imports.push(dep);
    }

    imports
}

pub fn transform_es_to_commonjs(code:&str)->String {
    let mut transformed = code.to_string();

    // 转换import xxx from './mod.js'
    let import_regex = regex::Regex::new(r#"import\s+(\w+)\s+from\s+['"](.+)['"]"#).unwrap();
    transformed = import_regex.replace_all(&transformed, r#"const $1 = require("$2");"#).to_string();

    // 转换export default xxx
    let export_regex = regex::Regex::new(r#"export\s+default\s+(.*);"#).unwrap();
    transformed = export_regex.replace_all(&transformed, r#"module.exports = $1;"#).to_string();

    transformed
}
