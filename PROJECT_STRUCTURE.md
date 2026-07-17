# DeepSeek Desktop Assistant - Project Structure Analysis

## Overview

This directory contains three related but distinct projects for DeepSeek and MiMo API monitoring on Windows:

1. **DeepSeekDesktopAssistant** (root) - Vue.js + Rust original project
2. **MimoDesktopAssistant** - Fork focused on MiMo platform support
3. **DeepSeekMonitorWindows** - New React + Tauri implementation with enhanced features

---

## Project Comparison

### 1. DeepSeekDesktopAssistant (Root Project)

**Type**: Vue.js + Rust (Tauri 2)  
**Version**: 1.0.0

**Frontend Stack**:
- Vue.js 3 with TypeScript
- Vue Router for navigation
- Pinia for state management
- Chart.js + vue-chartjs for visualizations
- Tailwind CSS for styling
- Vite as build tool

**Backend Stack**:
- Rust with Tauri 2
- Tauri Plugin: Single Instance
- reqwest for HTTP requests
- tokio for async runtime
- chrono for date handling
- winreg for Windows registry
- webview2-com for WebView integration

**Source Structure**:
```
MimoDesktopAssistant/
├── frontend/
│   └── src/
│       ├── App.vue
│       ├── main.ts
│       ├── components/
│       ├── stores/
│       ├── views/
│       ├── types/
│       └── style.css
└── src/ (Rust backend)
    ├── main.rs
    ├── lib.rs
    ├── api.rs
    ├── commands.rs
    ├── login.rs
    ├── models.rs
    ├── storage.rs
    ├── tray.rs
    └── windows.rs
```

**Build Commands**:
- Development: `dev-build.bat`
- Release: `release-build.bat`

---

### 2. DeepSeekMonitorWindows (New Clone)

**Type**: React + Tauri 2  
**Version**: 2.0.0  
**Repository**: https://github.com/Bronzesakon/DeepSeekMonitorWindows

**Frontend Stack**:
- React 18 with TypeScript
- lucide-react for icons
- marked for markdown parsing
- Tauri JS API 2
- Vite 5 as build tool

**Backend Stack**:
- Rust with Tauri 2.11.2
- Tauri Plugins: Log, Single Instance, Dialog, Process, Updater
- Windows DPAPI for credential encryption
- reqwest 0.12 for HTTP requests
- notify-rust for system notifications
- tiny_http for local server
- winreg for Windows registry

**Key Features**:
- ✅ Dual-platform support (DeepSeek + MiMo)
- ✅ Balance query for both platforms
- ✅ Usage statistics with token tracking
- ✅ Windows DPAPI encrypted credential storage
- ✅ Balance threshold notifications
- ✅ Auto-start and auto-refresh options
- ✅ Light/Dark/System theme switching
- ✅ 356px compact panel with system tray
- ✅ Automatic update mechanism

**Source Structure**:
```
DeepSeekMonitorWindows/
├── src/
│   ├── main.tsx
│   ├── components/
│   │   ├── DashboardPanel.tsx
│   │   ├── SettingsPanel.tsx
│   │   └── ModelDetailPanel.tsx
│   ├── styles.css
│   ├── types.ts
│   └── utils.ts
├── src-tauri/
│   └── src/
│       ├── lib.rs
│       └── modules/
│           ├── config.rs (DPAPI encryption)
│           ├── deepseek.rs
│           ├── mimo.rs
│           ├── tray.rs
│           └── types.rs
├── screenshots/
└── package.json
```

**Build Commands**:
- Development: `npm run tauri:dev`
- Build: `npm run tauri build`

---

### 3. Root Project (DeepSeekDesktopAssistant)

**Type**: Unknown (needs investigation)  
**Build System**: Cargo (Rust)

**Build Commands**:
- Development: `dev.bat` (builds frontend + backend debug, then launches)
- Release: `build.bat` (builds frontend + backend release)

**Note**: This appears to be a separate or earlier version of the project. Build scripts reference `frontend/` and `src/` directories.

---

## Technology Comparison

| Aspect | MimoDesktopAssistant | DeepSeekMonitorWindows |
|--------|---------------------|------------------------|
| Frontend | Vue.js 3 + TypeScript | React 18 + TypeScript |
| Styling | Tailwind CSS | Custom CSS |
| State Mgmt | Pinia | React hooks |
| Charts | Chart.js | Native SVG/HTML |
| Rust Backend | Tauri 2 | Tauri 2.11.2 |
| Encryption | ❌ | ✅ Windows DPAPI |
| Platform Support | MiMo only | DeepSeek + MiMo |
| Auto-Update | ❌ | ✅ |
| Notifications | ❌ | ✅ |
| Theme Switching | ❌ | ✅ |

---

## DeepSeekMonitorWindows Key Implementation Details

### Dashboard Panel
- Balance display (DeepSeek or MiMo)
- Usage statistics (tokens, cache hit rate, cost)
- 7-day consumption trend chart
- Model selection (V4 Pro / V4 Flash)

### Settings Panel
- **Account**: API Key configuration, usage token sync
- **General**: Refresh intervals (DeepSeek/MiMo independent)
- **Display**: Light/Dark/System theme
- **Notifications**: Balance threshold, cooldown time
- **About**: Version info, auto-start toggle

### Model Detail Panel
- Daily token breakdown
- Cache hit/miss/output statistics
- Historical consumption data

### Security Implementation
- Windows DPAPI encryption for:
  - DeepSeek API Key
  - Usage Token
  - MiMo account credentials
- Credentials never stored in plaintext config files

---

## Project Relationships

```
DeepSeekDesktopAssistant (Root)
├── frontend/ (Vue.js frontend)
├── src/ (Rust backend)
└── MimoDesktopAssistant/
    ├── frontend/ (Vue.js - MiMo focused fork)
    └── src/ (Rust - MiMo focused fork)
└── DeepSeekMonitorWindows/ (NEW - React + Tauri 2)
    ├── src/ (React components)
    └── src-tauri/ (Rust backend with DPAPI)
```

**Observations**:
1. DeepSeekMonitorWindows is the most feature-complete implementation
2. MimoDesktopAssistant is a focused fork for MiMo platform
3. Root project may be the original or a hybrid of both
4. All projects use Tauri 2 for desktop packaging

---

## File Paths

- Main project: `E:\DeepSeekDesktopAssistant\`
- MimoDesktopAssistant: `E:\DeepSeekDesktopAssistant\MimoDesktopAssistant\`
- DeepSeekMonitorWindows: `E:\DeepSeekDesktopAssistant\DeepSeekMonitorWindows\`

---

## Related GitHub Repositories

- https://github.com/Bronzesakon/DeepSeekMonitorWindows (cloned here)
- https://github.com/JayHome137/DeepSeekMonitor (original inspiration)
- https://github.com/Joyi-code/DeepSeekMonitorWindows (Windows adaptation)
- https://github.com/HaoyueQin/DeepSeekMonitorWindows (MiMo support reference)
- https://github.com/KerryChia/DeepSeek_Monitor_for_Windows (DPAPI encryption reference)

---

*Generated: 2026-07-15*
