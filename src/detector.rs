use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildContext {
    Rust,
    Go,
    Node,
    Python,
    Java,
    Dotnet,
}

impl BuildContext {
    pub fn name(&self) -> &'static str {
        match self {
            BuildContext::Rust => "rust",
            BuildContext::Go => "go",
            BuildContext::Node => "node",
            BuildContext::Python => "python",
            BuildContext::Java => "java",
            BuildContext::Dotnet => "dotnet",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "rust" => Some(BuildContext::Rust),
            "go" => Some(BuildContext::Go),
            "node" => Some(BuildContext::Node),
            "python" => Some(BuildContext::Python),
            "java" => Some(BuildContext::Java),
            "dotnet" => Some(BuildContext::Dotnet),
            _ => None,
        }
    }

    pub fn required_binary(&self) -> &'static str {
        match self {
            BuildContext::Rust => "cargo",
            BuildContext::Go => "go",
            BuildContext::Node => "node",
            BuildContext::Python => "python",
            BuildContext::Java => "javac", // standard compiler check
            BuildContext::Dotnet => "dotnet",
        }
    }
}

pub struct DetectedRoot {
    pub path: PathBuf,
    pub contexts: Vec<BuildContext>,
}

pub fn find_project_root() -> Option<DetectedRoot> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        let contexts = scan_directory(&current);
        // If we found any build files or an idc.yaml, this is the root
        if !contexts.is_empty() || current.join("idc.yaml").exists() {
            return Some(DetectedRoot {
                path: current,
                contexts,
            });
        }
        if !current.pop() {
            break;
        }
    }
    None
}

fn scan_directory(path: &Path) -> Vec<BuildContext> {
    let mut contexts = Vec::new();
    if path.join("Cargo.toml").exists() {
        contexts.push(BuildContext::Rust);
    }
    if path.join("go.mod").exists() {
        contexts.push(BuildContext::Go);
    }
    if path.join("package.json").exists() {
        contexts.push(BuildContext::Node);
    }
    if path.join("pyproject.toml").exists()
        || path.join("requirements.txt").exists()
        || path.join("poetry.lock").exists()
        || path.join("uv.lock").exists()
    {
        contexts.push(BuildContext::Python);
    }
    if path.join("pom.xml").exists()
        || path.join("build.gradle").exists()
        || path.join("build.gradle.kts").exists()
    {
        contexts.push(BuildContext::Java);
    }

    // Check for dotnet .csproj or .sln
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "csproj" || ext == "sln" {
                    contexts.push(BuildContext::Dotnet);
                    break;
                }
            }
        }
    }

    contexts
}
