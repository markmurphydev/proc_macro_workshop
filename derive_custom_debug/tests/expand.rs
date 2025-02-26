use std::fmt::Debug;

// use derive_custom_debug::CustomDebug;

// #[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bmsk: u16,
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("name", &self.name)
            .field("bmsk", &format!("0b{:08b}", &self.bmsk))
            // .field_with("bmsk", |f| write!(f, "0b{:08b}", &self.bmsk))
            .finish()
    }
}

#[test]
fn main() {
    // let f = Field {
    //     name: "hello",
    //     bmsk: 0b0011_1111,
    // };
    // eprintln!("{:?}", f);
    // todo!()
}
