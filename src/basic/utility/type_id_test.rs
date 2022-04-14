use std::any::{Any, TypeId};

fn is_string(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}
pub fn main() {
    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}