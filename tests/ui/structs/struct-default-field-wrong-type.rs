struct S {
    foo: u32 = false, //~ ERROR mismatched types [E0308]
}

fn main() {
    let s = S{ foo: 32 };
}
