use rusthero_rustlab2019::convert;

fn main() {
    let input = "22164224441".to_owned();
    let steps = 40;
    let length = (0..steps).fold(input,|x,_|convert(x)).len();
    println!("{}",length);
}