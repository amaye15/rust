

fn main() {
    let str_var: &str = "Hi";
    let int_var: i32 = 2;
    let float_var: f64 = 0.2;
    let mut mut_var = "Hello";
    println!("{}", str_var);
    println!("{}", int_var);
    println!("{}", float_var);
    println!("{}", mut_var);

    mut_var = "Good Morning";
    println!("{}", mut_var);
}
