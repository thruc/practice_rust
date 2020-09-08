fn main() {
    let immut_val = 10;
    let mut mut_val =20;
    println!("{}", mut_val);

    mut_val += immut_val;
    println!("{}", mut_val);

    let v1: u64 = 10;
    let v2 = 10u64;

    println!("{}", v1);
    println!("{}", v2);
}
