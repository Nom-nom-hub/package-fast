/**
 * Package Fast - A very fast Node.js package manager
 */

export interface PackageInfo {
  name: string;
  version: string;
  dependencies?: Record<string, string>;
}

export function getPackageInfo(name: string): PackageInfo {
  // Placeholder implementation
  return {
    name,
    version: '1.0.0',
  };
}

export default {
  getPackageInfo,
};