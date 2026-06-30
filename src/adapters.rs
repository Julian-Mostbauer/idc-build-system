use crate::detector::BuildContext;
use std::path::Path;
use std::process::Command;

pub async fn run_context_command(
    context: &BuildContext,
    command_verb: &str,
    project_root: &Path,
    args: &[String],
) -> anyhow::Result<()> {
    if *context == BuildContext::CMake {
        match command_verb {
            "build" => {
                let build_dir = project_root.join("build");
                if !build_dir.exists() {
                    println!("Build directory 'build/' missing. Configuring with cmake -B build...");
                    let configure_status = Command::new("cmake")
                        .current_dir(project_root)
                        .args(&["-B", "build"])
                        .status()?;
                    if !configure_status.success() {
                        anyhow::bail!("CMake configuration failed.");
                    }
                }
                let mut full_args = vec!["--build".to_string(), "build".to_string()];
                full_args.extend(args.iter().cloned());
                println!("Executing: cmake {}", full_args.join(" "));
                let build_status = Command::new("cmake")
                    .current_dir(project_root)
                    .args(&full_args)
                    .status()?;
                if !build_status.success() {
                    anyhow::bail!("CMake build failed.");
                }
                return Ok(());
            }
            "test" => {
                let mut full_args = vec!["--test-dir".to_string(), "build".to_string()];
                full_args.extend(args.iter().cloned());
                println!("Executing: ctest {}", full_args.join(" "));
                let test_status = Command::new("ctest")
                    .current_dir(project_root)
                    .args(&full_args)
                    .status()?;
                if !test_status.success() {
                    anyhow::bail!("ctest failed.");
                }
                return Ok(());
            }
            "run" => {
                // Ensure built
                let build_status = Command::new("cmake")
                    .current_dir(project_root)
                    .args(&["--build", "build"])
                    .status()?;
                if !build_status.success() {
                    anyhow::bail!("CMake build failed.");
                }

                let build_dir = project_root.join("build");
                let mut executables = Vec::new();
                scan_for_executables(&build_dir, &mut executables)?;

                if executables.is_empty() {
                    anyhow::bail!("No compiled executables found in build/ directory.");
                }

                let target_bin = if executables.len() == 1 {
                    executables[0].clone()
                } else {
                    let dir_name = project_root
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    let matched = executables.iter().find(|p| {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n == dir_name)
                            .unwrap_or(false)
                    });
                    if let Some(bin) = matched {
                        bin.clone()
                    } else {
                        let items: Vec<String> = executables
                            .iter()
                            .map(|p| p.strip_prefix(project_root).unwrap_or(p).display().to_string())
                            .collect();
                        println!("\nMultiple compiled binaries found:");
                        let selection = dialoguer::Select::new()
                            .with_prompt("Please choose which binary to run")
                            .items(&items)
                            .default(0)
                            .interact()?;
                        executables[selection].clone()
                    }
                };

                println!("Executing: {}", target_bin.display());
                let run_status = Command::new(&target_bin)
                    .current_dir(project_root)
                    .args(args)
                    .status()?;
                if !run_status.success() {
                    anyhow::bail!("Binary exited with failure: {:?}", run_status.code());
                }
                return Ok(());
            }
            _ => {}
        }
    }

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

fn scan_for_executables(dir: &Path, list: &mut Vec<std::path::PathBuf>) -> anyhow::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "CMakeFiles" || name == "_deps" || name.starts_with('.') {
                continue;
            }
            scan_for_executables(&path, list)?;
        } else if path.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = path.metadata() {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        list.push(path);
                    }
                }
            }
            #[cfg(not(unix))]
            {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext == "exe" {
                        list.push(path);
                    }
                }
            }
        }
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
        BuildContext::CMake => {
            // Already handled customly in run_context_command, but resolved for safety
            ("cmake".to_string(), vec![])
        }
        BuildContext::Makefile => match verb {
            "build" => ("make".to_string(), vec![]),
            "run" => ("make".to_string(), vec!["run".to_string()]),
            "test" => ("make".to_string(), vec!["test".to_string()]),
            _ => ("make".to_string(), vec![verb.to_string()]),
        },
        BuildContext::Deno => match verb {
            "build" => {
                if deno_has_task(root, "build") {
                    ("deno".to_string(), vec!["task".to_string(), "build".to_string()])
                } else {
                    ("echo".to_string(), vec!["No Deno build task defined".to_string()])
                }
            }
            "run" => {
                if deno_has_task(root, "start") {
                    ("deno".to_string(), vec!["task".to_string(), "start".to_string()])
                } else if deno_has_task(root, "run") {
                    ("deno".to_string(), vec!["task".to_string(), "run".to_string()])
                } else {
                    let entrypoint = find_deno_entrypoint(root);
                    ("deno".to_string(), vec!["run".to_string(), "--allow-all".to_string(), entrypoint])
                }
            }
            "test" => ("deno".to_string(), vec!["test".to_string(), "--allow-all".to_string()]),
            _ => ("deno".to_string(), vec![verb.to_string()]),
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
        "main.py".to_string()
    }
}

fn deno_has_task(root: &Path, task_name: &str) -> bool {
    let check_file = |p: std::path::PathBuf| -> Option<bool> {
        let content = std::fs::read_to_string(p).ok()?;
        let val: serde_yaml::Value = serde_yaml::from_str(&content).ok()?;
        let tasks = val.get("tasks")?;
        tasks.get(task_name).map(|_| true)
    };
    check_file(root.join("deno.json"))
        .or_else(|| check_file(root.join("deno.jsonc")))
        .unwrap_or(false)
}

fn find_deno_entrypoint(root: &Path) -> String {
    let candidates = ["main.ts", "main.js", "index.ts", "index.js", "mod.ts"];
    for c in &candidates {
        if root.join(c).exists() {
            return c.to_string();
        }
    }
    "main.ts".to_string()
}
