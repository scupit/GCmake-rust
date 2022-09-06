use regex::Regex;


lazy_static! {
  static ref TARGET_NAME_REGEX: Regex = Regex::new("^[-_a-zA-z0-9]+$").unwrap();
}

pub fn is_valid_target_name(name: &str) -> bool {
  return TARGET_NAME_REGEX.is_match(name);
}