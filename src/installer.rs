use crate::detector::BuildContext;
use dialoguer::Confirm;
use std::process::{Command, Stdio};

pub fn check_toolchain(context: &BuildContext) -> bool {
    let binary = context.required_binary();
    which::which(binary).is_ok()
}

pub fn prompt_and_install(context: &BuildContext) -> anyhow::Result<bool> {
    let name = context.name();
    let binary = context.required_binary();

    println!("\n⚠️  Detected {} context, but '{}' is not installed or not in PATH.", name, binary);

    let (_, installer_cmd) = get_installer_details(context);

    let prompt_msg = format!(
        "Would you like idc to automatically install {} using the following command?\n👉 {}",
        name, installer_cmd
    );

    if Confirm::new().with_prompt(prompt_msg).default(true).interact()? {
        println!("🚀 Starting installation...");
        run_install_script(&installer_cmd)?;
        println!("✅ Installation command finished. You might need to restart your terminal or reload PATH.");
        Ok(true)
    } else {
        println!("❌ Installation skipped. Please install '{}' manually to proceed.", binary);
        Ok(false)
    }
}

fn get_installer_details(context: &BuildContext) -> (String, String) {
    match context {
        BuildContext::Rust => (
            "rustup (Rust toolchain installer)".to_string(),
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y".to_string(),
        ),
        BuildContext::Node => (
            "fnm (Fast Node Manager)".to_string(),
            "curl -fsSL https://fnm.vercel.app/install | bash".to_string(),
        ),
        BuildContext::Python => (
            "uv (Astral Python Package Manager)".to_string(),
            "curl -LsSf https://astral.sh/uv/install.sh | sh".to_string(),
        ),
        BuildContext::Go => {
            // Check platform to offer apt or brew or direct link
            if cfg!(target_os = "macos") {
                ("Homebrew".to_string(), "brew install go".to_string())
            } else if which::which("apt-get").is_ok() {
                ("apt".to_string(), "sudo apt-get update && sudo apt-get install -y golang-go".to_string())
            } else {
                ("Go Website".to_string(), "echo 'Please visit https://go.dev/doc/install to install Go'".to_string())
            }
        }
        BuildContext::Java => (
            "SDKMAN!".to_string(),
            "curl -s \"https://get.sdkman.io\" | bash".to_string(),
        ),
        BuildContext::Dotnet => (
            "dotnet-install script".to_string(),
            "curl -sSL https://dot.net/v1/dotnet-install.sh | bash /dev/stdin".to_string(),
        ),
        BuildContext::CMake => {
            if cfg!(target_os = "macos") {
                ("Homebrew".to_string(), "brew install cmake".to_string())
            } else if which::which("apt-get").is_ok() {
                ("apt".to_string(), "sudo apt-get update && sudo apt-get install -y cmake".to_string())
            } else {
                ("CMake Website".to_string(), "echo 'Please visit https://cmake.org/download/ to install CMake'".to_string())
            }
        }
        BuildContext::Makefile => {
            if cfg!(target_os = "macos") {
                ("Homebrew / Xcode Command Line Tools".to_string(), "brew install make || xcode-select --install".to_string())
            } else if which::which("apt-get").is_ok() {
                ("apt".to_string(), "sudo apt-get update && sudo apt-get install -y build-essential".to_string())
            } else {
                ("GNU Make".to_string(), "echo 'Please install GNU Make using your system package manager'".to_string())
            }
        }
        BuildContext::Deno => (
            "Deno official installer".to_string(),
            "curl -fsSL https://deno.land/install.sh | sh".to_string(),
        ),
    }
}

fn run_install_script(cmd: &str) -> anyhow::Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("Installer command exited with error status: {:?}", status.code());
    }
    Ok(())
}
