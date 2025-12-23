# 🔍 Diagnostic Script - Run This to Find the Problem

## Step 1: Check if App Runs

```bash
npm run tauri:dev
```

**What should happen**: A desktop window should open after 10-30 seconds

**If it fails**: Copy the error message and tell me

---

## Step 2: Check Console When App Opens

Once the app window is open, press **F12** to open DevTools, then paste this into the Console tab:

```javascript
// Diagnostic Check
console.log('=== TAURI DIAGNOSTIC ===');
console.log('1. Window object:', typeof window);
console.log('2. __TAURI__ exists?', '__TAURI__' in window);
console.log('3. __TAURI__ object:', window.__TAURI__);
console.log('4. invoke function:', typeof window.__TAURI__?.tauri?.invoke);
console.log('5. Location:', window.location.href);
console.log('6. User Agent:', navigator.userAgent);
console.log('========================');
```

**Copy the output** and send it to me.

---

## Step 3: Test File Operations

In the app:
1. Click "📁 Open" button
2. Try to open a package file

**What happens?**
- ✅ Works - file loads
- ❌ Error - (copy the error message)
- ❌ Nothing happens

---

## Step 4: Test Marketplace Connection

In the app:
1. Click "⚙️ Marketplace" button
2. Click "Connect to Marketplace"

**What happens?**
- ✅ Browser opens
- ❌ Error in console - (copy it)
- ❌ Message "OAuth flow requires..." - (copy full message)
- ❌ Different error - (copy it)

---

## Step 5: Check Console for Errors

Look at the Console tab (F12) when the app first loads.

**Do you see any red error messages?**

Copy ALL error messages and send them to me.

---

## Step 6: Check Network Tab

In DevTools, click the "Network" tab.

**Do you see any failed requests (red)?**

If yes, click on them and tell me:
- Request URL
- Status code
- Error message

---

## What I Need from You

Please provide:

1. **Console diagnostic output** (from Step 2)
2. **Any error messages** (red text in console)
3. **What specifically doesn't work** (Steps 3 & 4)
4. **Full error text** (not just "it doesn't work")

With this information, I can fix the actual problem instead of guessing!

---

## Common Issues and Quick Checks

### Is Port 5175 in Use?

```powershell
Get-NetTCPConnection -LocalPort 5175 2>$null
```

If you see output, another process is using the port.

### Is Tauri Actually Running?

```powershell
Get-Process | Where-Object {$_.ProcessName -like "*rpg*"}
```

Should show `rpg-desktop` process if running.

### Did Cargo Build Succeed?

```bash
cd src-tauri
cargo build
```

Look for "Finished" at the end. If you see "error", copy the error.

---

**Please run these diagnostics and send me the results!** Then I can fix the real issue.

