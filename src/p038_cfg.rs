#[cfg(target_os = "linux")]
fn function_os() {
    println!("Function On Linux");
}

#[cfg(target_os = "windows")]
fn function_os() {
    println!("Function On Windows");
}

#[cfg(target_os = "macos")]
fn function_os() {println!("Function on Macos"); }

#[cfg(not(target_os = "linux"))]
fn function_not_linux() {
    println!("Function Not On Linux");
}

pub fn run() {
    println!("p038_cfg >>>>>>>>");
    function_os();
    function_not_linux();
    println!("\n");
}
