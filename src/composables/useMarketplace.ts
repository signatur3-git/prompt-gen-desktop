/**
 * Marketplace Authentication Composable
 *
 * Vue composable for managing marketplace authentication state
 */

import { ref, computed } from 'vue';
import { oauthCallbackHandler } from '../services/oauth-callback-handler';
import { tokenStore } from '../stores/token.store';
import { getDebugInfo } from '../services/debug.service';

const isConnecting = ref(false);
const authError = ref<string | null>(null);

export function useMarketplace() {
  const isAuthenticated = computed(() => tokenStore.isAuthenticated());
  const userInfo = computed(() => tokenStore.getUserInfo());

  /**
   * Connect to marketplace (OAuth flow)
   */
  async function connect(): Promise<boolean> {
    if (isConnecting.value) {
      return false;
    }

    isConnecting.value = true;
    authError.value = null;

    try {
      await oauthCallbackHandler.startAuthFlow();
      return true;
    } catch (error) {
      const baseMsg = (error as Error).message;
      // In release builds there may be no DevTools, so attach a tiny bit of debug context.
      const dbg = await getDebugInfo();
      authError.value = `${baseMsg}\n\n${dbg}`;
      console.error('Marketplace connection failed:', error);
      return false;
    } finally {
      isConnecting.value = false;
    }
  }

  /**
   * Disconnect from marketplace (logout)
   */
  async function disconnect(): Promise<void> {
    try {
      await oauthCallbackHandler.logout();
    } catch (error) {
      console.error('Disconnect failed:', error);
      // Clear local data even if server revocation fails
      tokenStore.clearAll();
    }
  }

  /**
   * Clear authentication error
   */
  function clearError(): void {
    authError.value = null;
  }

  return {
    isAuthenticated,
    isConnecting: computed(() => isConnecting.value),
    authError: computed(() => authError.value),
    userInfo,
    connect,
    disconnect,
    clearError,
  };
}
