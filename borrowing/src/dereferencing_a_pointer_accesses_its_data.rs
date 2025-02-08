fn main() {
    let mut x: Box<i32> = Box::new(1);
    println!("x = {}", x);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    println!("x = {}, a = {}", x, a);
    *x += 1; // *x on the left-side modifies the heap value, so x points to the value 2
    println!("x = {}, a = {}", x, a);

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    println!("x = {}, a = {}, r1 = {}", x, a, r1);
    let b: i32 = **r1; // two dereferences get us to the heap value
    println!("x = {}, a = {}, r1 = {}, b = {}", x, a, r1, b);

    let r2: &i32 = &*x; // r2 points to the heap value directly
    println!("x = {}, a = {}, r1 = {}, b = {}, r2 = {}", x, a, r1, b, r2);
    let c: i32 = *r2; // so only one dereference is needed to read it
    println!(
        "x = {}, a = {}, r1 = {}, b = {}, r2 = {}, c = {}",
        x, a, r1, b, r2, c
    );

    let d = *&*&*&*&*&*&*&*&*&*&c;
    println!(
        "x = {}, a = {}, r1 = {}, b = {}, r2 = {}, c = {}, d = {}",
        x, a, r1, b, r2, c, d
    );

    let x = Box::new(0);
    let y = Box::new(&x);
    let a = ***y;


}

fn move_owner(_v: Box<i32>) {}
