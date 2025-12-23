/**
 * Token Store for Marketplace Authentication
 *
 * Manages OAuth access token storage using localStorage.
 *
 * TODO: Consider using encrypted storage for production
 * (Tauri provides secure storage APIs)
 */

import { ref } from 'vue';

const TOKEN_KEY = 'marketplace_access_token';
const USER_INFO_KEY = 'marketplace_user_info';

// Keep reactive copies so UI updates immediately.
const accessTokenRef = ref<string | null>(localStorage.getItem(TOKEN_KEY));
const userInfoRef = ref<UserInfo | null>(readUserInfoFromStorage());

export interface UserInfo {
  id: string;
  username: string;
  email: string;
}

function readUserInfoFromStorage(): UserInfo | null {
  const data = localStorage.getItem(USER_INFO_KEY);
  try {
    return data ? (JSON.parse(data) as UserInfo) : null;
  } catch {
    return null;
  }
}

class TokenStore {
  /**
   * Store access token
   */
  setAccessToken(token: string): void {
    localStorage.setItem(TOKEN_KEY, token);
    accessTokenRef.value = token;
  }

  /**
   * Get access token
   */
  getAccessToken(): string | null {
    return accessTokenRef.value;
  }

  /**
   * Clear access token
   */
  clearAccessToken(): void {
    localStorage.removeItem(TOKEN_KEY);
    accessTokenRef.value = null;
  }

  /**
   * Check if user is authenticated
   */
  isAuthenticated(): boolean {
    return !!accessTokenRef.value;
  }

  /**
   * Store user info
   */
  setUserInfo(userInfo: UserInfo): void {
    localStorage.setItem(USER_INFO_KEY, JSON.stringify(userInfo));
    userInfoRef.value = userInfo;
  }

  /**
   * Get user info
   */
  getUserInfo(): UserInfo | null {
    return userInfoRef.value;
  }

  /**
   * Clear user info
   */
  clearUserInfo(): void {
    localStorage.removeItem(USER_INFO_KEY);
    userInfoRef.value = null;
  }

  /**
   * Clear all authentication data
   */
  clearAll(): void {
    this.clearAccessToken();
    this.clearUserInfo();
  }
}

// Export singleton instance
export const tokenStore = new TokenStore();
