# Quantum Energy Mobile - Cuarzo 4D

Mobile interface for Quantum Energy OS with advanced Cuarzo 4D integration.

## Features

### 🔬 Cuarzo 4D Quantum Core
- **Quantum Energy Predictions**: Advanced algorithms for energy output forecasting
- **Load Balancing**: Quantum annealing-inspired load distribution
- **Auto-Repair**: Quantum error correction for system health
- **Wave Function Evolution**: Real-time quantum state simulation

### 📱 Mobile Integration
- **Battery Monitoring**: Real-time battery status and charging detection
- **Device Sensors**: Temperature, magnetic field, acceleration, gyroscope, light
- **Push Notifications**: Solar storm alerts and system notifications
- **Offline-First**: SQLite database for local data storage

### 🎨 Modern UI/UX
- **Dark Mode**: Eye-friendly dark theme with quantum glow effects
- **Touch Gestures**: Swipe navigation and gesture feedback
- **Responsive Design**: Optimized for mobile and tablet devices
- **Real-time Visualization**: Animated quantum wave functions

## Installation

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+ and Cargo
- Tauri CLI v2

### Setup

1. **Install dependencies**:
```bash
npm install
```

2. **Install Tauri CLI** (if not already installed):
```bash
cargo install tauri-cli --version "^2.0.0"
```

3. **Run in development mode**:
```bash
npm run tauri dev
```

4. **Build for production**:
```bash
npm run tauri build
```

## Project Structure

```
quantum-energy-mobile/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── main.tsx           # Entry point
│   └── index.css          # Global styles
├── src-tauri/             # Rust backend
│   └── src/
│       └── main.rs        # Cuarzo 4D plugin & Tauri commands
├── public/                # Static assets
├── package.json           # Frontend dependencies
├── Cargo.toml             # Rust dependencies
├── tauri.conf.json        # Tauri configuration
├── vite.config.ts         # Vite configuration
└── tsconfig.json          # TypeScript configuration
```

## Rust Commands (Cuarzo 4D)

The following commands are available from JavaScript:

### Quantum State Management
- `get_quantum_state`: Retrieve current quantum energy state
- `update_quantum_state`: Update frequency, amplitude, or phase
- `evolve_quantum_state`: Evolve wave function with potential

### Energy Operations
- `predict_energy`: Predict future energy output
- `balance_system_load`: Distribute load across nodes
- `quantum_fourier_analysis`: Perform QFT on data

### System Health
- `run_auto_repair`: Detect and fix system issues
- `calculate_entanglement_factor`: Measure quantum entanglement

### Mobile Sensors
- `check_solar_storm`: Analyze sensor data for solar storms
- `get_sensor_cache`: Retrieve cached sensor readings

## JavaScript API

### Quantum State
```typescript
import { invoke } from '@tauri-apps/api/core';

// Get current state
const state = await invoke('get_quantum_state');

// Update state
const newState = await invoke('update_quantum_state', {
  frequency: 432.0,
  amplitude: 1.5,
  phase: 0.5
});
```

### Battery Plugin
```typescript
import { battery } from '@tauri-apps/plugin-battery';

const batteryInfo = await battery();
console.log(`Battery: ${batteryInfo.level * 100}%`);
console.log(`Charging: ${batteryInfo.charging}`);
```

### Notifications
```typescript
import { sendNotification, requestPermission } from '@tauri-apps/plugin-notification';

await requestPermission();
await sendNotification({
  title: 'Solar Storm Alert',
  body: 'Protective measures recommended'
});
```

## Configuration

### Tauri Configuration (`tauri.conf.json`)
- App identifier: `com.quantumenergy.mobile`
- Window title: "Quantum Energy Mobile"
- Plugins: battery, notification

### Cargo Dependencies
- `tauri`: v2 with mobile features
- `rusqlite`: SQLite for offline storage
- `serde`: Serialization
- `tokio`: Async runtime

## Mobile Features

### Android
- Battery status monitoring
- Sensor access (accelerometer, gyroscope, magnetometer)
- Push notifications
- Background processing

### iOS
- Battery status monitoring
- Core Motion sensor access
- User notifications
- Background app refresh

## Development

### Adding New Commands

1. Define the command in `src-tauri/src/main.rs`:
```rust
#[tauri::command]
async fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Processed: {}", param))
}
```

2. Register in the invoke handler:
```rust
.invoke_handler(tauri::generate_handler![my_command])
```

3. Call from JavaScript:
```typescript
const result = await invoke('my_command', { param: 'value' });
```

### Styling

The app uses CSS custom properties for theming:
```css
:root {
  --primary-color: #00d4ff;
  --secondary-color: #7c3aed;
  --background-dark: #0a0a1a;
}
```

## Building for Mobile

### Android
```bash
npm run tauri android build
```

### iOS
```bash
npm run tauri ios build
```

## Troubleshooting

### Common Issues

1. **Module not found errors**: Run `npm install` to install dependencies
2. **Rust compilation errors**: Ensure Rust toolchain is up to date
3. **Tauri CLI not found**: Install with `cargo install tauri-cli`

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug npm run tauri dev
```

## License

MIT License - See LICENSE file for details

## Credits

Developed by Giovanny Anthony Corpus Bernal
Quantum Energy OS - Cuarzo 4D Integration
