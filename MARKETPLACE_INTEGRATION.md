# Marketplace Integration Plan - Desktop Application

**Date:** 2025-12-23  
**Status:** Ready to Implement  
**Estimated Time:** 10-13 hours

⚠️ **IMPORTANT:** This plan is for the **DESKTOP APPLICATION** (prompt-gen-desktop, Tauri app).  
📋 **For the WEB SPA:** See the prompt-gen-web repository (different OAuth flow needed)

**Note:** The marketplace OAuth server is already deployed and working. We successfully integrated the web SPA in a separate repository. Now we're bringing the same functionality to the desktop app.

---

## 🎯 Goal

Connect the **prompt-gen-desktop** Tauri application to the deployed marketplace using OAuth 2.0 authentication flow, enabling users to:
- Sign in to marketplace from desktop app
- Browse marketplace packages
- Install packages directly
- Publish packages from desktop app

---

## ✅ What's Already Done (Marketplace Side)

- ✅ OAuth 2.0 authorization server with PKCE
- ✅ Authorization endpoint: `POST /api/v1/oauth/authorize`
- ✅ Token exchange endpoint: `POST /api/v1/oauth/token`
- ✅ Token revocation endpoint: `POST /api/v1/oauth/revoke`
- ✅ User registration and login working
- ✅ Deployed to production on Railway

---

## 📋 What Needs to Be Done (prompt-gen-desktop Side)

### Phase 1: OAuth Client Setup (2 hours)

#### 1.1 Register OAuth Client in Marketplace Database
**Action:** Add prompt-gen-desktop as an OAuth client

```sql
-- Run on marketplace database
INSERT INTO oauth_clients (id, client_id, client_name, redirect_uris, created_at)
VALUES (
  gen_random_uuid(),
  'prompt-gen-desktop',
  'Prompt Gen Desktop',
  ARRAY['http://localhost:51234/oauth/callback', 'promptgen://oauth/callback'],
  NOW()
);
```

**Notes:**
- Use custom URL scheme `promptgen://` for production
- Use `http://localhost:51234` for development
- Multiple redirect URIs for flexibility

#### 1.2 Store OAuth Configuration
**File:** `prompt-gen-desktop/src/config/marketplace.config.ts`

```typescript
export const marketplaceConfig = {
  authorizationEndpoint: 'https://your-marketplace.railway.app/api/v1/oauth/authorize',
  tokenEndpoint: 'https://your-marketplace.railway.app/api/v1/oauth/token',
  revokeEndpoint: 'https://your-marketplace.railway.app/api/v1/oauth/revoke',
  clientId: 'prompt-gen-desktop',
  redirectUri: 'http://localhost:51234/oauth/callback', // or promptgen://oauth/callback
  scope: 'read:packages write:packages'
};
```

---

### Phase 2: Implement PKCE Flow (3-4 hours)

#### 2.1 Create OAuth Service
**File:** `prompt-gen-desktop/src/services/oauth.service.ts`

```typescript
import crypto from 'crypto';

export class OAuthService {
  // Generate PKCE challenge
  generatePKCEChallenge(): { verifier: string; challenge: string } {
    const verifier = crypto.randomBytes(32).toString('base64url');
    const challenge = crypto
      .createHash('sha256')
      .update(verifier)
      .digest('base64url');
    return { verifier, challenge };
  }

  // Build authorization URL
  buildAuthorizationUrl(codeChallenge: string): string {
    const params = new URLSearchParams({
      client_id: marketplaceConfig.clientId,
      redirect_uri: marketplaceConfig.redirectUri,
      response_type: 'code',
      code_challenge: codeChallenge,
      code_challenge_method: 'S256',
      scope: marketplaceConfig.scope,
      state: crypto.randomBytes(16).toString('hex') // CSRF protection
    });
    return `${marketplaceConfig.authorizationEndpoint}?${params}`;
  }

  // Exchange code for token
  async exchangeCodeForToken(code: string, verifier: string): Promise<string> {
    const response = await fetch(marketplaceConfig.tokenEndpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        grant_type: 'authorization_code',
        code,
        redirect_uri: marketplaceConfig.redirectUri,
        client_id: marketplaceConfig.clientId,
        code_verifier: verifier
      })
    });

    const data = await response.json();
    return data.access_token;
  }

  // Revoke token
  async revokeToken(token: string): Promise<void> {
    await fetch(marketplaceConfig.revokeEndpoint, {
      method: 'POST',
      headers: { 
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      }
    });
  }
}
```

#### 2.2 Create OAuth Callback Server (Tauri Backend)
**File:** `prompt-gen-desktop/src/services/oauth-callback-server.ts`

**Note:** In Tauri, you can use a temporary HTTP server or register a custom protocol handler.

