use std::process::Command;

fn main() {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("(Get-CimInstance Win32_ComputerSystemProduct).UUID")
        .output()
        .expect("failed to execute process");
    if !output.stderr.is_empty() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("error: {}", error);
    } else {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("The unique key of your device is: {}", result);
    }
}
