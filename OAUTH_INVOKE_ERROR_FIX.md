# OAuth "invoke" Error - FIXED ✅
If you see "OAuth flow requires Tauri environment", you're running in browser mode - use `npm run tauri:dev` instead!

6. App receives callback
5. Log in on marketplace
4. Browser should open
3. Check console for "Starting OAuth flow"
2. Click "Connect to Marketplace"
1. Click "⚙️ Marketplace"
Then:

```
npm run tauri:dev
```bash

## Testing Command

✅ Ready to test in Tauri app
✅ Detailed logging
✅ Better error messages
✅ Tauri environment check added

## Status

- Check if browser opened and callback received
- Solution: OAuth flow didn't complete (different issue)
### "Missing authorization header"

- File: `src-tauri/capabilities/default.json`
- Solution: Add `shell:allow-open` permission
### "shell.open not allowed"

## Related Issues

```
console.log(typeof open); // Should be 'function'
import { open } from '@tauri-apps/plugin-shell';
```typescript
**Check shell plugin**:

```
console.log('__TAURI__' in window); // Should be true in Tauri
// In browser console
```typescript
**Check if in Tauri environment**:

## Verification

- Tauri API not initialized
- Not running in Tauri environment
- The IPC bridge isn't set up
The `invoke` function is the bridge between frontend and backend. If `invoke` is undefined, it means:

```
Vue Frontend → @tauri-apps/api → IPC → Rust Backend → Plugins
```
**Tauri v2 Architecture**:

## Why This Happens

- Verify permissions in `capabilities/default.json`
- Check that shell plugin is registered in `main.rs`
- Restart the dev server
### 3. If error persists

```
Browser opened successfully
Starting OAuth flow with URL: https://...
```
The app should show:
### 2. Check console for environment

```
npm run dev
# ❌ Wrong - runs in browser only

npm run tauri:dev
# ✅ Correct - runs in Tauri
```bash
### 1. Make sure you're running in Tauri (not browser)

## How to Test

```
}
  throw new Error(`Failed to open browser: ${errorMsg}`);
  const errorMsg = error instanceof Error ? error.message : String(error);
} catch (error) {
  await open(authUrl);
try {
```typescript
### 3. Better Error Handling

```
}
  // ... rest of OAuth flow
  
  }
    throw new Error('OAuth flow requires Tauri environment');
  if (!isTauriEnvironment()) {
  // Check if we're in Tauri environment
async startAuthFlow(): Promise<string> {
```typescript
### 2. Validate Before Using Shell Plugin

```
}
  return typeof window !== 'undefined' && '__TAURI__' in window;
function isTauriEnvironment(): boolean {
```typescript
### 1. Added Tauri Environment Check

## Fix Applied

3. Plugin not properly configured
2. Tauri context not initialized yet
1. Running in browser (not Tauri app)
This typically happens when:

The `open` function from `@tauri-apps/plugin-shell` internally calls `invoke`, but the Tauri API wasn't fully initialized when the OAuth flow was triggered.

## Root Cause

```
at OAuthCallbackHandler.startAuthFlow
TypeError: Cannot read properties of undefined (reading 'invoke')
```
## Error


