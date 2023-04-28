fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    println!("===> ~ file: main.rs:5 ~ {:?} ~ v1_iter:", v1_iter);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iteration_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.into_iter();

    // because of into iter i slot the ownership
    // println!("{}", v);

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}
