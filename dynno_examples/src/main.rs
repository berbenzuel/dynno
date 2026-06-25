use crate::foo::Foo;

mod foo;

fn main() {
    let foo = Foo {bar: 1};

    foo.write_definition()

}
