struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    println!("Hello, world!");
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    mix_up();
}

fn mix_up() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "hello",
        y: "world",
    };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}
