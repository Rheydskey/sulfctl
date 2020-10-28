use std::process::Command;
pub fn start_srv (service: String) {
    let optput = Command::new("rc-service")
        .args(&[&service, "start"])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput);
}
pub fn stop_srv (service: String) {
    let optput = Command::new("rc-service")
        .args(&[&service, "stop"])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput);
}
pub fn enable_srv (service: String) {
    let optput = Command::new("rc-update")
        .args(&["add", &service])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput);
}
pub fn disable_srv (service: String) {
    let optput = Command::new("rc-update")
        .args(&["add", &service])
        .spawn()
        .unwrap()
        .stdout;
        println!("{:?}", optput);
}