```typescript
import express from 'express';
import { Server } from 'http';

export class OAuthCallbackServer {
  private server: Server | null = null;
  private port = 51234;

  // Start temporary HTTP server to receive callback
  async waitForCallback(): Promise<{ code: string; state: string }> {
    return new Promise((resolve, reject) => {
      const app = express();
      
      app.get('/oauth/callback', (req, res) => {
        const { code, state, error } = req.query;
        
        if (error) {
          res.send('<h1>Authorization Failed</h1><p>You can close this window.</p>');
          reject(new Error(error as string));
        } else {
          res.send('<h1>Authorization Successful!</h1><p>You can close this window.</p>');
          resolve({ 
            code: code as string, 
            state: state as string 
          });
        }
        
        // Close server after handling callback
        setTimeout(() => this.stop(), 1000);
      });

      this.server = app.listen(this.port, () => {
        console.log(`OAuth callback server listening on port ${this.port}`);
      });
    });
  }

  stop() {
    if (this.server) {
      this.server.close();
      this.server = null;
    }
  }
}
```

---

### Phase 3: UI Integration (2-3 hours)

#### 3.1 Add "Connect to Marketplace" Button
**File:** `prompt-gen-web/src/components/Settings.vue` (or similar)

```vue
<template>
  <div class="marketplace-settings">
    <h3>Marketplace Connection</h3>
    
    <div v-if="!isConnected">
      <p>Connect to the Prompt Gen Marketplace to browse and install packages.</p>
      <button @click="connectToMarketplace" :disabled="isConnecting">
        {{ isConnecting ? 'Connecting...' : 'Connect to Marketplace' }}
      </button>
    </div>
    
    <div v-else>
      <p>✅ Connected to Marketplace</p>
      <button @click="disconnectFromMarketplace">Disconnect</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { shell } from 'electron';
import { OAuthService } from '../services/oauth.service';
import { OAuthCallbackServer } from '../services/oauth-callback-server';
import { tokenStore } from '../stores/token.store';

const oauthService = new OAuthService();
const callbackServer = new OAuthCallbackServer();

const isConnected = ref(!!tokenStore.getAccessToken());
const isConnecting = ref(false);

async function connectToMarketplace() {
  try {
    isConnecting.value = true;
    
    // Generate PKCE challenge
    const { verifier, challenge } = oauthService.generatePKCEChallenge();
    
    // Store verifier temporarily
    sessionStorage.setItem('pkce_verifier', verifier);
    
    // Start callback server
    const callbackPromise = callbackServer.waitForCallback();
    
    // Open browser for authorization
    const authUrl = oauthService.buildAuthorizationUrl(challenge);
    shell.openExternal(authUrl);
    
    // Wait for callback
    const { code, state } = await callbackPromise;
    
    // Exchange code for token
    const accessToken = await oauthService.exchangeCodeForToken(code, verifier);
    
    // Store token
    tokenStore.setAccessToken(accessToken);
    isConnected.value = true;
    
    alert('Successfully connected to Marketplace!');
  } catch (error) {
    console.error('OAuth error:', error);
    alert('Failed to connect to Marketplace');
  } finally {
    isConnecting.value = false;
    sessionStorage.removeItem('pkce_verifier');
  }
}

async function disconnectFromMarketplace() {
  const token = tokenStore.getAccessToken();
  if (token) {
    await oauthService.revokeToken(token);
    tokenStore.clearAccessToken();
    isConnected.value = false;
  }
}
</script>
```

#### 3.2 Create Token Store
**File:** `prompt-gen-web/src/stores/token.store.ts`

```typescript
// Simple token storage (use encrypted storage in production)
class TokenStore {
  private tokenKey = 'marketplace_access_token';

  setAccessToken(token: string) {
    localStorage.setItem(this.tokenKey, token);
  }

  getAccessToken(): string | null {
    return localStorage.getItem(this.tokenKey);
  }

  clearAccessToken() {
    localStorage.removeItem(this.tokenKey);
  }
}

export const tokenStore = new TokenStore();
```

---

### Phase 4: Marketplace API Integration (3-4 hours)

#### 4.1 Create Marketplace Client
**File:** `prompt-gen-web/src/services/marketplace-client.ts`

