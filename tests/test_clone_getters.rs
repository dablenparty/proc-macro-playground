use clone_getters_macro::CloneGetters;

#[derive(CloneGetters)]
struct OneFieldStruct {
    f1: usize,
}

#[derive(CloneGetters, Default)]
struct ThreeFieldStruct {
    f1: usize,
    f2: String,
    f3: Vec<u8>,
}

#[test]
fn test_clone_getters() {
    let s = OneFieldStruct { f1: 42 };
    assert_eq!(s.f1_clone(), 42);
}

#[test]
fn test_clone_getters_three_fields() {
    let s = ThreeFieldStruct::default();
    let f1 = s.f1_clone();
    assert_eq!(f1, usize::default());
    let f2 = s.f2_clone();
    assert_eq!(f2, String::default());
    let f3 = s.f3_clone();
    assert_eq!(f3, Vec::<u8>::default());
}
