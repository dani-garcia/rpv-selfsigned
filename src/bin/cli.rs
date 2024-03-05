use rpv_selfsigned::network_test;

fn main() {
    let out = network_test().unwrap();
    println!("{}", out);
}
