use {
    anyhow::{anyhow, Result},
    glob::glob,
    std::process::Command,
};

/// Globs all of the .slang shader files in the examples directory.
fn get_shader_file_paths() -> Result<Vec<String>> {
    let shaders = glob("src/**/*.slang")?;
    let mut file_names = vec![];
    for file in shaders {
        let file_name = file?.to_str().unwrap().to_string();
        println!("cargo::warning=source: {:?}", file_name);
        println!("cargo::rerun-if-changed={}", file_name);
        file_names.push(file_name);
    }
    Ok(file_names)
}

/// Computes the desired output artifact name based on a shader's file path.
fn get_shader_output_path(shader_source_path: &str) -> String {
    shader_source_path.replace("slang", "spv")
}

fn main() -> Result<()> {
    let file_names = get_shader_file_paths()?;

    for shader_source_path in file_names {
        let output_path = get_shader_output_path(&shader_source_path);
        let results = Command::new("slangc")
            .args(["-o", &output_path, "--", &shader_source_path])
            .output()?;
        if !results.status.success() {
            let error_message = String::from_utf8(results.stderr).unwrap();
            eprintln!("Error while compiling shader!\n\n{}", error_message);
            return Err(anyhow!("Error while compiling shader!"));
        }
    }

    Ok(())
}
