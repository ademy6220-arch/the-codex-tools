use std::fs;
use std::path::Path;

pub fn analyze_process(pid: u32) -> Result<String, std::io::Error> {
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    let raw_content = fs::read(Path::ne
let cmdline = raw_content
        .split(|&b| b == 0)
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect::<Vec<String>>()
        .join(" ");
        
    Ok(cmdline)
}

fn main() {
    let target_pid = 1; // Systemd / Init Prozess
    match analyze_process(target_pid) {
        Ok(cmd) => println!("[+] PID {} wurde gestartet mit: {}", target_pid, cmd),
        Err(_) => println!("[-] PID {} existiert nicht oder Zugriff verweigert.", target_pid),
    }
}
