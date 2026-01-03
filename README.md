# Flydigi Vader 4 Pro Linux Support (DInput → XInput)

Basic Linux support for the **Flydigi Vader 4 Pro** controller running in **DInput mode**.

This project parses the controller’s **evdev (DInput)** input and translates it into a **virtual XInput controller**, allowing games and applications that expect XInput to work seamlessly on Linux.

---

## ✨ Features

- 🎮 **DInput → XInput translation**  
  Converts the Flydigi Vader 4 Pro’s evdev inputs into an emulated XInput controller.

- ⚡ **1000Hz polling rate**  
  High-frequency input processing for low latency and responsive gameplay.

- 🎯 **Gyroscope support**  
  Uses the controller’s built-in mouse emulator for gyro input.

- 🔁 **Remappable back buttons**  
  Back buttons can be freely mapped to XInput buttons or axes.

---

## 🐧 Why This Project?

The Flydigi Vader 4 Pro works well on Windows, but Linux support—especially for XInput-only games—is limited.  
This project bridges that gap by:

- Capturing and locking the controller’s evdev input
- Translating it into a virtual XInput device
- Making games and applications see it as a standard Xbox-style controller

---

## 🚧 Status

**Work in progress.**

Planned improvements:
- More flexible input remapping
- Better gyro configuration
- Configuration files / CLI options
- Improved device detection and error handling

---

## 🛠 Requirements

- Linux
- `evdev`
- Access to `/dev/input/event*`
- Virtual input support (`uinput`)

> You may need elevated privileges or custom udev rules.

---

## 📦 Installation & Usage

Instructions will be added once core functionality is stable.

---

## 🤝 Contributing

Contributions, bug reports, and suggestions are welcome.  
If you’re interested in Linux input systems, controllers, or Rust, feel free to open an issue or pull request.

---

## 📄 License

MIT License
