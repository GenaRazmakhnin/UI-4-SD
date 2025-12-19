//! Project data models.
//!
//! Defines the core types for project management including projects,
//! resources, and dependencies.

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ir::FhirVersion;

/// A FHIR Implementation Guide project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Unique project identifier.
    pub id: String,
    /// Human-readable project name.
    pub name: String,
    /// Canonical base URL for resources in this project.
    pub canonical_base: String,
    /// FHIR version for this project.
    pub fhir_version: FhirVersion,
    /// Package dependencies.
    #[serde(default)]
    pub dependencies: Vec<PackageDependency>,
    /// Project description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Publisher name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Project version.
    #[serde(default = "default_version")]
    pub version: String,
    /// Project status.
    #[serde(default)]
    pub status: ProjectStatus,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp.
    pub modified_at: DateTime<Utc>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

/// Project status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    /// Project is in active development.
    #[default]
    Draft,
    /// Project is ready for review.
    Review,
    /// Project is published.
    Published,
    /// Project is archived.
    Archived,
}

impl Project {
    /// Create a new project with the given name and canonical base.
    pub fn new(id: impl Into<String>, name: impl Into<String>, canonical_base: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            canonical_base: canonical_base.into(),
            fhir_version: FhirVersion::R4,
            dependencies: Vec::new(),
            description: None,
            publisher: None,
            version: default_version(),
            status: ProjectStatus::Draft,
            created_at: now,
            modified_at: now,
        }
    }

    /// Set the FHIR version.
    pub fn with_fhir_version(mut self, version: FhirVersion) -> Self {
        self.fhir_version = version;
        self
    }

    /// Add a dependency.
    pub fn with_dependency(mut self, dep: PackageDependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Generate canonical URL for a resource.
    pub fn canonical_url(&self, resource_name: &str) -> String {
        format!("{}/StructureDefinition/{}", self.canonical_base, resource_name)
    }

    /// Mark the project as modified.
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }
}

/// A package dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDependency {
    /// Package name (e.g., "hl7.fhir.us.core").
    pub name: String,
    /// Version constraint (e.g., "6.1.0", "^6.0.0").
    pub version: String,
    /// Whether this is a dev-only dependency.
    #[serde(default)]
    pub dev: bool,
}

impl PackageDependency {
    /// Create a new dependency.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            dev: false,
        }
    }
}

/// A resource within a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResource {
    /// Resource ID (unique within project).
    pub id: String,
    /// Canonical URL of the resource.
    pub canonical_url: String,
    /// Kind of resource.
    pub kind: ResourceKind,
    /// Human-readable name.
    pub name: String,
    /// Source format preference.
    pub source_format: SourceFormat,
    /// Relative path to IR file.
    pub ir_path: PathBuf,
    /// Relative path to SD file (for raw imports).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd_path: Option<PathBuf>,
    /// Base definition this resource derives from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Resources this depends on (canonical URLs).
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp.
    pub modified_at: DateTime<Utc>,
}

impl ProjectResource {
    /// Create a new project resource.
    pub fn new(
        id: impl Into<String>,
        canonical_url: impl Into<String>,
        name: impl Into<String>,
        kind: ResourceKind,
    ) -> Self {
        let id = id.into();
        let now = Utc::now();
        Self {
            id: id.clone(),
            canonical_url: canonical_url.into(),
            name: name.into(),
            kind,
            source_format: SourceFormat::Ir,
            ir_path: PathBuf::from(format!("IR/resources/{}.json", id)),
            sd_path: None,
            base: None,
            dependencies: Vec::new(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Set the base definition.
    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// Set the source format.
    pub fn with_source_format(mut self, format: SourceFormat) -> Self {
        self.source_format = format;
        self
    }

    /// Add a dependency.
    pub fn with_dependency(mut self, canonical_url: impl Into<String>) -> Self {
        self.dependencies.push(canonical_url.into());
        self
    }

    /// Mark the resource as modified.
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }
}

/// Kind of FHIR resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    /// StructureDefinition profile.
    Profile,
    /// StructureDefinition extension.
    Extension,
    /// ValueSet.
    ValueSet,
    /// CodeSystem.
    CodeSystem,
    /// Instance (example resource).
    Instance,
}

impl ResourceKind {
    /// Get the subdirectory for this resource kind.
    pub fn sd_subdir(&self) -> &'static str {
        match self {
            Self::Profile | Self::Extension => "StructureDefinition",
            Self::ValueSet => "ValueSet",
            Self::CodeSystem => "CodeSystem",
            Self::Instance => "Instance",
        }
    }

    /// Get the FSH subdirectory for this resource kind.
    pub fn fsh_subdir(&self) -> &'static str {
        match self {
            Self::Profile => "profiles",
            Self::Extension => "extensions",
            Self::ValueSet => "valuesets",
            Self::CodeSystem => "codesystems",
            Self::Instance => "instances",
        }
    }

    /// Get the FHIR resource type for this kind.
    pub fn resource_type(&self) -> &'static str {
        match self {
            Self::Profile | Self::Extension => "StructureDefinition",
            Self::ValueSet => "ValueSet",
            Self::CodeSystem => "CodeSystem",
            Self::Instance => "Resource",
        }
    }
}

