use crate::detector::BuildContext;
use std::path::Path;
use std::process::Command;

pub async fn run_context_command(
    context: &BuildContext,
    command_verb: &str,
    project_root: &Path,
    args: &[String],
) -> anyhow::Result<()> {
    let (exe, base_args) = resolve_command(context, command_verb, project_root);

    let mut full_args = base_args;
    full_args.extend(args.iter().cloned());

    println!("Executing: {} {}", exe, full_args.join(" "));

    let status = Command::new(&exe)
        .current_dir(project_root)
        .args(&full_args)
        .status()?;

    if !status.success() {
        anyhow::bail!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}

fn resolve_command(
    context: &BuildContext,
    verb: &str,
    root: &Path,
) -> (String, Vec<String>) {
    match context {
        BuildContext::Rust => {
            let cmd = match verb {
                "build" => "build",
                "run" => "run",
                "test" => "test",
                _ => verb,
            };
            ("cargo".to_string(), vec![cmd.to_string()])
        }
        BuildContext::Go => match verb {
            "build" => ("go".to_string(), vec!["build".to_string()]),
            "run" => ("go".to_string(), vec!["run".to_string(), ".".to_string()]),
            "test" => ("go".to_string(), vec!["test".to_string(), "./...".to_string()]),
            _ => ("go".to_string(), vec![verb.to_string()]),
        },
        BuildContext::Node => {
            let mgr = detect_js_package_manager(root);
            match verb {
                "build" => (mgr, vec!["run".to_string(), "build".to_string()]),
                "run" => (mgr, vec!["start".to_string()]),
                "test" => (mgr, vec!["test".to_string()]),
                _ => (mgr, vec![verb.to_string()]),
            }
        }
        BuildContext::Python => {
            let has_uv = which::which("uv").is_ok();
            let has_poetry = root.join("poetry.lock").exists() || which::which("poetry").is_ok();

            if has_uv {
                match verb {
                    "build" => ("uv".to_string(), vec!["build".to_string()]),
                    "run" => {
                        let main_file = find_python_main(root);
                        ("uv".to_string(), vec!["run".to_string(), "python".to_string(), main_file])
                    }
                    "test" => ("uv".to_string(), vec!["run".to_string(), "pytest".to_string()]),
                    _ => ("uv".to_string(), vec!["run".to_string(), verb.to_string()]),
                }
            } else if has_poetry {
                match verb {
                    "build" => ("poetry".to_string(), vec!["build".to_string()]),
                    "run" => {
                        let main_file = find_python_main(root);
                        ("poetry".to_string(), vec!["run".to_string(), "python".to_string(), main_file])
                    }
                    "test" => ("poetry".to_string(), vec!["run".to_string(), "pytest".to_string()]),
                    _ => ("poetry".to_string(), vec!["run".to_string(), verb.to_string()]),
                }
            } else {
                match verb {
                    "build" => ("python".to_string(), vec!["-m".to_string(), "build".to_string()]),
                    "run" => {
                        let main_file = find_python_main(root);
                        ("python".to_string(), vec![main_file])
                    }
                    "test" => ("pytest".to_string(), vec![]),
                    _ => ("python".to_string(), vec![verb.to_string()]),
                }
            }
        }
        BuildContext::Java => {
            let use_gradle = root.join("build.gradle").exists()
                || root.join("build.gradle.kts").exists();
            if use_gradle {
                let gradlew = if root.join("gradlew").exists() {
                    "./gradlew".to_string()
                } else {
                    "gradle".to_string()
                };
                match verb {
                    "build" => (gradlew, vec!["build".to_string()]),
                    "run" => (gradlew, vec!["run".to_string()]),
                    "test" => (gradlew, vec!["test".to_string()]),
                    _ => (gradlew, vec![verb.to_string()]),
                }
            } else {
                match verb {
                    "build" => ("mvn".to_string(), vec!["package".to_string()]),
                    "run" => ("mvn".to_string(), vec!["exec:java".to_string()]),
                    "test" => ("mvn".to_string(), vec!["test".to_string()]),
                    _ => ("mvn".to_string(), vec![verb.to_string()]),
                }
            }
        }
        BuildContext::Dotnet => match verb {
            "build" => ("dotnet".to_string(), vec!["build".to_string()]),
            "run" => ("dotnet".to_string(), vec!["run".to_string()]),
            "test" => ("dotnet".to_string(), vec!["test".to_string()]),
            _ => ("dotnet".to_string(), vec![verb.to_string()]),
        },
    }
}

fn detect_js_package_manager(root: &Path) -> String {
    if root.join("yarn.lock").exists() {
        "yarn".to_string()
    } else if root.join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if root.join("bun.lockb").exists() {
        "bun".to_string()
    } else {
        "npm".to_string()
    }
}

fn find_python_main(root: &Path) -> String {
    if root.join("main.py").exists() {
        "main.py".to_string()
    } else if root.join("app.py").exists() {
        "app.py".to_string()
    } else {
        // Fallback to searching first python file, or just return main.py
        "main.py".to_string()
    }
}
