fn main() {
    longest_dangling_str()
}

fn dangling_reference() {
    let r;
    {
        let x = 5;
        // r = &x;
        // we will be using the r after the inner scope has ended/closed thus r will be pointing to a empty memory
        r = x;
    }
    println!("r: {}", r);
}

fn longest_dangling_str() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest_error(x: &str, y: &str) -> &str {
    // we don't know which borrowed value (x or y) will be returned from the longest function, as it's in the a condition
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // we don't know which borrowed value (x or y) will be returned from the longest function, as it's in the a condition
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
