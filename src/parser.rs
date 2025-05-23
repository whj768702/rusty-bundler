use crate::parser_cjs_exports::ExportCollector;
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

pub fn transform_es_to_commonjs(code: &str) -> String {
    let mut result = code.to_string();

    let mut collector = ExportCollector::new();
    collector.extract_from(&mut result);
    collector.inject_module_exports(&mut result);

    // import a from './x.js' => const a = require('./x.js').default;
    let import_default_only_re =
        regex::Regex::new(r#"import\s+([a-zA-Z_$][\w$]*)\s+from\s+['"](.+?)['"];"#).unwrap();
    result = import_default_only_re
        .replace_all(&result, r#"const $1 = require("$2").default;"#)
        .to_string();

    // import a, {b} from './x.js' => const a = require('./x.js').default; const { b } = require('./x.js');
    let import_default_and_named_re = regex::Regex::new(
        r#"import\s+([a-zA-Z_$][\w$]*)\s*,\s*\{([^}]+)\}\s+from\s+['"](.+?)['"];"#,
    )
    .unwrap();
    result = import_default_and_named_re
        .replace_all(
            &result,
            r#"const $1 = require("$3").default; const {$2} = require("$3");"#,
        )
        .to_string();

    // import { a } from './x.js' => const { a } = require('./x.js');
    let import_re = regex::Regex::new(r#"import\s+\{([^}]+)\}\s+from\s+['"](.+?)['"];"#).unwrap();
    result = import_re
        .replace_all(&result, r#"const {$1} = require("$2");"#)
        .to_string();

    // // export default xxx; => module.exports = xxx;
    // let export_default_re = regex::Regex::new(r#"export\s+default\s+([a-zA-Z_$]*)"#).unwrap();
    // result = export_default_re
    //     .replace_all(&result, r#"module.exports = {default: $1}"#)
    //     .to_string();

    // // export { a, b } => module.exports = { a, b }
    // let export_named_re = regex::Regex::new(r#"export\s+\{\s*([^}]+?)\s*\};"#).unwrap();
    // result = export_named_re
    //     .replace_all(&result, r#"module.exports = { $1 };"#)
    //     .to_string();

    // // export const a = ... => const a = ...
    // let export_named_re = regex::Regex::new(r#"export\s+(const|let|var|function)\s+"#).unwrap();
    // result = export_named_re.replace_all(&result, r#"$1 "#).to_string();

    result
}
