# OAuth Flow Analysis: Desktop App vs Web App

## Current Problem

The desktop app is using the **same OAuth flow as the web app** (PKCE without client_secret), but we have a **Rust backend** that could handle OAuth more securely.

**Current Error**: `{"error":"Missing authorization header"}`

This suggests we're either:
1. Not getting a token at all
2. Not sending the token correctly
3. Trying to access endpoints that don't exist

---

## OAuth Flow Options for Desktop Apps

### Option 1: Current (PKCE, No Secret) ❌
**What it is**: Pure frontend OAuth, like SPA

**Pros**:
- No secrets to manage
- Simple implementation
- Same as web app

**Cons**:
- **Not leveraging Rust backend**
- Less secure (token in frontend)
- Can't use confidential client features
- May have callback issues with deep linking

**Current Status**: Partially implemented, having issues

---

### Option 2: Confidential Client with Backend Proxy ✅ **RECOMMENDED**
**What it is**: Rust backend handles OAuth, frontend just displays UI

**Flow**:
1. Frontend: User clicks "Connect"
2. Rust: Generate PKCE, open browser
3. Browser: User authenticates on marketplace
4. Callback: Deep link to Rust (`promptgen://oauth/callback?code=...`)
5. Rust: Exchange code + client_secret for token
6. Rust: Store token securely
7. Frontend: Gets "authenticated" signal, no token exposure

**Pros**:
- ✅ **Client secret never exposed** (stored in Rust)
- ✅ **Token stored securely** (Rust backend, not localStorage)
- ✅ Leverages Tauri architecture properly
- ✅ More secure than web app
- ✅ Frontend doesn't handle sensitive data

**Cons**:
- More complex implementation
- Need Tauri commands for OAuth

**Implementation**:
```rust
// src-tauri/src/commands/oauth.rs
#[tauri::command]
async fn start_oauth_flow() -> Result<String, String> {
    // Generate PKCE
    // Open browser
    // Wait for callback (deep link)
    // Exchange code + secret for token
    // Store token in secure storage
    // Return success
}

#[tauri::command]
async fn get_auth_status() -> Result<AuthStatus, String> {
    // Check if we have valid token
}
```

---

### Option 3: Device Flow 🤔
**What it is**: User enters code on website

**Flow**:
1. App: Shows code (e.g., "ABC-123")
2. User: Goes to website, enters code
3. App: Polls for approval
4. App: Gets token when approved

**Pros**:
- No deep linking needed
- Works on any device
- Simple UX

**Cons**:
- Extra step for user
- Requires marketplace to support device flow
- Not typical for desktop apps

---

## What the Web App Does

Looking at prompt-gen-web (likely):
```typescript
// Web app (SPA) - NO backend
1. User clicks login
2. Frontend: Redirects to marketplace OAuth
3. Marketplace: User authenticates
4. Redirect: Back to http://localhost:5173/callback?code=...
5. Frontend: Exchanges code for token (PKCE, no secret)
6. Frontend: Stores token in localStorage
7. Frontend: Makes API calls with token
```

**Why this works for web**: 
- No backend to store secrets
- PKCE protects against code interception
- Browser handles redirects natively

**Why this doesn't work great for desktop**:
- We HAVE a backend (Rust)
- Deep linking is more complex than HTTP redirect
- We can store secrets securely

---

## Recommended Solution: Backend OAuth Proxy

### Architecture

```
Frontend (Vue)          Rust Backend          Marketplace
    │                       │                      │
    ├─► start_oauth() ─────►│                      │
    │                       │                      │
    │                       ├─► Open Browser ─────►│
    │                       │                      │
    │                       │     Deep Link        │
    │                       ◄────────────────────  │
    │                       │  (code + state)      │
    │                       │                      │
    │                       ├─► Exchange Code ────►│
    │                       │   (code + secret)    │
    │                       │                      │
    │                       ◄──── Token ───────────│
    │                       │                      │
    │                       │ Store in             │
    │                       │ secure storage       │
    │                       │                      │
    ◄──── success ──────────┤                      │
    │                       │                      │
    │                       │                      │
    ├─► API call ──────────►│                      │
    │                       │                      │
    │                       ├─► API + Token ──────►│
    │                       │                      │
    │                       ◄──── Response ────────│
    │                       │                      │
    ◄──── data ─────────────┤                      │
```

### Benefits
1. **Token never touches frontend** (most secure)
2. **Client secret in Rust only** (can't be extracted)
3. **Rust handles all OAuth** (proper separation)
4. **Frontend is just UI** (as it should be)

---

## What Needs to Change

### 1. Add Client Secret to Rust Backend
```rust
// src-tauri/src/config/marketplace.rs
const CLIENT_SECRET: &str = env!("MARKETPLACE_CLIENT_SECRET");
// Or load from secure keychain
```

### 2. Move OAuth to Rust Commands
```rust
#[tauri::command]
async fn oauth_login() -> Result<(), String>

#[tauri::command]
async fn oauth_logout() -> Result<(), String>

#[tauri::command]
async fn get_auth_status() -> Result<AuthStatus, String>
```

### 3. Proxy Marketplace API Calls
```rust
#[tauri::command]
async fn marketplace_api(
    endpoint: String,
    method: String,
    body: Option<String>
) -> Result<String, String> {
    // Get stored token
    // Make request with token
    // Return response
}
```

### 4. Register Deep Link Handler
```rust
// In main.rs
.on_deep_link(|app, url| {
    // Parse callback URL
    // Extract code + state
    // Complete OAuth flow
    // Emit event to frontend
})
```

---

## Decision: What to Do?

### Quick Fix (Keep Current Approach)
- Fix the callback handling
- Fix the token storage
- Keep PKCE-only flow
- **Time**: 1-2 hours
- **Security**: Medium (same as web)

### Proper Fix (Backend OAuth Proxy) ✅ **RECOMMENDED**
- Implement Rust OAuth commands
- Move OAuth logic to backend
- Use client secret securely
- **Time**: 4-6 hours
- **Security**: High (proper desktop app)

---

## My Recommendation

**Implement the Backend OAuth Proxy** because:

1. ✅ We have a Rust backend - use it properly
2. ✅ More secure than web app
3. ✅ Proper separation of concerns
4. ✅ Token never exposed to frontend
5. ✅ Can use confidential client features
6. ✅ Follows desktop app best practices

The current implementation is trying to be a web app when it's actually a desktop app with a proper backend. Let's leverage that!

---

## Next Steps

1. **Document the decision**
2. **Create Rust OAuth module**
3. **Implement deep link handler**
4. **Create Tauri commands**
5. **Update frontend to use commands**
6. **Test OAuth flow end-to-end**

Would you like me to implement the Backend OAuth Proxy approach?

