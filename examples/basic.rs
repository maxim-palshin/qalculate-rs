use qalculate_rs::Qalculate;

fn main() {
    let calc = Qalculate::new().unwrap();
    println!("{}", calc.calculate_string("pi * x = 3").unwrap());
}
