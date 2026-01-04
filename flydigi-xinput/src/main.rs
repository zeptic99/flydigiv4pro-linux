use anyhow::{Context, Result};
use evdev::{AbsoluteAxisCode, Device, EventType, RelativeAxisCode};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

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

    // Open devices once (keep them alive), and classify them
    let mut controller: Option<Device> = None;
    let mut mouse: Option<Device> = None;

    for path in &matches {
        let mut dev = Device::open(path).with_context(|| format!("Failed to open {:?}", path))?;
        let name = dev.name().unwrap_or("<unknown>");
        println!("Opening: {:?}  name={}", path, name);

        let has_brake = has_abs_brake(&dev);
        let is_mouse = is_mouse_like(&dev);

        // Grab immediately so nothing else can read while we decide
        dev.grab().with_context(|| format!("Failed to grab {:?}", path))?;
        println!("  -> grabbed!");

        if has_brake && controller.is_none() {
            println!("  -> Selected as CONTROLLER (ABS_BRAKE present)");
            controller = Some(dev);
        } else if is_mouse && mouse.is_none() {
            println!("  -> Selected as MOUSE (REL_X/REL_Y present)");
            mouse = Some(dev);
        } else {
            println!("  -> Not selected (kept grabbed only while dev lives in this loop)");
            // IMPORTANT: dev will drop here and ungrab automatically.
            // If you want to keep *all* devices grabbed, store them in a Vec instead.
        }
    }

    let mut controller =
        controller.ok_or_else(|| anyhow::anyhow!("No controller device found with ABS_BRAKE"))?;
    let mut mouse = mouse.ok_or_else(|| anyhow::anyhow!("No mouse-like device found (REL_X/REL_Y)"))?;

    println!("\nReady.");
    println!("Controller: {}", controller.name().unwrap_or("<unknown>"));
    println!("Mouse:      {}", mouse.name().unwrap_or("<unknown>"));
    println!("\nPolling at 1000 Hz. Press Ctrl+C to stop.\n");

    // Start with mouse grabbed (locked)
    // (it should already be grabbed, but this makes the intent clear)
    let _ = mouse.grab();
    let mut mouse_is_released = false;

    let poll_interval = Duration::from_micros(1000);
    let mut brake_value: i32 = 0;

    loop {
        let start = Instant::now();


        for ev in controller.fetch_events()? {
            if ev.event_type() == EventType::ABSOLUTE {
                // ev.code() is a number, compare it to ABS_BRAKE's numeric code
                if ev.code() == AbsoluteAxisCode::ABS_BRAKE.0 {
                    brake_value = ev.value();
                }
            }
        }

        if brake_value > 0 {
            // Brake pressed → ungrab mouse
            if !mouse_is_released {
                mouse.ungrab()?;
                mouse_is_released = true;
                println!("Brake pressed → mouse UNGRABBED");
            }
        } else {
            // Brake released → grab mouse again
            if mouse_is_released {
                mouse.grab()?;
                mouse_is_released = false;
                println!("Brake released → mouse GRABBED");
            }
        }

        let elapsed = start.elapsed();
        if elapsed < poll_interval {
            thread::sleep(poll_interval - elapsed);
        }
    }

    fn find_flydigi_devices() -> Result<Vec<PathBuf>> {
        let mut out = Vec::new();

        for entry in std::fs::read_dir("/dev/input").context("read_dir /dev/input")? {
            let entry = entry?;
            let path = entry.path();

            // only /dev/input/event*

            let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };

            if !fname.starts_with("event") {
                continue;
            }

            // Try opening; if no permission, skip (run with sudo)
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

    fn has_abs_brake(dev: &Device) -> bool {
        dev.supported_absolute_axes()
            .map(|axes| axes.contains(AbsoluteAxisCode::ABS_BRAKE))
            .unwrap_or(false)
    }

    fn is_mouse_like(dev: &Device) -> bool {
        dev.supported_relative_axes()
            .map(|axes| axes.contains(RelativeAxisCode::REL_X) && axes.contains(RelativeAxisCode::REL_Y))
            .unwrap_or(false)
    }
}
