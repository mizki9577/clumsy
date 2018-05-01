use super::*;
use utils::*;

#[test]
fn translate_abstraction() {
    let a = DeBruijnIndex::from_ast(&new_abstraction("x", new_variable("x")));
    let expected = DeBruijnIndex::Abstraction(box DeBruijnIndex::Index(Some(0)));
    assert_eq!(expected, a);
}
