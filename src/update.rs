//! Auto-update checker — checks latest release via git ls-remote.
//! The update check is only triggered when the user explicitly runs `hcom update`.
//! `get_update_info()` is read-only and only surfaces a cached result written by
//! the last `hcom update` invocation.

use crate::paths::{FLAGS_DIR, atomic_write, hcom_path};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn flag_path() -> PathBuf {
    hcom_path(&[FLAGS_DIR, "update_check"])
}

/// Parse version string "x.y.z" into comparable tuple.
fn parse_version(v: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = v.trim().trim_start_matches('v').split('.').collect();
    if parts.len() >= 3 {
        Some((
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
        ))
    } else {
        None
    }
}

/// Spawn a detached background process to fetch latest version and write the cache file.
/// Synchronously fetch the latest version. Tries git ls-remote first (no rate limits),
/// falls back to GitHub API if git is unavailable.
fn fetch_latest_version() -> Option<String> {
    fetch_via_git().or_else(fetch_via_curl)
}

fn fetch_via_git() -> Option<String> {
    let output = std::process::Command::new("git")
        .args([
            "ls-remote",
            "--tags",
            "--sort=version:refname",
            "https://github.com/salemaziel/hcom.git",
        ])
        .env("GIT_HTTP_LOW_SPEED_LIMIT", "1000")
        .env("GIT_HTTP_LOW_SPEED_TIME", "5")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let body = String::from_utf8_lossy(&output.stdout);
    let tag = body
        .lines()
        .rfind(|l| !l.ends_with("^{}"))?
        .split("refs/tags/")
        .nth(1)?
        .trim()
        .to_string();

    let ver = tag.trim_start_matches('v').to_string();
    if ver.is_empty() { None } else { Some(ver) }
}

fn fetch_via_curl() -> Option<String> {
    let output = std::process::Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "5",
            "https://api.github.com/repos/salemaziel/hcom/releases/latest",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let body = String::from_utf8_lossy(&output.stdout);
    let tag = body
        .lines()
        .find(|l| l.contains("\"tag_name\""))?
        .split('"')
        .nth(3)?
        .to_string();

    let ver = tag.trim_start_matches('v').to_string();
    if ver.is_empty() { None } else { Some(ver) }
}

/// Structured update information: current version, latest available, availability, and update command.
#[derive(Clone, Debug)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
    pub available: bool,
    pub cmd: &'static str,
}

/// Synchronously fetch current + latest version info from GitHub.
/// Single source of truth for all update-related logic (fetching, parsing, command selection).
/// Used by `hcom update` command for fresh checks. Also writes the result to the
/// update-check flag file so that subsequent `get_update_info()` calls (which are
/// read-only) can surface the update notice banner without re-fetching.
pub fn fetch_update_info() -> anyhow::Result<UpdateInfo> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let latest =
        fetch_latest_version().ok_or_else(|| anyhow::anyhow!("Could not reach GitHub API"))?;

    let current_parsed = parse_version(&current);
    let latest_parsed = parse_version(&latest);
    let available = current_parsed < latest_parsed;
    let cmd = get_update_cmd();

    // Persist result so the banner appears on subsequent commands without re-fetching.
    let flag = flag_path();
    if available {
        atomic_write(&flag, &latest);
    } else {
        atomic_write(&flag, "");
    }

    Ok(UpdateInfo {
        current,
        latest,
        available,
        cmd,
    })
}

/// Detect install method and return appropriate update command.
fn get_update_cmd() -> &'static str {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => {
            return "curl -fsSL https://github.com/salemaziel/hcom/releases/latest/download/hcom-installer.sh | sh";
        }
    };

    // Resolve symlinks (e.g. Homebrew Cellar, uv shims).
    let resolved = std::fs::canonicalize(&exe).unwrap_or(exe);
    let path_str = resolved.to_string_lossy();

    // Homebrew install (Cellar path on both Apple Silicon and Intel)
    if path_str.contains("/Cellar/") {
        return "brew upgrade hcom";
    }

    // uv tool install
    if path_str.contains("/uv/") || path_str.contains("/.local/share/uv/") {
        return "uv tool upgrade hcom";
    }

    // pip install inside a venv or directly in site-packages/dist-packages
    if path_str.contains("/site-packages/")
        || path_str.contains("/dist-packages/")
        || path_str.contains("/venv/")
    {
        return "pip install -U hcom";
    }

    // pip install --user with maturin `bindings = "bin"` puts the binary in
    // ~/.local/bin, so the executable path alone doesn't reveal pip ownership.
    if is_user_site_pip_install(&resolved) {
        return "pip install -U hcom";
    }

    // Default: curl installer
    "curl -fsSL https://github.com/salemaziel/hcom/releases/latest/download/hcom-installer.sh | sh"
}

