use std::env;

fn main() {

    println!("ID the hash!");

    // get input from passed argument

    let args: Vec<String> = env::args().collect();
    let hashz = &args[1];
    let count :i32 = hashz.len().try_into().unwrap();

    // println!("{}", count);
    
    match count {
        32 => println!("md5"),
        40 => println!("sha1"),
        56 => println!("sha224"),
        64 => println!("sha256"),
        96 => println!("sha384"),
        128 => println!("sha512"),
        _ => println!("Un-indetified!"),
    };
}