/// Source format preference for a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceFormat {
    /// IR is the source of truth (default for new resources).
    #[default]
    Ir,
    /// FSH is the source of truth (imported from FSH).
    Fsh,
    /// StructureDefinition JSON is the source (imported from SD).
    Sd,
}

/// Project resource index stored in IR/index.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndex {
    /// Version of the index format.
    #[serde(default = "default_project_index_version")]
    pub version: u32,
    /// Last modified timestamp.
    #[serde(default = "default_project_index_modified_at")]
    pub modified_at: DateTime<Utc>,
    /// Resources in the project.
    #[serde(default)]
    pub resources: HashMap<String, ProjectResource>,
}

fn default_project_index_version() -> u32 {
    1
}

fn default_project_index_modified_at() -> DateTime<Utc> {
    Utc::now()
}

impl Default for ProjectIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectIndex {
    /// Create a new empty index.
    pub fn new() -> Self {
        Self {
            version: 1,
            modified_at: Utc::now(),
            resources: HashMap::new(),
        }
    }

    /// Add a resource to the index.
    pub fn add_resource(&mut self, resource: ProjectResource) {
        self.resources.insert(resource.id.clone(), resource);
        self.modified_at = Utc::now();
    }

    /// Remove a resource from the index.
    pub fn remove_resource(&mut self, resource_id: &str) -> Option<ProjectResource> {
        let removed = self.resources.remove(resource_id);
        if removed.is_some() {
            self.modified_at = Utc::now();
        }
        removed
    }

    /// Get a resource by ID.
    pub fn get_resource(&self, resource_id: &str) -> Option<&ProjectResource> {
        self.resources.get(resource_id)
    }

    /// Get a mutable reference to a resource.
    pub fn get_resource_mut(&mut self, resource_id: &str) -> Option<&mut ProjectResource> {
        self.resources.get_mut(resource_id)
    }

    /// Find a resource by canonical URL.
    pub fn find_by_canonical(&self, canonical_url: &str) -> Option<&ProjectResource> {
        self.resources.values().find(|r| r.canonical_url == canonical_url)
    }

    /// Get all resources of a specific kind.
    pub fn resources_by_kind(&self, kind: ResourceKind) -> Vec<&ProjectResource> {
        self.resources.values().filter(|r| r.kind == kind).collect()
    }

    /// Get the total number of resources.
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

/// Node kind for project explorer tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Folder,
    File,
}

/// Root folder type in project structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectTreeRoot {
    IR,
    SD,
    FSH,
}

/// File tree node for project explorer.
/// Matches frontend ProjectTreeNode interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTreeNode {
    /// Full path relative to project root.
    pub path: String,
    /// Display label for the node (usually the last segment of the path).
    pub name: String,
    /// Whether this is a folder or file.
    pub kind: NodeKind,
    /// Root folder type (IR, SD, or FSH).
    pub root: ProjectTreeRoot,
    /// Resource ID if this is a resource file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Canonical URL if this is a resource file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    /// Resource type (e.g., "StructureDefinition", "ValueSet").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// Resource kind if this is a resource file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<ResourceKind>,
    /// Children nodes (for directories).
    #[serde(default)]
    pub children: Vec<FileTreeNode>,
}

impl FileTreeNode {
    /// Create a new directory node.
    pub fn directory(name: impl Into<String>, path: impl Into<String>, root: ProjectTreeRoot) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            kind: NodeKind::Folder,
            root,
            resource_id: None,
            canonical_url: None,
            resource_type: None,
            resource_kind: None,
            children: Vec::new(),
        }
    }

    /// Create a new file node.
    pub fn file(name: impl Into<String>, path: impl Into<String>, root: ProjectTreeRoot) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            kind: NodeKind::File,
            root,
            resource_id: None,
            canonical_url: None,
            resource_type: None,
            resource_kind: None,
            children: Vec::new(),
        }
    }

    /// Set the resource info.
    pub fn with_resource(
        mut self,
        kind: ResourceKind,
        id: impl Into<String>,
        canonical_url: impl Into<String>,
    ) -> Self {
        self.resource_kind = Some(kind);
        self.resource_id = Some(id.into());
        self.canonical_url = Some(canonical_url.into());
        self.resource_type = Some(kind.resource_type().to_string());
        self
    }

    /// Add a child node.
    pub fn add_child(&mut self, child: FileTreeNode) {
        self.children.push(child);
    }
}

