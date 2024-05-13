use std::process::Command;

const BLUEPRINT: &'static str = "./resources/glushkovizer.blp";
const UI: &'static str = "./resources/glushkovizer.ui";

fn main() -> Result<(), std::io::Error> {
    Command::new("blueprint-compiler")
        .args(["compile", "--output", UI, BLUEPRINT])
        .spawn()
        .expect(r#"Failed to start "blueprint-compiler""#)
        .wait()?;

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "glushkovizer.gresource",
    );
    Ok(())
}
