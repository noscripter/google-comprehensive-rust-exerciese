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

    // basic syntax: compound types
    let a = [20, 30, 40];
    println!("{a:?}");

    let a2 = [0; 3];
    println!("{a2:?}");

    let a3 = [3; 4];
    println!("{a3:?}");

    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");

    // showcasing compound types of tuples
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    println!("{t:?}");
    println!("{t:#?}"); // use '#?' to pretty format the output

    let empty_tuple = (); // this is an empty tuple, and also called an unit type
                          // consider it void in other programming languages
                          // it's used to indicate that a function or expression has no return
                          // value
    println!("{empty_tuple:?}");
    println!("{empty_tuple:#?}");

    // references
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("ref_x: {}", *ref_x);
    let ref_x_count_ones: u32 = ref_x.count_ones();
    println!("ref_x_count_ones = {}", ref_x_count_ones);
    println!("x: {}", x); // NOTE: important! This line of print value x can only be used after all the above print
                          // code, otherwise it would leads to compile time error:
                          // cannot borrow `x` as immutable because it is also borrowed as mutable: mutable borrow occurs here

    // dangling references
    let ref_y: &i32;
    let y: i32 = 10;
    ref_y = &y;
    println!("ref_y: {ref_y}");
    // dangling references demo
    //let ref_y: &i32;
    //{
        //let y: i32 = 10;
        //ref_y = &y;
    //}
    //println!("ref_y: {ref_y}");

    // slices
    let arr: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("arr: {arr:?}");
    println!("arr: {arr:#?}");
}
