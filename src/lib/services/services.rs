use std::process::Command;
pub fn start_srv (service: String) {
    match Command::new("rc-service").args(&[&service, "start"]).spawn() {
        Ok(_out) => {
            println!("");
        }
        Err(_e) => {
            println!("{}", console::style(&format!("Unable to start service {}", service)).red());
            std::process::exit(1);
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("");
}
pub fn stop_srv (service: String) {
    match Command::new("rc-service").args(&[&service, "stop"]).spawn() {
        Ok(_out) => {
            println!("");
        }
        Err(_e) => {
            println!("{}", console::style(&format!("Unable to stop service {}", service)).red());
            std::process::exit(1);
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("");
}
pub fn enable_srv (service: String) {
    match Command::new("rc-update").args(&["add", &service, "default"]).spawn() {
        Ok(_out) => {
            println!("");
        }
        Err(_e) => {
            println!("{}", console::style(&format!("Unable to enable service {}", service)).red());
            std::process::exit(1);
        }
    }
}
pub fn disable_srv (service: String) {
    match Command::new("rc-update").args(&["del", &service, "default"]).spawn() {
        Ok(_out) => {
            println!("");
        }
        Err(_e) => {
            println!("{}", console::style(&format!("Unable to disable service {}", service)).red());
            std::process::exit(1);
        }
    }
}