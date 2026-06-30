mod adapters;
mod config;
mod detector;
mod installer;

use clap::{CommandFactory, Parser, Subcommand};
use detector::BuildContext;

#[derive(Parser, Debug)]
#[command(name = "idc", about = "Instant Developer Companion - Unified Build CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Compile or package the codebase")]
    Build {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(about = "Run the main application or development server")]
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(about = "Execute unit or integration tests")]
    Test {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(about = "Generate shell autocompletions")]
    Completion {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Locate project root and check custom commands early
    let project_root = detector::find_project_root();

    if let Some(ref root) = project_root {
        let config_path = root.path.join("idc.yaml");
        if config_path.exists() {
            if let Ok(config) = config::IdcConfig::load(&config_path) {
                if let Some(subcmd) = std::env::args().nth(1) {
                    if let Some(script) = config.commands.get(&subcmd) {
                        let mut full_cmd = script.to_string();
                        for arg in std::env::args().skip(2) {
                            full_cmd.push(' ');
                            full_cmd.push_str(&arg);
                        }
                        println!("🚀 Executing custom task: {}", full_cmd);

                        #[cfg(unix)]
                        let shell = "sh";
                        #[cfg(unix)]
                        let shell_flag = "-c";
                        #[cfg(not(unix))]
                        let shell = "cmd";
                        #[cfg(not(unix))]
                        let shell_flag = "/C";

                        let status = std::process::Command::new(shell)
                            .arg(shell_flag)
                            .arg(&full_cmd)
                            .current_dir(&root.path)
                            .status()?;
                        std::process::exit(status.code().unwrap_or(0));
                    }
                }
            }
        }
    }

    // 2. Parse standard CLI subcommands
    let cli = Cli::parse();

    if let Commands::Completion { shell } = cli.command {
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, "idc", &mut std::io::stdout());
        return Ok(());
    }

    let (verb, args) = match cli.command {
        Commands::Build { args } => ("build", args),
        Commands::Run { args } => ("run", args),
        Commands::Test { args } => ("test", args),
        Commands::Completion { .. } => unreachable!(),
    };

    let project_root = match project_root {
        Some(root) => root,
        None => {
            eprintln!("❌ Error: No build system configuration found in the current or parent directories.");
            std::process::exit(1);
        }
    };

    println!("📂 Detected project root: {}", project_root.path.display());

    let config_path = project_root.path.join("idc.yaml");
    let mut default_context = None;
    if config_path.exists() {
        if let Ok(config) = config::IdcConfig::load(&config_path) {
            default_context = config.default_context.and_then(|name| BuildContext::from_name(&name));
        }
    }

    let selected_context = match default_context {
        Some(ctx) => {
            println!("⚙️  Using configured default build system: {}", ctx.name());
            ctx
        }
        None => {
            if project_root.contexts.is_empty() {
                eprintln!("❌ Error: Could not automatically detect any supported build system in the project root.");
                std::process::exit(1);
            } else if project_root.contexts.len() == 1 {
                let ctx = project_root.contexts[0].clone();
                println!("🔎 Automatically detected build system: {}", ctx.name());
                ctx
            } else {
                let items: Vec<String> = project_root.contexts.iter().map(|c| c.name().to_string()).collect();
                println!("\n📦 Multiple build systems detected:");
                let selection = dialoguer::Select::new()
                    .with_prompt("Please choose which build system to target")
                    .items(&items)
                    .default(0)
                    .interact()?;
                
                let chosen = project_root.contexts[selection].clone();

                if dialoguer::Confirm::new()
                    .with_prompt(format!("Would you like to save '{}' as the default in idc.yaml?", chosen.name()))
                    .default(false)
                    .interact()?
                {
                    let mut config = config::IdcConfig::load(&config_path).unwrap_or(config::IdcConfig {
                        default_context: None,
                        commands: std::collections::HashMap::new(),
                    });
                    config.default_context = Some(chosen.name().to_string());
                    if let Err(e) = config.save(&config_path) {
                        eprintln!("⚠️  Warning: Failed to save configuration: {}", e);
                    } else {
                        println!("✅ Config saved to idc.yaml");
                    }
                }
                chosen
            }
        }
    };

    if !installer::check_toolchain(&selected_context) {
        match installer::prompt_and_install(&selected_context) {
            Ok(true) => {
                if !installer::check_toolchain(&selected_context) {
                    eprintln!("⚠️  Toolchain install command completed, but '{}' is still not found in PATH. You may need to reload your shell.", selected_context.required_binary());
                    std::process::exit(1);
                }
            }
            Ok(false) => {
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("❌ Error during installation: {}", e);
                std::process::exit(1);
            }
        }
    }

    if let Err(e) = adapters::run_context_command(&selected_context, verb, &project_root.path, &args).await {
        eprintln!("❌ Executing command failed: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
