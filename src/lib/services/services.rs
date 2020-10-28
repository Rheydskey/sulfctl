use std::process::Command;
pub fn start_srv (service: String) {
    match Command::new("rc-service").args(&[&service, "start"]).spawn().unwrap() {
        Ok(out) => {
            println!("");
        }
        Err(e) => {
            println!("{}", console::style(&format!("Unable to start service {}", service)).red());
            std::process::exit(1);
        }
    }
}
pub fn stop_srv (service: String) {
    match Command::new("rc-service").args(&[&service, "stop"]).spawn().unwrap() {
        Ok(out) => {
            println!("");
        }
        Err(e) => {
            println!("{}", console::style(&format!("Unable to stop service {}", service)).red());
            std::process::exit(1);
        }
    }
}
pub fn enable_srv (service: String) {
    match Command::new("rc-update").args(&["add", &service, "default"]).spawn().unwrap() {
        Ok(out) => {
            println!("");
        }
        Err(e) => {
            println!("{}", console::style(&format!("Unable to enable service {}", service)).red());
            std::process::exit(1);
        }
    }
}
pub fn disable_srv (service: String) {
    match Command::new("rc-update").args(&["del", &service, "default"]).spawn().unwrap() {
        Ok(out) => {
            println!("");
        }
        Err(e) => {
            println!("{}", console::style(&format!("Unable to disable service {}", service)).red());
            std::process::exit(1);
        }
    }
}