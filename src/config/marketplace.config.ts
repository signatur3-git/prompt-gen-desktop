/**
 * Marketplace OAuth and API Configuration
 *
 * This configuration connects the desktop app to the deployed marketplace.
 * Update MARKETPLACE_BASE_URL for your environment.
 */

// Production marketplace URL - can be overridden with VITE_MARKETPLACE_URL env var
const DEFAULT_PROD_BASE_URL = 'https://prompt-gen-marketplace-production.up.railway.app';

// IMPORTANT:
// Vite always loads `.env` for all modes (dev + build). If `.env` contains localhost URLs
// (typical for development), `tauri:build` would accidentally bake localhost into your
// production app.
//
// Therefore:
// - In DEV: allow overriding via VITE_MARKETPLACE_* in `.env`.
// - In PROD: default to the production Railway URL. Only override if you provide env vars
//   through a production-specific mechanism (e.g., CI, `.env.production`, etc.).
const MARKETPLACE_BASE_URL = import.meta.env.DEV
  ? (import.meta.env.VITE_MARKETPLACE_URL || DEFAULT_PROD_BASE_URL)
  : (import.meta.env.VITE_MARKETPLACE_URL && !import.meta.env.VITE_MARKETPLACE_URL.includes('localhost')
      ? import.meta.env.VITE_MARKETPLACE_URL
      : DEFAULT_PROD_BASE_URL);

// OAuth endpoints are often split: authorize may be handled by a top-level route that sets cookies/session
// while token/revoke may live under an API prefix.
const MARKETPLACE_AUTH_BASE_URL = import.meta.env.DEV
  ? (import.meta.env.VITE_MARKETPLACE_AUTH_URL || MARKETPLACE_BASE_URL)
  : (import.meta.env.VITE_MARKETPLACE_AUTH_URL && !import.meta.env.VITE_MARKETPLACE_AUTH_URL.includes('localhost')
      ? import.meta.env.VITE_MARKETPLACE_AUTH_URL
      : MARKETPLACE_BASE_URL);

const MARKETPLACE_TOKEN_BASE_URL = import.meta.env.DEV
  ? (import.meta.env.VITE_MARKETPLACE_TOKEN_URL || import.meta.env.VITE_MARKETPLACE_OAUTH_URL || MARKETPLACE_BASE_URL)
  : (
      // Prefer explicit token base URL if provided and not localhost.
      (import.meta.env.VITE_MARKETPLACE_TOKEN_URL && !import.meta.env.VITE_MARKETPLACE_TOKEN_URL.includes('localhost')
        ? import.meta.env.VITE_MARKETPLACE_TOKEN_URL
        : (import.meta.env.VITE_MARKETPLACE_OAUTH_URL && !import.meta.env.VITE_MARKETPLACE_OAUTH_URL.includes('localhost')
            ? import.meta.env.VITE_MARKETPLACE_OAUTH_URL
            : MARKETPLACE_BASE_URL))
    );

export const marketplaceConfig = {
  // OAuth endpoints
  authorizationEndpoint: `${MARKETPLACE_AUTH_BASE_URL}/oauth/authorize`,
  // NOTE: In production the marketplace serves the interactive consent UI under `/oauth/*`,
  // but the programmatic token endpoints are under the API prefix.
  tokenEndpoint: `${MARKETPLACE_TOKEN_BASE_URL}/api/v1/oauth/token`,
  revokeEndpoint: `${MARKETPLACE_TOKEN_BASE_URL}/api/v1/oauth/revoke`,

  // API endpoint
  apiBaseUrl: `${MARKETPLACE_BASE_URL}/api/v1`,

  // OAuth client configuration
  clientId: 'prompt-gen-desktop',

  // Redirect URIs
  // DEV: Must match exactly what the marketplace has registered for the client.
  //      Override with VITE_MARKETPLACE_REDIRECT_URI to match DB seed data.
  // PROD: Use the custom protocol deep link.
  redirectUri: import.meta.env.DEV
    ? (import.meta.env.VITE_MARKETPLACE_REDIRECT_URI || 'http://localhost:51234/oauth/callback')
    : 'promptgen://oauth/callback',

  // OAuth scopes
  scope: 'read:packages write:packages',

  // Callback server configuration (for development)
  callbackServerPort: 51234,
};

export default marketplaceConfig;
