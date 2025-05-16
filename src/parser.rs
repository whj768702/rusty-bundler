use regex::Regex;

pub fn parse_imports(code: &str)->Vec<String>{
    let mut result = Vec::new();
    let re = Regex::new(r#"(?m)^\s*import\s.*?["'](.*?)["'];?"#).unwrap();

    for cap in re.captures_iter(code) {
        if let Some(m) = cap.get(1) {
            result.push(m.as_str().to_string());
        }
    }
    result
}
