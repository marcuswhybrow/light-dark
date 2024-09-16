fn main() {
    if terminal_light::luma().is_ok_and(|l| l >= 0.5) {
        println!("light");
        std::process::exit(0)
    } else {
        println!("dark");
        std::process::exit(1)
    }
}
