use std::process::Command;

fn main() {
    let output = Command::new("docker")
        .args(&["ps"])
        .output()
        .expect("Failed to execute 'docker ps' command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);

        let lines: Vec<&str> = output_str.lines().collect();

        if lines.len() > 1 {
            println!("Running Docker Containers:");
            for line in lines.iter() {
                println!("{}", line);
            }
        } else {
            println!("No running containers found.");
        }
    } else {
        eprintln!("Error running 'docker ps' command:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
