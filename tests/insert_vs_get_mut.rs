use std::collections::HashMap;

pub fn check_map_insert(map: HashMap<String, String>) {}

pub fn check_map_get_mut(map: HashMap<String, String>) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(true, false);
    }
}
