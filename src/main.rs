// This is the code to solve the `Collatz conjecture` problem in Math.
fn main() { // fn => function; main => entry point of the whole program
    // mut => mutable, important here, otherwise error would be thrown in compile time
    let mut x: i32 = 27; // Mutable variable binding
    print!("{x}");      // println! is a macro
    while x != 1 { // NOTE: while block in rust doesn't need parentheses around
        if x % 2 == 0 { // NOTE: if block in rust doesn't need parenthese either
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" => {x}");
    }
    println!();
    //Basic syntax: scalar types
    let i8: i8 = 10;
    println!("i8= {i8}");
    let test: f32 = 2_f32;
    println!("test = {test}");
    let str: &str = "foo";
    println!("str = {str}");
    let s: &str = "🤣";
    println!("s = {s}");
    let c: char = '∞';
    println!("c = {c}");
    let f: bool = false;
    println!("f = {f}");
    let t: bool = true;
    println!("t = {t}");

    // below codes showcasing the usage of raw strings
    println!(r"\n"); // print \n in the raw form instead of a line feed
    println!(r#"<a href="link.html">link</a>"#); // use equal amount of # on either side of the
                                                 // quotes for raw strings
    println!("<a href=\"link.html\">link</a>");

    // below codes are byte strings
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);

    // check the value of scalar numbers
    println!("{x}", x = 1_000);
    println!("{x}", x = 10_00);
    println!("{x}", x = 1000);
    println!("{x}", x = 123_i64);
    println!("{x}", x = 123);
}
