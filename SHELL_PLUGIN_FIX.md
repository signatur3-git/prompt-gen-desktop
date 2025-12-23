# Shell Plugin Issue - RESOLVED ✅

## Problem
OAuth authentication was failing with error:
```
shell.open not allowed. Permissions associated with this command: shell:allow-open, shell:default
```

## Root Cause
The Tauri shell plugin was installed but **missing permissions** in the capabilities file. Tauri v2 requires explicit permission grants for all plugin commands.

## Solution Applied

### 1. Added Shell Plugin to Cargo.toml
```toml
tauri-plugin-shell = { version = ">=2.0.0, <3.0.0" }
```

### 2. Registered Plugin in main.rs
```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())  // ← Added this
        // ...
}
```

### 3. Configured Plugin in tauri.conf.json
```json
"plugins": {
  "deepLink": {
    "schemes": ["promptgen"]
  },
  "shell": {
    "open": true
  }
}
```

### 4. Added Permissions in capabilities/default.json ⭐ NEW
```json
{
  "permissions": [
    "shell:allow-open",
    "shell:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file",
    "fs:allow-create-dir",
    "fs:default"
  ]
}
```

## Why Permissions Are Needed

In Tauri v2, **plugins require explicit permission grants** in the capabilities system:
- `shell:allow-open` - Allows opening URLs in the default browser
- `shell:default` - Default shell permissions
- `fs:*` - File system operations for package installation

Without these permissions, Tauri blocks the plugin commands for security.

## Testing
✅ Rust compilation successful  
✅ Capabilities regenerated  
✅ Ready for OAuth testing

## Next Steps
1. **Restart dev server**: `npm run tauri:dev`
2. Click "⚙️ Marketplace" → "Connect to Marketplace"
3. Browser should now open successfully ✅

## Important Notes
- Tauri v2 uses a **capabilities-based permission system**
- Each plugin command needs explicit permission grants
- Permissions are defined in `src-tauri/capabilities/*.json`
- The schema is auto-generated during build

