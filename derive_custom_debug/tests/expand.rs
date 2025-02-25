use derive_custom_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bmsk: u16,
}

fn main() {}
