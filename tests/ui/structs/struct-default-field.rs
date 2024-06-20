//@ run-pass

struct S {
    foo: u32 = 123,
    bar: bool = true,
}

fn main() {
    let a = S { foo: 42 };
    assert_eq!(a.foo, 42);
    assert_eq!(a.bar, true);

    let b = S { bar: false };
    assert_eq!(b.foo, 123);
    assert_eq!(b.bar, false);

    let c = S {};
    assert_eq!(c.foo, 123);
    assert_eq!(c.bar, true);
}
