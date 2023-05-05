use std::ops::Deref;

use smart_pointers::cons_with_mutability;

struct MyBox<T>(T);
struct MyBox2<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let m = MyBox2::new(String::from("rust"));
    hello(&m);
    println!("Hello, world!");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
    cons_with_mutability();
}

fn hello(name: &str) {
    println!("Hello, {}", name)
}

#[test]
fn testing_pointers() {
    let x = 10;
    // let y = MyBox::deref(&x);

    assert_eq!(10, x);
    // assert_eq!(10, *y);
}
