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
                    let project_name = get_cmake_project_name(&build_dir);
                    let matched = if let Some(ref name) = project_name {
                        executables.iter().find(|p| {
                            p.file_name()
                                .and_then(|n| n.to_str())
                                .map(|n| n == name)
                                .unwrap_or(false)
                        })
                    } else {
                        None
                    };

                    if let Some(bin) = matched {
                        bin.clone()
                    } else {
                        let dir_name = project_root
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        let matched_dir = executables.iter().find(|p| {
                            p.file_name()
                                .and_then(|n| n.to_str())
                                .map(|n| n == dir_name)
                                .unwrap_or(false)
                        });
                        if let Some(bin) = matched_dir {
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
            "clean" => {
                let mut full_args = vec!["--build".to_string(), "build".to_string(), "--target".to_string(), "clean".to_string()];
                full_args.extend(args.iter().cloned());
                println!("Executing: cmake {}", full_args.join(" "));
                let clean_status = Command::new("cmake")
                    .current_dir(project_root)
                    .args(&full_args)
                    .status()?;
                if !clean_status.success() {
                    anyhow::bail!("CMake clean failed.");
                }
                return Ok(());
            }
            "fmt" => {
                if which::which("clang-format").is_ok() {
                    println!("Formatting CMake source files using clang-format...");
                    let mut files = Vec::new();
                    find_cpp_files(project_root, &mut files)?;
                    if !files.is_empty() {
                        let mut format_args = vec!["-i".to_string()];
                        format_args.extend(files.iter().map(|p| p.to_string_lossy().to_string()));
                        let status = Command::new("clang-format")
                            .current_dir(project_root)
                            .args(&format_args)
                            .status()?;
                        if !status.success() {
                            anyhow::bail!("clang-format failed.");
                        }
                    } else {
                        println!("No C++ source files (.cpp, .h, .cc, .cxx, .hpp) found to format.");
                    }
                    return Ok(());
                } else {
                    println!("⚠️ Warning: 'clang-format' is not installed or not in PATH.");
                    return Ok(());
                }
            }
            _ => {}
        }
    }

    if *context == BuildContext::Node && command_verb == "clean" {
        if node_has_script(project_root, "clean") {
            let mgr = detect_js_package_manager(project_root);
            let mut full_args = vec!["run".to_string(), "clean".to_string()];
            full_args.extend(args.iter().cloned());
            println!("Executing: {} {}", mgr, full_args.join(" "));
            let status = Command::new(&mgr)
                .current_dir(project_root)
                .args(&full_args)
                .status()?;
            if !status.success() {
                anyhow::bail!("Node clean script failed.");
            }
        } else {
            println!("No custom 'clean' script found in package.json. Deleting standard build folders...");
            let targets = ["dist", "build", "out", ".next"];
            for t in &targets {
                let dir = project_root.join(t);
                if dir.exists() {
                    println!("Removing directory: {}", dir.display());
                    let _ = std::fs::remove_dir_all(dir);
                }
            }
        }
        return Ok(());
    }

    if *context == BuildContext::Python && command_verb == "clean" {
        println!("Cleaning Python cache and build files...");
        let mut dirs_to_delete = Vec::new();
        find_python_cache_dirs(project_root, &mut dirs_to_delete)?;
        for dir in dirs_to_delete {
            println!("Removing directory: {}", dir.display());
            let _ = std::fs::remove_dir_all(dir);
        }
        return Ok(());
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
                "fmt" => "fmt",
                "clean" => "clean",
                _ => verb,
            };
            ("cargo".to_string(), vec![cmd.to_string()])
        }
        BuildContext::Go => match verb {
            "build" => ("go".to_string(), vec!["build".to_string()]),
            "run" => ("go".to_string(), vec!["run".to_string(), ".".to_string()]),
            "test" => ("go".to_string(), vec!["test".to_string(), "./...".to_string()]),
            "fmt" => ("go".to_string(), vec!["fmt".to_string(), "./...".to_string()]),
            "clean" => ("go".to_string(), vec!["clean".to_string(), "-i".to_string(), "-cache".to_string()]),
            _ => ("go".to_string(), vec![verb.to_string()]),
        },
        BuildContext::Node => {
            let mgr = detect_js_package_manager(root);
            match verb {
                "build" => (mgr, vec!["run".to_string(), "build".to_string()]),
                "run" => (mgr, vec!["start".to_string()]),
                "test" => (mgr, vec!["test".to_string()]),
                "fmt" => {
                    if node_has_script(root, "format") {
                        (mgr, vec!["run".to_string(), "format".to_string()])
                    } else if node_has_script(root, "fmt") {
                        (mgr, vec!["run".to_string(), "fmt".to_string()])
                    } else {
                        ("npx".to_string(), vec!["prettier".to_string(), "--write".to_string(), ".".to_string()])
                    }
                }
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
                    "fmt" => {
                        let fmt_tool = if which::which("ruff").is_ok() { "ruff" } else { "black" };
                        let cmd_args = if fmt_tool == "ruff" { vec!["format".to_string(), ".".to_string()] } else { vec![".".to_string()] };
                        let mut full = vec!["run".to_string(), fmt_tool.to_string()];
                        full.extend(cmd_args);
                        ("uv".to_string(), full)
                    }
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
                    "fmt" => {
                        let fmt_tool = if which::which("ruff").is_ok() { "ruff" } else { "black" };
                        let cmd_args = if fmt_tool == "ruff" { vec!["format".to_string(), ".".to_string()] } else { vec![".".to_string()] };
                        let mut full = vec!["run".to_string(), fmt_tool.to_string()];
                        full.extend(cmd_args);
                        ("poetry".to_string(), full)
                    }
                    _ => ("poetry".to_string(), vec!["run".to_string(), verb.to_string()]),
                }
            } else {
                let python_exe = if which::which("python").is_ok() {
                    "python".to_string()
                } else {
                    "python3".to_string()
                };
                match verb {
                    "build" => (python_exe.clone(), vec!["-m".to_string(), "build".to_string()]),
                    "run" => {
                        let main_file = find_python_main(root);
                        (python_exe.clone(), vec![main_file])
                    }
                    "test" => ("pytest".to_string(), vec![]),
                    "fmt" => {
                        let fmt_tool = if which::which("ruff").is_ok() { "ruff" } else { "black" };
                        let cmd_args = if fmt_tool == "ruff" { vec!["format".to_string(), ".".to_string()] } else { vec![".".to_string()] };
                        (fmt_tool.to_string(), cmd_args)
                    }
                    _ => (python_exe.clone(), vec![verb.to_string()]),
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
                    "fmt" => (gradlew, vec!["format".to_string()]),
                    "clean" => (gradlew, vec!["clean".to_string()]),
                    _ => (gradlew, vec![verb.to_string()]),
                }
            } else {
                match verb {
                    "build" => ("mvn".to_string(), vec!["package".to_string()]),
                    "run" => ("mvn".to_string(), vec!["exec:java".to_string()]),
                    "test" => ("mvn".to_string(), vec!["test".to_string()]),
                    "fmt" => ("mvn".to_string(), vec!["spotless:apply".to_string()]),
                    "clean" => ("mvn".to_string(), vec!["clean".to_string()]),
                    _ => ("mvn".to_string(), vec![verb.to_string()]),
                }
            }
        }
        BuildContext::Dotnet => match verb {
            "build" => ("dotnet".to_string(), vec!["build".to_string()]),
            "run" => ("dotnet".to_string(), vec!["run".to_string()]),
            "test" => ("dotnet".to_string(), vec!["test".to_string()]),
            "fmt" => ("dotnet".to_string(), vec!["format".to_string()]),
            "clean" => ("dotnet".to_string(), vec!["clean".to_string()]),
            _ => ("dotnet".to_string(), vec![verb.to_string()]),
        },
        BuildContext::CMake => {
            ("cmake".to_string(), vec![])
        }
        BuildContext::Makefile => match verb {
            "build" => ("make".to_string(), vec![]),
            "run" => ("make".to_string(), vec!["run".to_string()]),
            "test" => ("make".to_string(), vec!["test".to_string()]),
            "fmt" => ("make".to_string(), vec!["fmt".to_string()]),
            "clean" => ("make".to_string(), vec!["clean".to_string()]),
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
            "fmt" => ("deno".to_string(), vec!["fmt".to_string()]),
            "clean" => {
                if deno_has_task(root, "clean") {
                    ("deno".to_string(), vec!["task".to_string(), "clean".to_string()])
                } else {
                    ("echo".to_string(), vec!["No custom clean task defined in Deno project".to_string()])
                }
            }
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

fn get_cmake_project_name(build_dir: &Path) -> Option<String> {
    let cache_path = build_dir.join("CMakeCache.txt");
    if let Ok(content) = std::fs::read_to_string(cache_path) {
        for line in content.lines() {
            if line.starts_with("CMAKE_PROJECT_NAME:") {
                if let Some(pos) = line.find('=') {
                    let val = line[pos + 1..].trim();
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
            }
        }
    }
    None
}

fn node_has_script(root: &Path, script_name: &str) -> bool {
    let pkg_path = root.join("package.json");
    if let Ok(content) = std::fs::read_to_string(pkg_path) {
        if let Ok(val) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(scripts) = val.get("scripts") {
                return scripts.get(script_name).is_some();
            }
        }
    }
    false
}

fn find_python_cache_dirs(dir: &Path, list: &mut Vec<std::path::PathBuf>) -> anyhow::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "__pycache__" || name == ".pytest_cache" || name == ".ruff_cache" || name == "build" || name == "dist" || name.ends_with(".egg-info") {
                list.push(path);
            } else if !name.starts_with('.') {
                find_python_cache_dirs(&path, list)?;
            }
        }
    }
    Ok(())
}

fn find_cpp_files(dir: &Path, list: &mut Vec<std::path::PathBuf>) -> anyhow::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "build" || name == "CMakeFiles" || name == "_deps" || name.starts_with('.') {
                continue;
            }
            find_cpp_files(&path, list)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "cpp" || ext == "h" || ext == "cc" || ext == "cxx" || ext == "hpp" {
                    list.push(path);
                }
            }
        }
    }
    Ok(())
}