```typescript
import { tokenStore } from '../stores/token.store';

export class MarketplaceClient {
  private baseUrl = 'https://your-marketplace.railway.app/api/v1';

  private async fetch(path: string, options: RequestInit = {}) {
    const token = tokenStore.getAccessToken();
    const headers = {
      'Content-Type': 'application/json',
      ...(token && { Authorization: `Bearer ${token}` }),
      ...options.headers
    };

    const response = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      headers
    });

    if (!response.ok) {
      throw new Error(`API error: ${response.statusText}`);
    }

    return response.json();
  }

  // Browse packages
  async searchPackages(query?: string) {
    const params = query ? `?search=${encodeURIComponent(query)}` : '';
    return this.fetch(`/packages${params}`);
  }

  // Get package details
  async getPackage(namespace: string, name: string) {
    return this.fetch(`/packages/${namespace}/${name}`);
  }

  // Download package
  async downloadPackage(namespace: string, name: string, version: string) {
    const token = tokenStore.getAccessToken();
    const response = await fetch(
      `${this.baseUrl}/packages/${namespace}/${name}/${version}/download`,
      { headers: token ? { Authorization: `Bearer ${token}` } : {} }
    );
    return response.text(); // YAML content
  }

  // Publish package
  async publishPackage(formData: FormData) {
    const token = tokenStore.getAccessToken();
    if (!token) throw new Error('Not authenticated');

    const response = await fetch(`${this.baseUrl}/packages`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${token}` },
      body: formData
    });

    if (!response.ok) {
      throw new Error(`Publish failed: ${response.statusText}`);
    }

    return response.json();
  }

  // Get user's packages
  async getMyPackages() {
    return this.fetch('/packages/me');
  }
}

export const marketplaceClient = new MarketplaceClient();
```

#### 4.2 Add Package Browser UI
**File:** `prompt-gen-web/src/components/MarketplaceBrowser.vue`

```vue
<template>
  <div class="marketplace-browser">
    <h2>Marketplace Packages</h2>
    
    <input 
      v-model="searchQuery" 
      @input="searchPackages" 
      placeholder="Search packages..."
    />
    
    <div class="package-list">
      <div 
        v-for="pkg in packages" 
        :key="`${pkg.namespace}.${pkg.name}`"
        class="package-item"
      >
        <h3>{{ pkg.namespace }}.{{ pkg.name }}</h3>
        <p>{{ pkg.description }}</p>
        <button @click="installPackage(pkg)">Install</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { marketplaceClient } from '../services/marketplace-client';

const searchQuery = ref('');
const packages = ref([]);

async function searchPackages() {
  packages.value = await marketplaceClient.searchPackages(searchQuery.value);
}

async function installPackage(pkg: any) {
  const latestVersion = pkg.versions[0]?.version;
  if (!latestVersion) return;
  
  const yaml = await marketplaceClient.downloadPackage(
    pkg.namespace, 
    pkg.name, 
    latestVersion
  );
  
  // Save to local packages directory
  // TODO: Implement local package storage
  console.log('Downloaded package:', yaml);
  alert(`Installed ${pkg.namespace}.${pkg.name}@${latestVersion}`);
}

// Load packages on mount
searchPackages();
</script>
```

---

## 🚀 Implementation Checklist

### Setup
- [ ] Register OAuth client in marketplace database
- [ ] Add marketplace config to prompt-gen-web
- [ ] Install required dependencies (express for callback server)

### OAuth Flow
- [ ] Implement OAuthService (PKCE generation, URL building, token exchange)
- [ ] Implement OAuthCallbackServer (temporary HTTP server)
- [ ] Implement TokenStore (secure token storage)
- [ ] Add "Connect to Marketplace" button in settings
- [ ] Test authorization flow end-to-end

### Marketplace Integration
- [ ] Implement MarketplaceClient (API wrapper)
- [ ] Add package browser UI
- [ ] Add package installation logic
- [ ] Add package publishing UI
- [ ] Test all marketplace operations

### Polish
- [ ] Error handling for network failures
- [ ] Loading states for async operations
- [ ] Token refresh logic (if implementing refresh tokens)
- [ ] Disconnect/logout functionality
- [ ] User feedback (success/error messages)

---

## 🔒 Security Notes

1. **PKCE is mandatory** - Never skip code_verifier validation
2. **State parameter** - Use for CSRF protection
3. **Token storage** - Consider encrypting access tokens on disk
4. **HTTPS in production** - Never use http:// for OAuth in production
5. **Token expiration** - Implement token refresh or re-authentication

---

## 📊 Testing Plan

### Manual Testing
1. Click "Connect to Marketplace"
2. Browser opens with authorization page
3. User approves authorization
4. Desktop app receives token
5. Browse packages works
6. Download package works
7. Publish package works
8. Disconnect works

### Edge Cases
- User denies authorization
- Network timeout during token exchange
- Invalid/expired token
- Callback server port already in use
- Multiple authorization attempts

---

## 📚 References

- OAuth 2.0 RFC: https://datatracker.ietf.org/doc/html/rfc6749
- PKCE RFC: https://datatracker.ietf.org/doc/html/rfc7636
- Marketplace OAuth docs: (your marketplace documentation)

---

## 🎯 Success Criteria

✅ User can connect desktop app to marketplace  
✅ User can browse marketplace packages  
✅ User can install packages from marketplace  
✅ User can publish packages to marketplace  
✅ User can disconnect from marketplace  
✅ All operations work with proper authentication  

---

**Status:** Ready to implement  
**Next Step:** Register OAuth client and start Phase 1


