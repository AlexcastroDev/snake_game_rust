fn main() {
    let a = 10;
    let b = a;
    let c = 15;
    let sum = add(a, c);
    let sum2 = add(b, c);

    println!("{}", sum);
    println!("{}", sum2);

    // Printing
    p_c();
    p_d();
    p_f();

    // Stack Overflow
    // p_g();
    // p_h();
}


/**
 * @Function add
 */

fn add(x: u32, y: u32) -> u32 {
    let result = x + y;

    result
}


/**
 * Print section
 */

fn p_a() {
    println!("Calling A");
    p_e();
}

fn p_b() {
    println!("Calling B");
}

fn p_c() {
    println!("Calling C");
}

fn p_d() {
    println!("Calling D");
    p_a();
}

fn p_e() {
    println!("Calling E");
}

fn p_f() {
    println!("Calling F");
    p_b();
}


fn p_g() {
    println!("Calling G");
    p_h();
}

fn p_h() {
    println!("Calling H");
    p_g();
}