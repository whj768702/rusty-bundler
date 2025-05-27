use crate::parser_cjs_exports::ExportCollector;

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
        r#"import\s+([a-zA-Z_$][\w$]*)\s*,\s*\{([^}]+)}\s+from\s+['"](.+?)['"];"#,
    )
        .unwrap();
    result = import_default_and_named_re
        .replace_all(
            &result,
            r#"const $1 = require("$3").default; const {$2} = require("$3");"#,
        )
        .to_string();

    // import { a } from './x.js' => const { a } = require('./x.js');
    let import_re = regex::Regex::new(r#"import\s+\{([^}]+)}\s+from\s+['"](.+?)['"];"#).unwrap();
    result = import_re
        .replace_all(&result, r#"const {$1} = require("$2");"#)
        .to_string();

    result
}
