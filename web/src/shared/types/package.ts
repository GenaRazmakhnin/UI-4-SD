export interface Package {
  id: string;
  name: string;
  version: string;
  description?: string;
  fhirVersion: string;
  installed: boolean;
  size: string;
  dependencies?: PackageDependency[];
}

export interface PackageDependency {
  name: string;
  version: string;
}
