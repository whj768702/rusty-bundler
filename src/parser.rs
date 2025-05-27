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