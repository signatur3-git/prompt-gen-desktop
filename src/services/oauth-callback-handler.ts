/**
 * OAuth Callback Handler
 *
 * Handles OAuth callbacks in Tauri environment.
 * Uses Tauri's deep linking feature to receive OAuth callbacks.
 */

import { open } from '@tauri-apps/plugin-shell';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { oauthService } from './oauth.service';
import { tokenStore } from '../stores/token.store';
import marketplaceConfig from '../config/marketplace.config';

export interface OAuthCallbackData {
  code: string;
  state: string;
}

export class OAuthCallbackHandler {
  private resolveCallback: ((data: OAuthCallbackData) => void) | null = null;
  private rejectCallback: ((error: Error) => void) | null = null;

  constructor() {
    // Set up deep link listener when handler is created
    this.setupDeepLinkListener();
  }

  /**
   * Set up listener for OAuth callback deep links
   */
  private async setupDeepLinkListener(): Promise<void> {
    try {
      await listen<string>('oauth-callback', (event) => {
        console.log('Received OAuth callback:', event.payload);
        this.handleCallback(event.payload);
      });
    } catch (error) {
      console.error('Failed to set up deep link listener:', error);
    }
  }

  /**
   * Start OAuth flow
   * Opens browser and waits for callback
   */
  async startAuthFlow(): Promise<string> {
    try {
      const { verifier, challenge } = await oauthService.generatePKCEChallenge();
      const state = oauthService.generateState();

      // DEV: start loopback callback server in Rust and use its redirect URI.
      // PROD: use deep-link redirect URI on macOS/Linux, but on Windows deep links may spawn
      //       a second instance. For packaged Windows builds, prefer loopback too.
      let originalRedirectUri: string | null = null;

      const shouldUseLoopback =
        import.meta.env.DEV ||
        // In production Windows builds, deep link callbacks may start a new instance.
        (import.meta.env.PROD && navigator.userAgent.toLowerCase().includes('windows'));

      if (shouldUseLoopback) {
        originalRedirectUri = marketplaceConfig.redirectUri;
        const resp = await invoke<{ redirectUri: string }>('oauth_start_loopback');
        marketplaceConfig.redirectUri = resp.redirectUri;
      }

      const authUrl = oauthService.buildAuthorizationUrl(challenge, state);
      console.log('[oauth] Starting OAuth flow with URL:', authUrl);

      try {
        await open(authUrl);
        console.log('[oauth] Browser opened successfully');
      } catch (error) {
        console.error('[oauth] Failed to open browser:', error);
        const errorMsg = error instanceof Error ? error.message : String(error);
        throw new Error(
          `Failed to open browser: ${errorMsg}\n\n` +
          `Please ensure:\n` +
          `1. You're running the Tauri app (npm run tauri:dev)\n` +
          `2. Shell plugin permissions are configured\n` +
          `3. Try restarting the dev server`
        );
      }

      console.log('[oauth] Waiting for callback...');
      const callbackData = await this.waitForCallback();

      // IMPORTANT: Do NOT restore redirectUri before the token exchange.
      // OAuth servers typically require the token request's redirect_uri to match exactly
      // what was used during authorization.

      console.log('[oauth] Callback received, validating state...');
      if (callbackData.state !== state) {
        throw new Error('Invalid state parameter (CSRF protection)');
      }

      console.log('[oauth] State ok, exchanging code for token...');
      const accessToken = await oauthService.exchangeCodeForToken(callbackData.code, verifier);

      // Restore configured redirect URI (defensive)
      if (shouldUseLoopback && originalRedirectUri) {
        marketplaceConfig.redirectUri = originalRedirectUri;
      }

      tokenStore.setAccessToken(accessToken);
      await this.fetchUserInfo(accessToken);
      console.log('[oauth] Auth flow complete.');

      return accessToken;
    } finally {
      this.cleanup();
    }
  }

  /**
   * Wait for OAuth callback
   * Returns a promise that resolves when callback is received
   */
  private waitForCallback(): Promise<OAuthCallbackData> {
    return new Promise((resolve, reject) => {
      this.resolveCallback = resolve;
      this.rejectCallback = reject;

      console.log('[oauth] Waiting for callback...');

      // Set timeout (5 minutes)
      setTimeout(() => {
        if (this.rejectCallback) {
          console.warn('[oauth] OAuth callback wait timeout, rejecting...');
          reject(new Error('OAuth timeout'));
          this.cleanup();
        }
      }, 5 * 60 * 1000);
    });
  }

  /**
   * Handle OAuth callback from deep link
   * This should be called when the app receives a callback URL
   */
  handleCallback(callbackUrl: string): void {
    try {
      console.log('[oauth] handleCallback called with:', callbackUrl);
      const url = new URL(callbackUrl);
      const code = url.searchParams.get('code');
      const state = url.searchParams.get('state');
      const error = url.searchParams.get('error');
      const errorDescription = url.searchParams.get('error_description');

      if (error) {
        const errorMsg = errorDescription || error;
        if (this.rejectCallback) {
          this.rejectCallback(new Error(`OAuth error: ${errorMsg}`));
          this.cleanup();
        }
        return;
      }

      if (!code || !state) {
        if (this.rejectCallback) {
          this.rejectCallback(new Error('Missing code or state in callback'));
          this.cleanup();
        }
        return;
      }

      if (this.resolveCallback) {
        this.resolveCallback({ code, state });
        this.cleanup();
      }
    } catch (error) {
      if (this.rejectCallback) {
        this.rejectCallback(error as Error);
        this.cleanup();
      }
    }
  }

  /**
   * Fetch user info after successful authentication
   */
  private async fetchUserInfo(accessToken: string): Promise<void> {
    try {
      // TODO: Replace with actual user info endpoint
      const response = await fetch(`${marketplaceConfig.apiBaseUrl}/user/profile`, {
        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      });

      if (response.ok) {
        const userInfo = await response.json();
        tokenStore.setUserInfo(userInfo);
      }
    } catch (error) {
      console.error('Failed to fetch user info:', error);
      // Don't throw - user info fetch failure shouldn't block auth
    }
  }

  /**
   * Clean up callback handlers
   */
  private cleanup(): void {
    this.resolveCallback = null;
    this.rejectCallback = null;
  }

  /**
   * Logout user
   */
  async logout(): Promise<void> {
    const token = tokenStore.getAccessToken();
    if (token) {
      await oauthService.revokeToken(token);
    }
    tokenStore.clearAll();
  }
}

// Export singleton instance
export const oauthCallbackHandler = new OAuthCallbackHandler();

