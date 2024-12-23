use bevy::reflect::{Reflect, Struct};

#[derive(Debug, Reflect, Default)]
struct TestStruct {
    pub(crate) a: f32,
    pub(crate) b: String,
}

fn main() {
    let test_value = TestStruct {
        a: 1.2,
        b: "Test String".into(),
    };

    let test: Box<dyn Struct> = Box::new(test_value);
    for (name, field) in (0..test.field_len()).map(|i| (test.name_at(i), test.field_at(i))) {
        dbg!((name, field));
    }
}
