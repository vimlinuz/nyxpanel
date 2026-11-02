//! System integration helpers. Every function here blocks and is meant to be
//! run inside a `Task::perform` on a background executor thread.
//!
//! niri integration lives in [`crate::niri`] and uses the `niri-ipc` crate
//! rather than spawning `niri msg` subprocesses.

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BatteryInfo {
    pub pct: f32,
    pub status: String,
    pub available: bool,
    pub plugged: bool,
}

impl BatteryInfo {
    pub fn is_charging(&self) -> bool {
        self.status == "Charging"
    }
}

// ---------------------------------------------------------------------------
// Low-level command helper
// ---------------------------------------------------------------------------

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

// ---------------------------------------------------------------------------
// Battery (sysfs, same paths as the quickshell BatterySys service)
// ---------------------------------------------------------------------------

pub fn read_battery() -> BatteryInfo {
    const BAT: &str = "/sys/class/power_supply/BAT0";
    const ADP: &str = "/sys/class/power_supply/ADP0";

    let capacity =
        run("cat", &[&format!("{BAT}/capacity")]).and_then(|s| s.trim().parse::<f32>().ok());
    let status = run("cat", &[&format!("{BAT}/status")])
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let adapter_online = run("cat", &[&format!("{ADP}/online")])
        .map(|s| s.trim() == "1")
        .unwrap_or(false);

    let Some(pct) = capacity else {
        return BatteryInfo {
            available: false,
            ..Default::default()
        };
    };

    let charging = status == "Charging";
    let plugged = charging || (adapter_online && (status == "Not charging" || status == "Full"));

    BatteryInfo {
        pct: (pct / 100.0).clamp(0.0, 1.0),
        status,
        available: true,
        plugged,
    }
}
