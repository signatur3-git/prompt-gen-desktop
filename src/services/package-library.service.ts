// Package Library Service
// Frontend service for interacting with the package library

import { invoke } from '@tauri-apps/api/core';

export interface LibraryEntry {
  id: string;
  name: string;
  version: string;
  source: 'marketplace' | 'local' | 'imported';
  path: string;
  installedAt: number;
  lastUsed: number;
  metadata: {
    authors: string[];
    description?: string;
    tags: string[];
  };
}

export interface Package {
  id: string;
  version: string;
  metadata: {
    name: string;
    description?: string;
    authors: string[];
  };
  namespaces: Record<string, any>;
}

/**
 * Initialize the package library
 */
export async function initLibrary(): Promise<void> {
  await invoke('init_library');
}

/**
 * List all packages in the library
 */
export async function listLibraryPackages(): Promise<LibraryEntry[]> {
  return await invoke('list_library_packages');
}

/**
 * Install a package to the library
 */
export async function installPackageToLibrary(
  pkg: Package,
  yamlContent: string,
  source: 'marketplace' | 'local' | 'imported'
): Promise<LibraryEntry> {
  return await invoke('install_package_to_library', {
    package: pkg,
    yamlContent,
    source,
  });
}

/**
 * Remove a package from the library
 */
export async function removePackageFromLibrary(
  packageId: string,
  version: string
): Promise<void> {
  await invoke('remove_package_from_library', { packageId, version });
}

/**
 * Load a package from the library
 */
export async function loadPackageFromLibrary(
  packageId: string,
  version: string
): Promise<Package> {
  return await invoke('load_package_from_library', { packageId, version });
}

/**
 * Load all packages from the library
 */
export async function loadAllLibraryPackages(): Promise<Record<string, Package>> {
  return await invoke('load_all_library_packages');
}

/**
 * Get the library path
 */
export async function getLibraryPath(): Promise<string> {
  return await invoke('get_library_path');
}

/**
 * Refresh the library (re-scan filesystem)
 */
export async function refreshLibrary(): Promise<LibraryEntry[]> {
  return await invoke('refresh_library');
}

