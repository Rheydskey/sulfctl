use std::process::Command;
pub fn start_srv (service: String) {
    let optput = Command::new("rc-service")
        .args(&[&service, "start"])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput.unwrap())
}
pub fn stop_srv (service: String) {
    let optput = Command::new("rc-service")
        .args(&[&service, "start"])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput.unwrap())
}
pub fn enable_srv (service: String) {
    let optput = Command::new("rc-update")
        .args(&["add", &service])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput.unwrap())
}
pub fn disable_srv (service: String) {
    let optput = Command::new("rc-update")
        .args(&["add", &service])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput.unwrap())
}