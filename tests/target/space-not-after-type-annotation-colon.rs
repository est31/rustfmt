// rustfmt-space_before_type_annotation: true
// rustfmt-space_after_type_annotation_colon: false

static staticVar :i32 = 42;
const constVar :i32 = 42;
fn foo(paramVar :i32) {
    let localVar :i32 = 42;
}
struct S {
    fieldVar :i32,
}
fn f() {
    S { fieldVar: 42 }
}
