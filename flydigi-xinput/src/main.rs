use anyhow::{Context, Result};
use evdev::Device;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Find all matching Flydigi/VADER4 devices
    let matches = find_flydigi_devices()?;

    if matches.is_empty() {
        println!("No Flydigi/VADER4 evdev devices found.");
        println!("Tip: run `evtest` and confirm the device name contains Flydigi/VADER4.");
        return Ok(());
    }

    println!("Found {} matching device(s):", matches.len());
    for path in &matches {
        println!("  {:?}", path);
    }

    // Open + grab them
    let mut grabbed = Vec::new();

    for path in &matches {
        let mut dev = Device::open(&path)
            .with_context(|| format!("Failed to open {:?}", path))?;

        let name = dev.name().unwrap_or("<unknown>");
        println!("Opening: {:?}  name={}", path, name);

        dev.grab().with_context(|| format!("Failed to grab {:?}", path))?;
        println!("  -> grabbed!");

        // Keep the device alive by storing it; if it gets dropped, grab is released.
        grabbed.push(dev);
    }

    println!("\nDetecting which device has ABS_BRAKE (controller):");

    for path in &matches {
        let dev = Device::open(path)
            .with_context(|| format!("Failed to open {:?} for inspection", path))?;

        if has_abs_brake(&dev) {
            println!("-> Controller device: {:?}", path);
        } else {
            println!("-> Not controller:    {:?}", path);
        }
    }

    println!("\nAll devices grabbed. Press Enter to release and exit...");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();


    // grabbed devices are dropped here automatically -> grab released
    Ok(())
}

fn find_flydigi_devices() -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for entry in std::fs::read_dir("/dev/input").context("read_dir /dev/input")? {
        let entry = entry?;
        let path = entry.path();

        // only /dev/input/event*
        let fname = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue,
        };
        if !fname.starts_with("event") {
            continue;
        }

        // Try opening; if no permission, skip (we’ll handle perms by running with sudo)
        let dev = match Device::open(&path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let name = dev.name().unwrap_or("").to_lowercase();
        if name.contains("flydigi") || name.contains("vader4") {
            out.push(path);
        }
    }

    out.sort();
    Ok(out)
}

use evdev::AbsoluteAxisCode;

fn has_abs_brake(dev: &Device) -> bool {
    let has_brake = dev
        .supported_absolute_axes()
        .map(|axes| axes.contains(AbsoluteAxisCode::ABS_BRAKE))
        .unwrap_or(false);

    let name = dev.name().unwrap_or("<unknown>");

    if has_brake {
        println!("ABS_BRAKE found on device: {}", name);
    } else {
        println!("No ABS_BRAKE on device: {}", name);
    }

    has_brake
}
