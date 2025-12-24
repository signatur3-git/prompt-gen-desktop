/**
 * OAuth Service for Marketplace Authentication
 *
 * Implements OAuth 2.0 with PKCE (Proof Key for Code Exchange) flow.
 *
 * PKCE Flow:
 * 1. Generate code_verifier (random string)
 * 2. Generate code_challenge (SHA256 hash of verifier)
 * 3. Send challenge to authorization server
 * 4. Exchange authorization code + verifier for access token
 */

import marketplaceConfig from '../config/marketplace.config';
import { invoke } from '@tauri-apps/api/core';

function hasTauriInvoke(): boolean {
  // We consider ourselves in a Tauri-capable environment if `invoke` exists.
  // Do NOT rely on window.__TAURI__ globals (can be absent depending on WebView injection).
  return typeof invoke === 'function';
}

export interface PKCEChallenge {
  verifier: string;
  challenge: string;
}

export class OAuthService {
  /**
   * Generate PKCE code verifier and challenge
   *
   * Uses Web Crypto API (browser-compatible)
   */
  async generatePKCEChallenge(): Promise<PKCEChallenge> {
    // Generate random verifier
    const array = new Uint8Array(32);
    crypto.getRandomValues(array);
    const verifier = this.base64UrlEncode(array);

    // Generate challenge (SHA256 of verifier)
    const encoder = new TextEncoder();
    const data = encoder.encode(verifier);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const challenge = this.base64UrlEncode(new Uint8Array(hashBuffer));

    return { verifier, challenge };
  }

  /**
   * Build authorization URL for OAuth flow
   */
  buildAuthorizationUrl(codeChallenge: string, state: string): string {
    const params = new URLSearchParams({
      client_id: marketplaceConfig.clientId,
      redirect_uri: marketplaceConfig.redirectUri,
      response_type: 'code',
      code_challenge: codeChallenge,
      code_challenge_method: 'S256',
      scope: marketplaceConfig.scope,
      state,
    });

    return `${marketplaceConfig.authorizationEndpoint}?${params.toString()}`;
  }

  /**
   * Exchange authorization code for access token
   */
  async exchangeCodeForToken(code: string, verifier: string): Promise<string> {
    console.log('[oauth] tokenEndpoint=', marketplaceConfig.tokenEndpoint);
    console.log('[oauth] redirectUri used for token exchange=', marketplaceConfig.redirectUri);

    // Prefer Rust-side exchange in Tauri builds (more reliable in Windows release environments).
    if (hasTauriInvoke()) {
      try {
        const token = await invoke<string>('oauth_exchange_code', {
          tokenEndpoint: marketplaceConfig.tokenEndpoint,
          clientId: marketplaceConfig.clientId,
          code,
          codeVerifier: verifier,
          redirectUri: marketplaceConfig.redirectUri,
        });
        if (token) return token;
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        console.error('[oauth] Rust token exchange failed:', msg);

        // Best-effort: ask the Rust backend to probe the endpoint so we can distinguish
        // TLS/proxy/network failures from application-level OAuth errors.
        let probe: string | null = null;
        try {
          const p = await invoke<any>('oauth_probe', { tokenEndpoint: marketplaceConfig.tokenEndpoint });
          probe = JSON.stringify(p);
        } catch (probeErr) {
          probe = `probe failed: ${probeErr instanceof Error ? probeErr.message : String(probeErr)}`;
        }

        throw new Error(
          `Token exchange failed (Rust backend): ${msg}\n` +
          `tokenEndpoint=${marketplaceConfig.tokenEndpoint}\n` +
          `probe=${probe}`
        );
      }
    }

    const body = new URLSearchParams({
      grant_type: 'authorization_code',
      code,
      redirect_uri: marketplaceConfig.redirectUri,
      client_id: marketplaceConfig.clientId,
      code_verifier: verifier,
    });

    // Note: don't log the full `code` or `code_verifier`.
    console.log('[oauth] token request params=', {
      grant_type: 'authorization_code',
      client_id: marketplaceConfig.clientId,
      redirect_uri: marketplaceConfig.redirectUri,
      code_len: code?.length ?? 0,
      verifier_len: verifier?.length ?? 0,
    });

    let response: Response;
    try {
      response = await fetch(marketplaceConfig.tokenEndpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: body.toString(),
      });
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('[oauth] Token exchange network error:', {
        tokenEndpoint: marketplaceConfig.tokenEndpoint,
        message: msg,
      });
      throw new Error(
        `Token exchange failed: ${msg}\n` +
        `tokenEndpoint=${marketplaceConfig.tokenEndpoint}\n` +
        `Common causes: network/TLS failure, wrong URL, server unreachable, blocked by firewall/proxy.`
      );
    }

    const contentType = response.headers.get('content-type') || '';
    const responseBody = await response.text();

    if (!response.ok) {
      throw new Error(
        `Token exchange failed (HTTP ${response.status} ${response.statusText}, content-type: ${contentType}): ${responseBody}`
      );
    }

    // Try JSON first, but fall back to urlencoded just in case.
    let accessToken: string | undefined;
    if (contentType.includes('application/json')) {
      try {
        const data = JSON.parse(responseBody);
        accessToken = data.access_token;
      } catch {
        // fall through
      }
    }

    if (!accessToken) {
      const parsed = new URLSearchParams(responseBody);
      accessToken = parsed.get('access_token') || undefined;
    }

    if (!accessToken) {
      throw new Error(`Token exchange succeeded but no access_token was found in response: ${responseBody}`);
    }

    return accessToken;
  }

  /**
   * Revoke access token (logout)
   */
  async revokeToken(token: string): Promise<void> {
    try {
      await fetch(marketplaceConfig.revokeEndpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
      });
    } catch (error) {
      console.error('Token revocation failed:', error);
      // Don't throw - revocation failure shouldn't block logout
    }
  }

  /**
   * Generate random state for CSRF protection
   */
  generateState(): string {
    const array = new Uint8Array(16);
    crypto.getRandomValues(array);
    return this.base64UrlEncode(array);
  }

  /**
   * Base64URL encode (URL-safe base64 without padding)
   */
  private base64UrlEncode(buffer: Uint8Array): string {
    const base64 = btoa(String.fromCharCode(...buffer));
    return base64
      .replace(/\+/g, '-')
      .replace(/\//g, '_')
      .replace(/=/g, '');
  }
}

// Export singleton instance
export const oauthService = new OAuthService();

