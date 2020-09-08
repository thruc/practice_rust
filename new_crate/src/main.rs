use new_crate::module_b::module_c::add;
use new_crate::module_b::module_d::sub;
fn main() {
    let x = add(1, 2);
    println!("{}", x);
    let y = sub(2, 1);
    println!("{}", y);

}