fn main() {
    // let a = 10;
    // let b = a; // copying
    // let c = 10;
    let message = String::from("Hello");
    let message_2 = message;

    // Moved
    //println!("{}", message);

    // Moved to message_2
    println!("{}", message_2);

    // Another moving
    print_message(message_2)
}


fn print_message(a: String) {
    println!("{}", a);
    // Now, a is moved
    let c = a;

    // value borrowed here after move
    // println!("{}", a);
    println!("{}", c);
}