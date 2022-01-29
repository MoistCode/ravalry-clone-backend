use regex::Regex;

pub fn sanitize_string(string: String) -> String {
    let re = Regex::new(r"[^A-Za-z]+").unwrap();
    re.replace_all(&string, "").to_string()
}