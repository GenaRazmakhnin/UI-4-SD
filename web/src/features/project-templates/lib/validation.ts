import type { ProjectConfig, ProjectConfigErrors } from './types';

/**
 * Validate project name
 */
export function validateProjectName(name: string): string | undefined {
  if (!name.trim()) {
    return 'Project name is required';
  }
  if (name.length < 2) {
    return 'Project name must be at least 2 characters';
  }
  if (name.length > 64) {
    return 'Project name must be less than 64 characters';
  }
  if (!/^[a-zA-Z0-9][a-zA-Z0-9\s\-_]*$/.test(name)) {
    return 'Project name can only contain letters, numbers, spaces, hyphens, and underscores';
  }
  return undefined;
}

/**
 * Validate canonical base URL
 */
export function validateCanonicalBase(url: string): string | undefined {
  if (!url.trim()) {
    return 'Canonical base URL is required';
  }
  try {
    const parsed = new URL(url);
    if (!['http:', 'https:'].includes(parsed.protocol)) {
      return 'URL must use http or https protocol';
    }
  } catch {
    return 'Invalid URL format';
  }
  if (url.endsWith('/')) {
    return 'URL should not end with a slash';
  }
  return undefined;
}

/**
 * Validate package ID (npm-style)
 */
export function validatePackageId(packageId: string): string | undefined {
  if (!packageId.trim()) {
    return 'Package ID is required';
  }
  // FHIR package ID format: scope.package.name
  if (!/^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)+$/.test(packageId)) {
    return 'Package ID must be lowercase, dot-separated (e.g., org.example.myig)';
  }
  if (packageId.length > 128) {
    return 'Package ID must be less than 128 characters';
  }
  return undefined;
}

/**
 * Validate version (semver)
 */
export function validateVersion(version: string): string | undefined {
  if (!version.trim()) {
    return 'Version is required';
  }
  // Simple semver check
  if (!/^\d+\.\d+\.\d+(-[a-zA-Z0-9]+)?$/.test(version)) {
    return 'Version must be in semver format (e.g., 1.0.0 or 1.0.0-beta)';
  }
  return undefined;
}

/**
 * Validate entire project config
 */
export function validateProjectConfig(config: ProjectConfig): ProjectConfigErrors {
  return {
    name: validateProjectName(config.name),
    canonicalBase: validateCanonicalBase(config.canonicalBase),
    packageId: validatePackageId(config.packageId),
    version: validateVersion(config.version),
  };
}

/**
 * Check if config has any errors
 */
export function hasErrors(errors: ProjectConfigErrors): boolean {
  return Object.values(errors).some((error) => error !== undefined);
}

/**
 * Generate package ID suggestion from project name
 */
export function suggestPackageId(projectName: string, canonicalBase: string): string {
  // Try to extract domain from canonical base
  let prefix = 'org.example';
  try {
    const url = new URL(canonicalBase);
    const parts = url.hostname.split('.').reverse();
    if (parts.length >= 2) {
      prefix = parts.slice(0, 2).join('.');
    }
  } catch {
    // Use default prefix
  }

  // Clean project name
  const cleanName = projectName
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '')
    .replace(/-+/g, '.');

  return cleanName ? `${prefix}.${cleanName}` : prefix;
}
