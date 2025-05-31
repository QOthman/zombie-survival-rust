# 🧟 Zombie Survival in Rust
A 2D zombie survival game built with [Macroquad](https://github.com/not-fl3/macroquad). Fight for your life as waves of the undead close in during a violent thunderstorm. Armed with limited ammo and surrounded by the undead, can you survive the night?

---

## 🧠 Features
* 🎮 **Smooth Player Controls**: Walk, run, shoot, and reload with fluid animations
* 🧟 **Zombies with Basic AI**: They pursue, attack, and fall when shot — complete with animations
* 🔫 **Ammo System**: Collect ammo, reload manually, and manage resources carefully
* 💀 **Player Death and Respawn**: Hit `R` to restart or `Esc` to quit when you die
* 🌩️ **Immersive Environment**:
  * Realistic rain and splash effects
  * Flashing lightning with timed thunder sounds
  * Atmospheric audio: rain, thunder, gunfire, and zombie growls

---

## 🌐 WASM Features
* 🚀 **Web Deployment**: Runs seamlessly in modern web browsers via WebAssembly
* ⚡ **Near-Native Performance**: Optimized WASM build maintains smooth 60 FPS gameplay
* 📱 **Cross-Platform Compatibility**: Works on desktop, mobile, and tablet browsers
* 🔧 **No Installation Required**: Play instantly without downloading or installing
* 💾 **Small Bundle Size**: Optimized for fast loading times
* 🎵 **Web Audio Support**: Full audio experience including ambient sounds and effects
* 🖱️ **Input Flexibility**: Supports both keyboard and touch controls for mobile devices
* 📦 **Asset Streaming**: Efficient loading of game assets for web deployment

---

## 🖥️ Host Features
* 🔥 **High Performance**: Native compilation delivers maximum frame rates
* 🎧 **Advanced Audio**: Full surround sound support and low-latency audio processing
* 💿 **Local Asset Loading**: Direct file system access for faster resource loading
* 🎮 **Gamepad Support**: Native controller input with haptic feedback
* 🖼️ **High Resolution**: Support for 4K displays and custom resolution scaling
* ⚙️ **System Integration**: Access to system notifications and window management
* 💾 **Save System**: Local file persistence for game progress and settings
* 🔧 **Debug Tools**: Full debugging capabilities and performance profiling
* 🚀 **Multi-Threading**: Leverage multiple CPU cores for enhanced performance

---

## 🚀 Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install)

### Running the Game (Native)
```bash
cargo run --release
```

### Building for WASM
```bash
# Install required tools
cargo install basic-http-server
rustup target add wasm32-unknown-unknown

# Build WASM version
cargo build --release --target wasm32-unknown-unknown

# Serve locally (example)
basic-http-server .
```

---

## 🛠 Built With
* [Macroquad](https://github.com/not-fl3/macroquad) — simple game framework for Rust
* Rust — for performance and safety in a real-time game

---

## 🧪 Controls
* `Arrow Keys` — Move
* `Space` — Shoot
* `R` — Restart after death
* `Esc` — Quit game

---

## 📦 License
This project is open source under the [MIT License](LICENSE).

---

## 🤝 Contributing
Got ideas for new enemy types, mechanics, or optimizations? Feel free to open issues or pull requests!