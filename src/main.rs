use rusthero_rustlab2019::convert;

fn main() {
    let mut s = "22164224441".to_owned();
    for _ in 0..40 {
        s = convert(s);
    }
    println!("{}",s);
}