fn is_user_site_pip_install(exe: &Path) -> bool {
    let home = match std::env::var_os("HOME") {
        Some(home) => PathBuf::from(home),
        None => return false,
    };

    let local_bin = home.join(".local/bin");
    if !exe.starts_with(&local_bin) {
        return false;
    }

    let local_lib = home.join(".local/lib");
    let Ok(entries) = fs::read_dir(local_lib) else {
        return false;
    };

    for entry in entries.flatten() {
        let py_dir = entry.path();
        if !py_dir.is_dir() {
            continue;
        }

        for pkg_dir_name in ["site-packages", "dist-packages"] {
            let pkg_dir = py_dir.join(pkg_dir_name);
            let Ok(pkg_entries) = fs::read_dir(pkg_dir) else {
                continue;
            };

            if pkg_entries.flatten().any(|pkg| {
                pkg.file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("hcom-") && name.ends_with(".dist-info"))
            }) {
                return true;
            }
        }
    }

    false
}

/// Check for updates from the local cache only. Returns (latest_version, update_cmd) or None.
///
/// Read-only: never spawns a subprocess. The cache is populated only when the
/// user explicitly runs `hcom update` (or `hcom update --check`), which calls
/// `fetch_update_info()` and writes the result to the flag file.
pub fn get_update_info() -> Option<(String, &'static str)> {
    let flag = flag_path();
    let current = env!("CARGO_PKG_VERSION");

    // Read cached result written by the last explicit `hcom update` invocation.
    let latest = fs::read_to_string(&flag).ok()?.trim().to_string();
    if latest.is_empty() {
        return None;
    }

    // Clear stale cache if already on latest (handles manual upgrades).
    if parse_version(current) >= parse_version(&latest) {
        atomic_write(&flag, "");
        return None;
    }

    Some((latest, get_update_cmd()))
}

/// Return update notice string for stderr, or None if up to date.
pub fn get_update_notice() -> Option<String> {
    let (latest, _cmd) = get_update_info()?;
    Some(format!("→ hcom v{latest} available — run `hcom update`"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("0.7.0"), Some((0, 7, 0)));
        assert_eq!(parse_version("v1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_version("bad"), None);
        assert_eq!(parse_version("1.2"), None);
    }

    #[test]
    fn test_version_comparison() {
        assert!(parse_version("0.8.0") > parse_version("0.7.0"));
        assert!(parse_version("1.0.0") > parse_version("0.99.99"));
        assert!(parse_version("0.7.0") == parse_version("0.7.0"));
    }

    #[test]
    fn test_get_update_cmd_default() {
        // Test binary path won't match any known install method, so we expect
        // the curl installer fallback.
        let cmd = get_update_cmd();
        assert!(cmd.contains("curl"), "expected curl fallback, got: {cmd}");
    }

    #[test]
    #[serial]
    fn test_user_site_pip_detection() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path();
        let exe = home.join(".local/bin/hcom");
        let dist_info = home.join(".local/lib/python3.13/site-packages/hcom-0.7.8.dist-info");

        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&dist_info).unwrap();
        std::fs::write(&exe, b"binary").unwrap();

        let old_home = std::env::var_os("HOME");
        unsafe {
            std::env::set_var("HOME", home);
        }
        assert!(is_user_site_pip_install(&exe));
        match old_home {
            Some(val) => unsafe { std::env::set_var("HOME", val) },
            None => unsafe { std::env::remove_var("HOME") },
        }
    }

    #[test]
    #[serial]
    fn test_user_site_pip_detection_ignores_plain_local_bin() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path();
        let exe = home.join(".local/bin/hcom");

        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::write(&exe, b"binary").unwrap();

        let old_home = std::env::var_os("HOME");
        unsafe {
            std::env::set_var("HOME", home);
        }
        assert!(!is_user_site_pip_install(&exe));
        match old_home {
            Some(val) => unsafe { std::env::set_var("HOME", val) },
            None => unsafe { std::env::remove_var("HOME") },
        }
    }
}
