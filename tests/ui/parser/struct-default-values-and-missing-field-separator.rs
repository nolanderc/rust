//@ run-rustfix
#![allow(dead_code)]

enum E {
    A,
}

struct S {
    field1: i32 = 42,
    field2: E = E::A,
    field3: i32 = 1 + 2,
    field4: i32 = { 1 + 2 },
    field5: E = foo(42),
    field6: E = { foo(42) },
}

struct S1 {
    field1: i32 //~ ERROR expected `,`, or `}`, found `field2`
    field2: E //~ ERROR expected `,`, or `}`, found `field3`
    field3: i32 = 1 + 2,
    field4: i32 = { 1 + 2 },
    field5: E = foo(42),
    field6: E = { foo(42) },
}

struct S2 {
    field1 = i32, //~ ERROR expected `:`, found `=`
    field2; E, //~ ERROR expected `:`, found `;`
}

const fn foo(_: i32) -> E {
    E::A
}

fn main() {}