/// Dependency graph for cross-reference resolution.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// Forward dependencies (resource -> resources it depends on).
    forward: HashMap<String, Vec<String>>,
    /// Reverse dependencies (resource -> resources that depend on it).
    reverse: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the graph from a project index.
    pub fn from_index(index: &ProjectIndex) -> Self {
        let mut graph = Self::new();
        for resource in index.resources.values() {
            for dep in &resource.dependencies {
                graph.add_dependency(&resource.canonical_url, dep);
            }
        }
        graph
    }

    /// Add a dependency relationship.
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.forward
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        self.reverse
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }

    /// Get resources that a given resource depends on.
    pub fn dependencies_of(&self, canonical_url: &str) -> Vec<&str> {
        self.forward
            .get(canonical_url)
            .map(|deps| deps.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Get resources that depend on a given resource.
    pub fn dependents_of(&self, canonical_url: &str) -> Vec<&str> {
        self.reverse
            .get(canonical_url)
            .map(|deps| deps.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Check for circular dependencies starting from a resource.
    pub fn has_circular_dependency(&self, canonical_url: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![canonical_url.to_string()];

        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                // Already visited - circular dependency
                return true;
            }

            if let Some(deps) = self.forward.get(&current) {
                for dep in deps {
                    if dep == canonical_url {
                        return true;
                    }
                    stack.push(dep.clone());
                }
            }
        }

        false
    }

    /// Get all resources in topological order (dependencies first).
    pub fn topological_order(&self) -> Result<Vec<String>, CircularDependencyError> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();

        // Collect all nodes from both forward and reverse maps
        let mut all_nodes = std::collections::HashSet::new();
        for node in self.forward.keys() {
            all_nodes.insert(node.clone());
        }
        for node in self.reverse.keys() {
            all_nodes.insert(node.clone());
        }

        for node in &all_nodes {
            if !visited.contains(node) {
                self.visit_topo(node, &mut visited, &mut temp_visited, &mut result)?;
            }
        }

        // Result is already in correct order (dependencies first)
        // because we visit dependencies before pushing the node
        Ok(result)
    }

    fn visit_topo(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), CircularDependencyError> {
        if temp_visited.contains(node) {
            return Err(CircularDependencyError {
                resource: node.to_string(),
            });
        }
        if visited.contains(node) {
            return Ok(());
        }

        temp_visited.insert(node.to_string());

        if let Some(deps) = self.forward.get(node) {
            for dep in deps {
                self.visit_topo(dep, visited, temp_visited, result)?;
            }
        }

        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());

        Ok(())
    }
}

/// Error indicating a circular dependency was detected.
#[derive(Debug, Clone)]
pub struct CircularDependencyError {
    /// The resource where the cycle was detected.
    pub resource: String,
}

impl std::fmt::Display for CircularDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circular dependency detected at: {}", self.resource)
    }
}

impl std::error::Error for CircularDependencyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("my-ig", "My IG", "http://example.org/fhir");
        assert_eq!(project.id, "my-ig");
        assert_eq!(project.name, "My IG");
        assert_eq!(project.canonical_base, "http://example.org/fhir");
        assert_eq!(project.fhir_version, FhirVersion::R4);
    }

    #[test]
    fn test_project_canonical_url() {
        let project = Project::new("my-ig", "My IG", "http://example.org/fhir");
        let url = project.canonical_url("MyPatient");
        assert_eq!(url, "http://example.org/fhir/StructureDefinition/MyPatient");
    }

    #[test]
    fn test_project_resource_creation() {
        let resource = ProjectResource::new(
            "my-patient",
            "http://example.org/fhir/StructureDefinition/MyPatient",
            "MyPatient",
            ResourceKind::Profile,
        )
        .with_base("Patient");

        assert_eq!(resource.id, "my-patient");
        assert_eq!(resource.kind, ResourceKind::Profile);
        assert_eq!(resource.base, Some("Patient".to_string()));
    }

    #[test]
    fn test_project_index() {
        let mut index = ProjectIndex::new();

        let resource = ProjectResource::new(
            "my-patient",
            "http://example.org/fhir/StructureDefinition/MyPatient",
            "MyPatient",
            ResourceKind::Profile,
        );

        index.add_resource(resource);
        assert_eq!(index.resource_count(), 1);

        let found = index.find_by_canonical("http://example.org/fhir/StructureDefinition/MyPatient");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "MyPatient");
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("A", "C");
        graph.add_dependency("B", "C");

        assert_eq!(graph.dependencies_of("A"), vec!["B", "C"]);
        assert_eq!(graph.dependents_of("C"), vec!["A", "B"]);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");
        graph.add_dependency("C", "A");

        assert!(graph.has_circular_dependency("A"));
    }

    #[test]
    fn test_topological_order() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");

        let order = graph.topological_order().unwrap();
        // C has no dependencies, then B, then A
        let a_pos = order.iter().position(|x| x == "A").unwrap();
        let b_pos = order.iter().position(|x| x == "B").unwrap();
        let c_pos = order.iter().position(|x| x == "C").unwrap();

        assert!(c_pos < b_pos);
        assert!(b_pos < a_pos);
    }
}
