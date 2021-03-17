use rand::Rng;

pub fn f1() {
    let num = rand::thread_rng().gen_range(-1.0, 1.0);
    println!("{}", num);
}

pub fn f2() {
    println!("f2");
}