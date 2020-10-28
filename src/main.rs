fn main() {    
    if users::get_current_uid() != 0 {
        println!("{}", console::style("Please run this program as root!").red());
        std::process::exit(1);
    }
    println!("Sulfctl")
}