/**
 * Marketplace API Client
 *
 * Provides methods to interact with the marketplace API:
 * - Browse and search packages
 * - Download package files
 * - Publish packages
 * - Manage user packages
 */

import marketplaceConfig from '../config/marketplace.config';
import { tokenStore } from '../stores/token.store';

export interface MarketplacePackage {
  id: string;
  namespace: string;
  name: string;
  description: string;
  // Listing endpoint returns persona id; detailed endpoint may later return a resolved author/display name.
  author?: string;
  author_persona_id?: string;
  authorPersonaId?: string;
  versions?: PackageVersion[];
  tags?: string[];
  created_at: string;
  updated_at: string;
}

export interface PackageVersion {
  version: string;
  changelog?: string;
  published_at: string;
}

export interface Persona {
  id: string;
  username?: string;
  display_name?: string;
  displayName?: string;
  name?: string;
}

export interface PublishResponse {
  success: boolean;
  package: MarketplacePackage;
  message: string;
}

export class MarketplaceClient {
  private baseUrl = marketplaceConfig.apiBaseUrl;

  private personaNameCache = new Map<string, string>();

  /**
   * Generic fetch wrapper with authentication
   */
  private async fetch(path: string, options: RequestInit = {}): Promise<any> {
    const token = tokenStore.getAccessToken();
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    // Add authorization header if token exists
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    // Merge with options headers
    if (options.headers) {
      Object.assign(headers, options.headers);
    }

    const url = `${this.baseUrl}${path}`;

    // Prevent infinite loading if fetch hangs (common when a dev server is down / blocked).
    const controller = new AbortController();
    const timeoutMs = 10_000;
    const timeout = setTimeout(() => controller.abort(), timeoutMs);

    let response: Response;
    try {
      response = await fetch(url, {
        ...options,
        headers,
        signal: controller.signal,
      });
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      throw new Error(`Network error calling ${url}: ${msg}`);
    } finally {
      clearTimeout(timeout);
    }

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`API error (${response.status}) calling ${url}: ${errorText}`);
    }

    const contentType = response.headers.get('content-type') || '';
    if (contentType.includes('application/json')) {
      return response.json();
    }

    // Fallback for endpoints that return non-JSON.
    return response.text();
  }

  /**
   * Fetch personas (requires auth on your marketplace)
   */
  async getPersonas(): Promise<Persona[]> {
    const data = await this.fetch('/personas');
    if (Array.isArray(data)) {
      return data as Persona[];
    }
    if (data && typeof data === 'object' && Array.isArray((data as any).personas)) {
      return (data as any).personas as Persona[];
    }
    return [];
  }

  /**
   * Resolve a persona id to a human-friendly name.
   *
   * Prefers the persona's `name` field (marketplace contract) and avoids showing raw ids.
   */
  async resolvePersonaName(personaId: string): Promise<string> {
    if (!personaId) return '';

    const cached = this.personaNameCache.get(personaId);
    if (cached) return cached;

    try {
      const personas = await this.getPersonas();
      // Fill cache for all returned personas (cheap and avoids repeated calls)
      for (const p of personas) {
        // Marketplace persona router requires `name`, so prefer it.
        const name = p.name || p.displayName || p.display_name || p.username;
        if (p.id && name) {
          this.personaNameCache.set(p.id, name);
        }
      }
    } catch {
      // ignore and fall back
    }

    // If we can't resolve a name (e.g., persona belongs to another user), don't show the id.
    return this.personaNameCache.get(personaId) || '';
  }

  /**
   * Search packages
   * @param query - Search query (optional)
   * @param tags - Filter by tags (optional)
   */
  async searchPackages(query?: string, tags?: string[]): Promise<MarketplacePackage[]> {
    const params = new URLSearchParams();
    if (query) {
      params.append('search', query);
    }
    if (tags && tags.length > 0) {
      params.append('tags', tags.join(','));
    }

    const path = `/packages${params.toString() ? `?${params.toString()}` : ''}`;
    const data = await this.fetch(path);

    // The marketplace API returns either an array or an envelope like { packages: [...] }.
    if (Array.isArray(data)) {
      return data as MarketplacePackage[];
    }
    if (data && typeof data === 'object' && Array.isArray((data as any).packages)) {
      return (data as any).packages as MarketplacePackage[];
    }

    return [];
  }

  /**
   * Get package details
   */
  async getPackage(namespace: string, name: string): Promise<MarketplacePackage> {
    return this.fetch(`/packages/${namespace}/${name}`);
  }

  /**
   * Get specific package version details
   */
  async getPackageVersion(
    namespace: string,
    name: string,
    version: string
  ): Promise<PackageVersion> {
    return this.fetch(`/packages/${namespace}/${name}/${version}`);
  }

  /**
   * Download package YAML content
   */
  async downloadPackage(
    namespace: string,
    name: string,
    version: string
  ): Promise<string> {
    const token = tokenStore.getAccessToken();
    const headers: Record<string, string> = {};

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(
      `${this.baseUrl}/packages/${namespace}/${name}/${version}/download`,
      { headers }
    );

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Download failed (${response.status}): ${errorText}`);
    }

    return response.text(); // Return YAML content as string
  }

  /**
   * Publish a new package or version
   * @param yamlContent - Package YAML content
   * @param file - Optional package file (if different from YAML)
   */
  async publishPackage(yamlContent: string, file?: File): Promise<PublishResponse> {
    const token = tokenStore.getAccessToken();
    if (!token) {
      throw new Error('Authentication required to publish packages');
    }

    const formData = new FormData();

    // Add YAML content as blob
    const yamlBlob = new Blob([yamlContent], { type: 'application/x-yaml' });
    formData.append('package', yamlBlob, 'package.yaml');

    // Add additional file if provided
    if (file) {
      formData.append('file', file);
    }

    const response = await fetch(`${this.baseUrl}/packages`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
      },
      body: formData,
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Publish failed (${response.status}): ${errorText}`);
    }

    return response.json();
  }

  /**
   * Get current user's published packages
   */
  async getMyPackages(): Promise<MarketplacePackage[]> {
    const token = tokenStore.getAccessToken();
    if (!token) {
      throw new Error('Authentication required');
    }
    return this.fetch('/packages/me');
  }

  /**
   * Delete a package (requires ownership)
   */
  async deletePackage(namespace: string, name: string): Promise<void> {
    const token = tokenStore.getAccessToken();
    if (!token) {
      throw new Error('Authentication required');
    }

    await fetch(`${this.baseUrl}/packages/${namespace}/${name}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  /**
   * Get user profile information
   */
  async getUserProfile(): Promise<any> {
    const token = tokenStore.getAccessToken();
    if (!token) {
      throw new Error('Authentication required');
    }
    return this.fetch('/user/profile');
  }
}

// Export singleton instance
export const marketplaceClient = new MarketplaceClient();
