use std::process::Command;

fn main() {
    let command = Command::new("defaults").args(["read", "-g", "AppleInterfaceStyle"]).output().expect("Failed to execute process");

    let result = String::from_utf8_lossy(&command.stdout);

    let mut kitten = Command::new("kitty");
    kitten.arg("+kitten");
    kitten.arg("themes");
    kitten.arg("--reload-in=all");

    if result.trim() == "Dark" {
        kitten.arg("Adventure Time");
    } else {
        kitten.arg("fairyfloss");
    }

    kitten.spawn().expect("Failed to change the theme");
}
