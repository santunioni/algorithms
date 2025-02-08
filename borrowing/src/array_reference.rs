fn main() {
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[2];
    // println!("Third element is {}", *num);
    // v.push(4);

    let mut v = vec![1, 2, 3];
    let num = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &mut v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        // v[0] = up;
        *c = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v2 = v1;
    v2.push(4);

    println!("{}", v1[0]);
}
