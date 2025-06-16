use std::process::Command;

fn main() {
    // Print a message indicating the start of the build script
    println!("cargo:warning=Running build script...");
    // Check if the tests are being run
    // Run the Makefile in the ./test_roms directory
    let status = Command::new("make").arg("-C").arg("test_roms").status();

    match status {
        Ok(status) => {
            if !status.success() {
                panic!("Makefile execution failed");
            }
        }
        Err(e) => {
            panic!("Failed to execute make: {}", e);
        }
    }
}
