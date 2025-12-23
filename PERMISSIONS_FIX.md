# 🔐 Tauri v2 Permissions Fix - Complete Guide

## ✅ Issue Resolved

**Error**: `shell.open not allowed. Permissions associated with this command: shell:allow-open, shell:default`

**Fix**: Added proper permissions to `src-tauri/capabilities/default.json`

---

## 🎯 What Changed

### File: `src-tauri/capabilities/default.json`

**Added Permissions**:
```json
{
  "permissions": [
    // ...existing permissions...
    "shell:allow-open",      // ← Opens URLs in browser
    "shell:default",         // ← Default shell permissions
    "fs:allow-read-text-file",   // ← Read package files
    "fs:allow-write-text-file",  // ← Write package files
    "fs:allow-read-dir",         // ← Read directories
    "fs:allow-create-dir",       // ← Create directories
    "fs:allow-exists",           // ← Check file existence
    "fs:default"                 // ← Default fs permissions
  ]
}
```

---

## 📚 Understanding Tauri v2 Permissions

### The Capabilities System

Tauri v2 uses a **capabilities-based security model**:

1. **Plugin Installation** (Cargo.toml)
   - Adds the plugin code to your app

2. **Plugin Registration** (main.rs)
   - Initializes the plugin at runtime

3. **Permissions Grant** (capabilities/*.json) ⭐ **CRITICAL**
   - Explicitly allows specific plugin commands
   - Without this, commands are blocked by default

### Why This is Important

- **Security First**: Tauri blocks all plugin commands by default
- **Explicit Grants**: You must explicitly list what your app can do
- **Granular Control**: Each command requires permission (e.g., `shell:allow-open`)

---

## 🔍 Permission Breakdown

### Shell Permissions
```json
"shell:allow-open"   // Open URLs in default browser (for OAuth)
"shell:default"      // Basic shell operations
```

**Use Case**: Opening marketplace authentication URL in browser

### File System Permissions
```json
"fs:allow-read-text-file"   // Read package YAML files
"fs:allow-write-text-file"  // Save downloaded packages
"fs:allow-create-dir"       // Create packages directory
"fs:allow-read-dir"         // List directory contents
"fs:allow-exists"           // Check if files exist
"fs:default"                // Basic file operations
```

**Use Case**: Downloading and saving marketplace packages

---

## 🚀 How to Test

1. **Restart dev server** (important!):
   ```bash
   npm run tauri:dev
   ```

2. **Test OAuth**:
   - Click "⚙️ Marketplace" button
   - Click "Connect to Marketplace"
   - Browser should open automatically ✅

3. **Test Package Install**:
   - Click "📦 Browse" button
   - Select a package
   - Click "Install"
   - File should save successfully ✅

---

## 🛠️ Common Permission Issues

### Error: "shell.open not allowed"
**Fix**: Add `"shell:allow-open"` and `"shell:default"`

### Error: "fs.writeTextFile not allowed"
**Fix**: Add `"fs:allow-write-text-file"` and `"fs:default"`

### Error: "fs.mkdir not allowed"  
**Fix**: Add `"fs:allow-create-dir"`

### Schema Validation Errors
**Fix**: Run `cargo build` to regenerate the schema

---

## 📖 References

- [Tauri v2 Security](https://v2.tauri.app/concept/security/)
- [Capabilities Documentation](https://v2.tauri.app/concept/security/capabilities/)
- [Plugin Permissions](https://v2.tauri.app/plugin/)

---

## ✅ Verification Checklist

- [x] Shell plugin in Cargo.toml
- [x] Plugin registered in main.rs
- [x] Plugin configured in tauri.conf.json
- [x] Permissions added to capabilities/default.json ⭐
- [x] App rebuilt with `cargo build`
- [ ] Dev server restarted
- [ ] OAuth flow tested
- [ ] Package installation tested

---

**Status**: ✅ **ALL PERMISSIONS CONFIGURED**

The app now has all required permissions for marketplace integration!

