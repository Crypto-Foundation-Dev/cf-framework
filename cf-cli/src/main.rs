use std::path::Path;
use std::process::{Command, exit};
use std::{env, fs};

const TEMPLATE_REPO: &str = "https://github.com/Crypto-Foundation-Dev/cf-framework.git";
const USAGE: &str = "Usage: cf new <project-name>";

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  let name = match args
    .iter()
    .map(String::as_str)
    .collect::<Vec<_>>()
    .as_slice()
  {
    ["new", name] => name.to_string(),
    _ => {
      eprintln!("{USAGE}");
      exit(1);
    }
  };

  if let Err(message) = new_project(&name) {
    eprintln!("Error: {message}");
    exit(1);
  }

  println!("Project {name} created.\n");
  println!("Next steps:");
  println!("  cd {name}");
  println!("  # fill in DATABASE_* in .env");
  println!("  cargo run -p cf-api");
}

fn new_project(name: &str) -> Result<(), String> {
  if !is_valid_name(name) {
    return Err(format!(
      "invalid project name '{name}': use lowercase letters, digits, and hyphens, starting with a letter"
    ));
  }

  let root = Path::new(name);
  if root.exists() {
    return Err(format!("'{name}' already exists"));
  }

  run_git(&["clone", "-q", "--depth", "1", TEMPLATE_REPO, name])?;

  fs::remove_dir_all(root.join(".git")).map_err(|e| format!("cannot remove .git: {e}"))?;
  fs::remove_dir_all(root.join("cf-cli")).map_err(|e| format!("cannot remove cf-cli: {e}"))?;

  for file in ["Cargo.toml", "README.md", "CLAUDE.md"] {
    let path = root.join(file);
    let content = read(&path)?;
    let kept: Vec<&str> = content
      .lines()
      .filter(|line| !line.contains("cf-cli"))
      .collect();
    write(&path, &(kept.join("\n") + "\n"))?;
  }

  let mut files: Vec<(std::path::PathBuf, bool)> = vec![
    (root.join("Jenkinsfile"), true),
    (root.join("README.md"), false),
    (root.join("cf-api/src/main.rs"), false),
    (root.join("cf-api/README.md"), false),
    (root.join("cf-entity/README.md"), false),
    (root.join("cf-migration/README.md"), false),
    (root.join("cf-repository/README.md"), false),
    (root.join("cf-util/README.md"), false),
  ];
  let k8s = fs::read_dir(root.join("k8s")).map_err(|e| format!("cannot read k8s: {e}"))?;
  for entry in k8s {
    files.push((
      entry.map_err(|e| format!("cannot read k8s: {e}"))?.path(),
      true,
    ));
  }

  for (path, is_deploy_file) in files {
    let content = read(&path)?;
    write(&path, &rename(&content, name, is_deploy_file))?;
  }

  fs::copy(root.join(".env.example"), root.join(".env"))
    .map_err(|e| format!("cannot create .env: {e}"))?;

  run_git(&["-C", name, "init", "-q"])
}

fn is_valid_name(name: &str) -> bool {
  name.starts_with(|c: char| c.is_ascii_lowercase())
    && !name.ends_with('-')
    && name
      .chars()
      .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

// Uses temporary markers so a project name that contains "framework" is not replaced twice.
fn rename(content: &str, name: &str, is_deploy_file: bool) -> String {
  let mut result = content
    .replace(
      "A clean, modular Rust API framework template using Actix-web, SeaORM, and Utoipa.",
      "Built with Actix-web, SeaORM, and Utoipa.",
    )
    .replace("Rust API Framework Template", "\0")
    .replace("API framework template", "\0")
    .replace("CF Framework", "\0")
    .replace("cf-framework", "\0")
    .replace("framework-template", "\0")
    .replace("framework_db", "\u{1}");

  if is_deploy_file {
    result = result.replace("framework", "\0");
  }

  result
    .replace('\0', name)
    .replace('\u{1}', &format!("{}_db", name.replace('-', "_")))
}

fn run_git(args: &[&str]) -> Result<(), String> {
  let status = Command::new("git")
    .args(args)
    .status()
    .map_err(|e| format!("cannot run git: {e}"))?;
  if !status.success() {
    return Err(format!("git {} failed", args.join(" ")));
  }
  Ok(())
}

fn read(path: &Path) -> Result<String, String> {
  fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn write(path: &Path, content: &str) -> Result<(), String> {
  fs::write(path, content).map_err(|e| format!("cannot write {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validates_project_names() {
    assert!(is_valid_name("my-app"));
    assert!(is_valid_name("app2"));
    assert!(!is_valid_name(""));
    assert!(!is_valid_name("MyApp"));
    assert!(!is_valid_name("2app"));
    assert!(!is_valid_name("my_app"));
    assert!(!is_valid_name("my-app-"));
    assert!(!is_valid_name("../app"));
  }

  #[test]
  fn renames_deploy_files() {
    let content =
      "framework-template framework-secret app=framework framework_db api.framework.local";
    assert_eq!(
      rename(content, "my-app", true),
      "my-app my-app-secret app=my-app my_app_db api.my-app.local"
    );
  }

  #[test]
  fn keeps_generic_framework_word_in_docs() {
    assert_eq!(
      rename("# CF Framework\nA Rust API framework", "my-app", false),
      "# my-app\nA Rust API framework"
    );
  }

  #[test]
  fn does_not_rename_twice_when_name_contains_framework() {
    assert_eq!(
      rename("framework-template", "my-framework", true),
      "my-framework"
    );
  }
}
