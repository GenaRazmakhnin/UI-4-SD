//! Project service for managing FHIR IG projects.
//!
//! Provides operations for creating, loading, saving, and managing projects.

use std::path::{Path, PathBuf};

use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::ir::{
    BaseDefinition, DocumentMetadata, FhirVersion, ProfileDocument, ProfiledResource,
};

/// Convert a string to a URL-safe slug.
fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

use super::model::*;

/// Project service error type.
#[derive(Debug, Error)]
pub enum ProjectError {
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Project not found.
    #[error("Project not found: {0}")]
    NotFound(String),

    /// Resource not found.
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// Invalid project structure.
    #[error("Invalid project structure: {0}")]
    InvalidStructure(String),

    /// Project already exists.
    #[error("Project already exists: {0}")]
    AlreadyExists(String),

    /// Resource already exists.
    #[error("Resource already exists: {0}")]
    ResourceAlreadyExists(String),

    /// Invalid canonical URL.
    #[error("Invalid canonical URL: {0}")]
    InvalidCanonicalUrl(String),

    /// Dependency error.
    #[error("Dependency error: {0}")]
    DependencyError(String),

    /// Circular dependency detected.
    #[error("Circular dependency: {0}")]
    CircularDependency(String),
}

pub type ProjectResult<T> = Result<T, ProjectError>;

/// Project service for managing projects.
#[derive(Debug, Clone)]
pub struct ProjectService {
    /// Workspace root directory.
    workspace_dir: PathBuf,
}

impl ProjectService {
    /// Create a new project service.
    pub fn new(workspace_dir: impl Into<PathBuf>) -> Self {
        Self {
            workspace_dir: workspace_dir.into(),
        }
    }

    /// Get the workspace directory.
    pub fn workspace_dir(&self) -> &Path {
        &self.workspace_dir
    }

    /// Get the path to a project directory.
    pub fn project_path(&self, project_id: &str) -> PathBuf {
        self.workspace_dir.join(project_id)
    }

    /// Get the path to project.json.
    fn project_config_path(&self, project_id: &str) -> PathBuf {
        self.project_path(project_id).join("project.json")
    }

    /// Get the path to IR/index.json.
    fn index_path(&self, project_id: &str) -> PathBuf {
        self.project_path(project_id).join("IR").join("index.json")
    }

    /// Get the path to IR/resources/.
    fn resources_dir(&self, project_id: &str) -> PathBuf {
        self.project_path(project_id).join("IR").join("resources")
    }

    /// Get the path to SD/<resource_type>/.
    fn sd_dir(&self, project_id: &str, resource_type: &str) -> PathBuf {
        self.project_path(project_id).join("SD").join(resource_type)
    }

    // === Project Operations ===

    /// List all projects in the workspace.
    pub async fn list_projects(&self) -> ProjectResult<Vec<Project>> {
        let mut projects = Vec::new();

        if !self.workspace_dir.exists() {
            return Ok(projects);
        }

        let mut entries = fs::read_dir(&self.workspace_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let config_path = path.join("project.json");
                if config_path.exists() {
                    if let Ok(project) = self.load_project_config(&path).await {
                        projects.push(project);
                    }
                }
            }
        }

        // Sort by modified_at descending (most recent first)
        projects.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

        Ok(projects)
    }

    /// Load project configuration from a directory.
    async fn load_project_config(&self, project_dir: &Path) -> ProjectResult<Project> {
        let config_path = project_dir.join("project.json");
        let content = fs::read_to_string(&config_path).await?;
        let project: Project = serde_json::from_str(&content)?;
        Ok(project)
    }

    /// Create a new project.
    pub async fn create_project(&self, request: CreateProjectRequest) -> ProjectResult<Project> {
        let project_dir = self.project_path(&request.id);

        // Check if project already exists
        if project_dir.exists() {
            return Err(ProjectError::AlreadyExists(request.id));
        }

        // Create directory structure
        fs::create_dir_all(&project_dir).await?;
        fs::create_dir_all(project_dir.join("IR").join("resources")).await?;
        fs::create_dir_all(project_dir.join("SD").join("StructureDefinition")).await?;
        fs::create_dir_all(project_dir.join("SD").join("ValueSet")).await?;
        fs::create_dir_all(project_dir.join("FSH").join("profiles")).await?;
        fs::create_dir_all(project_dir.join("FSH").join("extensions")).await?;
        fs::create_dir_all(project_dir.join("FSH").join("valuesets")).await?;

        // Create project
        let mut project = Project::new(&request.id, &request.name, &request.canonical_base);
        project.fhir_version = request.fhir_version.unwrap_or(FhirVersion::R4);
        project.description = request.description;
        project.publisher = request.publisher;
        project.dependencies = request.dependencies.unwrap_or_default();

        // Save project config
        self.save_project_config(&request.id, &project).await?;

        // Create empty index
        let index = ProjectIndex::new();
        self.save_index(&request.id, &index).await?;

        Ok(project)
    }

    /// Load a project by ID.
    pub async fn load_project(&self, project_id: &str) -> ProjectResult<Project> {
        let project_dir = self.project_path(project_id);

        if !project_dir.exists() {
            return Err(ProjectError::NotFound(project_id.to_string()));
        }

        self.load_project_config(&project_dir).await
    }

    /// Update project configuration.
    pub async fn update_project(
        &self,
        project_id: &str,
        request: UpdateProjectRequest,
    ) -> ProjectResult<Project> {
        let mut project = self.load_project(project_id).await?;

        if let Some(name) = request.name {
            project.name = name;
        }
        if let Some(description) = request.description {
            project.description = Some(description);
        }
        if let Some(publisher) = request.publisher {
            project.publisher = Some(publisher);
        }
        if let Some(version) = request.version {
            project.version = version;
        }
        if let Some(status) = request.status {
            project.status = status;
        }
        if let Some(dependencies) = request.dependencies {
            project.dependencies = dependencies;
        }

        project.touch();
        self.save_project_config(project_id, &project).await?;

        Ok(project)
    }

    /// Save project configuration.
    async fn save_project_config(&self, project_id: &str, project: &Project) -> ProjectResult<()> {
        let path = self.project_config_path(project_id);
        let content = serde_json::to_string_pretty(project)?;
        self.atomic_write(&path, &content).await?;
        Ok(())
    }

    /// Delete a project (removes from disk).
    pub async fn delete_project(&self, project_id: &str) -> ProjectResult<()> {
        let project_dir = self.project_path(project_id);

        if !project_dir.exists() {
            return Err(ProjectError::NotFound(project_id.to_string()));
        }

        fs::remove_dir_all(&project_dir).await?;
        Ok(())
    }

    // === Resource Index Operations ===

    /// Load the project index.
    pub async fn load_index(&self, project_id: &str) -> ProjectResult<ProjectIndex> {
        let path = self.index_path(project_id);

        if !path.exists() {
            return Ok(ProjectIndex::new());
        }

        let content = fs::read_to_string(&path).await?;
        let index: ProjectIndex = serde_json::from_str(&content)?;
        Ok(index)
    }

    /// Save the project index.
    async fn save_index(&self, project_id: &str, index: &ProjectIndex) -> ProjectResult<()> {
        let path = self.index_path(project_id);
        fs::create_dir_all(path.parent().unwrap()).await?;
        let content = serde_json::to_string_pretty(index)?;
        self.atomic_write(&path, &content).await?;
        Ok(())
    }

    // === Resource Operations ===

    /// Add a new resource to a project.
    pub async fn add_resource(
        &self,
        project_id: &str,
        request: AddResourceRequest,
    ) -> ProjectResult<ProjectResource> {
        let project = self.load_project(project_id).await?;
        let mut index = self.load_index(project_id).await?;

        // Generate resource ID if not provided
        let resource_id = request.id.clone().unwrap_or_else(|| {
            slugify(&request.name)
        });

        // Check if resource already exists
        if index.get_resource(&resource_id).is_some() {
            return Err(ProjectError::ResourceAlreadyExists(resource_id));
        }

        // Generate canonical URL
        let sd_type = match request.kind {
            ResourceKind::Profile | ResourceKind::Extension => "StructureDefinition",
            ResourceKind::ValueSet => "ValueSet",
            ResourceKind::CodeSystem => "CodeSystem",
            ResourceKind::Instance => "Instance",
        };
        let canonical_url = request.canonical_url.clone().unwrap_or_else(|| {
            format!("{}/{}/{}", project.canonical_base, sd_type, request.name)
        });

        // Create project resource
        let mut resource = ProjectResource::new(
            &resource_id,
            &canonical_url,
            &request.name,
            request.kind,
        );
        resource.base = request.base.clone();
        resource.source_format = request.source_format.unwrap_or_default();

        // Handle raw content if provided (for extension/valueset imports)
        if let Some(ref content) = request.content {
            // Save the raw content to SD folder
            let sd_path = self.save_sd_content(project_id, &resource_id, sd_type, content).await?;
            resource.sd_path = Some(sd_path);
        } else if matches!(request.kind, ResourceKind::Profile | ResourceKind::Extension) {
            // Create the IR document for profiles/extensions (when no raw content)
            let doc = self.create_resource_document(&project, &resource, &request)?;
            self.save_resource_document(project_id, &resource_id, &doc).await?;
        }

        // Add to index
        index.add_resource(resource.clone());
        self.save_index(project_id, &index).await?;

        // Update project modified time
        let mut project = project;
        project.touch();
        self.save_project_config(project_id, &project).await?;

        Ok(resource)
    }

    /// Create the IR document for a new resource.
    fn create_resource_document(
        &self,
        project: &Project,
        resource: &ProjectResource,
        request: &AddResourceRequest,
    ) -> ProjectResult<ProfileDocument> {
        let metadata = DocumentMetadata::new(
            &resource.id,
            &resource.canonical_url,
            &resource.name,
        );

        let base_def = match request.kind {
            ResourceKind::Profile => {
                let base_type = request.base.as_deref().unwrap_or("Resource");
                BaseDefinition::resource(base_type)
            }
            ResourceKind::Extension => {
                BaseDefinition::new("http://hl7.org/fhir/StructureDefinition/Extension")
            }
            _ => return Err(ProjectError::InvalidStructure(
                "Only Profile and Extension resources have IR documents".to_string()
            )),
        };

        let profiled_resource = ProfiledResource::new(
            &resource.canonical_url,
            project.fhir_version,
            base_def,
        );

        Ok(ProfileDocument::new(metadata, profiled_resource))
    }

    /// Save a resource document to disk.
    async fn save_resource_document(
        &self,
        project_id: &str,
        resource_id: &str,
        doc: &ProfileDocument,
    ) -> ProjectResult<()> {
        let dir = self.resources_dir(project_id);
        fs::create_dir_all(&dir).await?;

        let path = dir.join(format!("{}.json", resource_id));
        let content = serde_json::to_string_pretty(doc)?;
        self.atomic_write(&path, &content).await?;

        Ok(())
    }

    /// Save raw FHIR content to SD folder.
    async fn save_sd_content(
        &self,
        project_id: &str,
        resource_id: &str,
        resource_type: &str,
        content: &str,
    ) -> ProjectResult<PathBuf> {
        let dir = self.sd_dir(project_id, resource_type);
        fs::create_dir_all(&dir).await?;

        let path = dir.join(format!("{}.json", resource_id));

        // Parse and re-serialize for consistent formatting
        let parsed: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| ProjectError::InvalidStructure(format!("Invalid JSON: {}", e)))?;
        let formatted = serde_json::to_string_pretty(&parsed)?;

        self.atomic_write(&path, &formatted).await?;

        Ok(path)
    }

    /// Load a resource document from disk.
    pub async fn load_resource_document(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> ProjectResult<ProfileDocument> {
        let path = self.resources_dir(project_id).join(format!("{}.json", resource_id));

        if !path.exists() {
            return Err(ProjectError::ResourceNotFound(resource_id.to_string()));
        }

        let content = fs::read_to_string(&path).await?;
        let doc: ProfileDocument = serde_json::from_str(&content)?;
        Ok(doc)
    }

    /// Remove a resource from a project.
    pub async fn remove_resource(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> ProjectResult<ProjectResource> {
        let mut index = self.load_index(project_id).await?;

        // Check for dependents
        let graph = DependencyGraph::from_index(&index);
        if let Some(resource) = index.get_resource(resource_id) {
            let dependents = graph.dependents_of(&resource.canonical_url);
            if !dependents.is_empty() {
                return Err(ProjectError::DependencyError(format!(
                    "Cannot delete resource '{}': {} resource(s) depend on it",
                    resource_id,
                    dependents.len()
                )));
            }
        }

        // Remove from index
        let resource = index
            .remove_resource(resource_id)
            .ok_or_else(|| ProjectError::ResourceNotFound(resource_id.to_string()))?;

        // Remove IR file
        let ir_path = self.resources_dir(project_id).join(format!("{}.json", resource_id));
        if ir_path.exists() {
            fs::remove_file(&ir_path).await?;
        }

        // Remove SD file if exists
        let sd_path = self.project_path(project_id)
            .join("SD")
            .join(resource.kind.sd_subdir())
            .join(format!("{}.json", resource.name));
        if sd_path.exists() {
            fs::remove_file(&sd_path).await?;
        }

        // Remove FSH file if exists
        let fsh_path = self.project_path(project_id)
            .join("FSH")
            .join(resource.kind.fsh_subdir())
            .join(format!("{}.fsh", resource.name));
        if fsh_path.exists() {
            fs::remove_file(&fsh_path).await?;
        }

        // Save updated index
        self.save_index(project_id, &index).await?;

        Ok(resource)
    }

    /// List all resources in a project.
    pub async fn list_resources(&self, project_id: &str) -> ProjectResult<Vec<ProjectResource>> {
        let index = self.load_index(project_id).await?;
        Ok(index.resources.into_values().collect())
    }

    /// Get a resource by ID.
    pub async fn get_resource(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> ProjectResult<ProjectResource> {
        let index = self.load_index(project_id).await?;
        index
            .get_resource(resource_id)
            .cloned()
            .ok_or_else(|| ProjectError::ResourceNotFound(resource_id.to_string()))
    }

    // === File Tree ===

    /// Build a file tree for the project explorer.
    pub async fn build_file_tree(&self, project_id: &str) -> ProjectResult<FileTreeNode> {
        let project_dir = self.project_path(project_id);
        let index = self.load_index(project_id).await?;

        // Root node uses IR as default root type
        let mut root = FileTreeNode::directory(project_id, project_id, ProjectTreeRoot::IR);

        // Build IR tree
        let ir_dir = project_dir.join("IR");
        if ir_dir.exists() {
            let mut ir_node = FileTreeNode::directory("IR", "IR", ProjectTreeRoot::IR);

            let resources_dir = ir_dir.join("resources");
            if resources_dir.exists() {
                let mut resources_node = FileTreeNode::directory("resources", "IR/resources", ProjectTreeRoot::IR);

                let mut entries = fs::read_dir(&resources_dir).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.extension().is_some_and(|e| e == "json") {
                        let name = path.file_name().unwrap().to_string_lossy().to_string();
                        let resource_id = path.file_stem().unwrap().to_string_lossy().to_string();
                        let rel_path = format!("IR/resources/{}", name);

                        let mut file_node = FileTreeNode::file(&name, &rel_path, ProjectTreeRoot::IR);
                        if let Some(resource) = index.get_resource(&resource_id) {
                            file_node = file_node.with_resource(
                                resource.kind,
                                &resource_id,
                                &resource.canonical_url,
                            );
                        } else {
                            // Resource not in index - read IR file to detect type
                            // This handles files imported from SD with derivation "constraint"
                            if let Ok(ir_content) = fs::read_to_string(&path).await {
                                if let Ok(ir_json) = serde_json::from_str::<serde_json::Value>(&ir_content) {
                                    // Get metadata and resource info
                                    let metadata_id = ir_json.get("metadata")
                                        .and_then(|m| m.get("id"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or(&resource_id);
                                    let canonical_url = ir_json.get("metadata")
                                        .and_then(|m| m.get("url"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");

                                    // Determine resource kind from IR structure
                                    // IR files with resource.base are constraint profiles
                                    let resource_kind = ir_json.get("resource")
                                        .and_then(|r| r.get("kind"))
                                        .and_then(|v| v.as_str());
                                    let has_base = ir_json.get("resource")
                                        .and_then(|r| r.get("base"))
                                        .is_some();

                                    let kind = if resource_kind == Some("resource") && has_base {
                                        // StructureDefinition with derivation constraint = profile
                                        ResourceKind::Profile
                                    } else if resource_kind == Some("complex-type") && has_base {
                                        // Extension
                                        ResourceKind::Extension
                                    } else {
                                        // Default to profile for unknown IR resources
                                        ResourceKind::Profile
                                    };

                                    file_node = file_node.with_resource(
                                        kind,
                                        metadata_id,
                                        canonical_url,
                                    );
                                }
                            }
                        }
                        resources_node.add_child(file_node);
                    }
                }

                ir_node.add_child(resources_node);
            }

            root.add_child(ir_node);
        }

        // Build SD tree
        let sd_dir = project_dir.join("SD");
        if sd_dir.exists() {
            let mut sd_node = FileTreeNode::directory("SD", "SD", ProjectTreeRoot::SD);

            for subdir in ["StructureDefinition", "ValueSet", "CodeSystem"] {
                let sub_path = sd_dir.join(subdir);
                if sub_path.exists() {
                    let mut sub_node = FileTreeNode::directory(subdir, &format!("SD/{}", subdir), ProjectTreeRoot::SD);

                    let mut entries = fs::read_dir(&sub_path).await?;
                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();
                        if path.extension().is_some_and(|e| e == "json") {
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let rel_path = format!("SD/{}/{}", subdir, name);
                            sub_node.add_child(FileTreeNode::file(&name, &rel_path, ProjectTreeRoot::SD));
                        }
                    }

                    if !sub_node.children.is_empty() {
                        sd_node.add_child(sub_node);
                    }
                }
            }

            if !sd_node.children.is_empty() {
                root.add_child(sd_node);
            }
        }

        // Build FSH tree
        let fsh_dir = project_dir.join("FSH");
        if fsh_dir.exists() {
            let mut fsh_node = FileTreeNode::directory("FSH", "FSH", ProjectTreeRoot::FSH);

            for subdir in ["profiles", "extensions", "valuesets", "codesystems", "instances"] {
                let sub_path = fsh_dir.join(subdir);
                if sub_path.exists() {
                    let mut sub_node = FileTreeNode::directory(subdir, &format!("FSH/{}", subdir), ProjectTreeRoot::FSH);

                    let mut entries = fs::read_dir(&sub_path).await?;
                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();
                        if path.extension().is_some_and(|e| e == "fsh") {
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let rel_path = format!("FSH/{}/{}", subdir, name);
                            sub_node.add_child(FileTreeNode::file(&name, &rel_path, ProjectTreeRoot::FSH));
                        }
                    }

                    if !sub_node.children.is_empty() {
                        fsh_node.add_child(sub_node);
                    }
                }
            }

            if !fsh_node.children.is_empty() {
                root.add_child(fsh_node);
            }
        }

        Ok(root)
    }

    // === Dependency Graph ===

    /// Build the dependency graph for a project.
    pub async fn build_dependency_graph(&self, project_id: &str) -> ProjectResult<DependencyGraph> {
        let index = self.load_index(project_id).await?;
        Ok(DependencyGraph::from_index(&index))
    }

    // === Helpers ===

    /// Write file atomically (write to temp, sync, rename).
    async fn atomic_write(&self, path: &Path, content: &str) -> ProjectResult<()> {
        let temp_path = path.with_extension("tmp");

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write to temp file
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;

        // Rename to final path
        fs::rename(&temp_path, path).await?;

        Ok(())
    }
}

/// Request to create a new project.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    /// Project ID (used as directory name).
    pub id: String,
    /// Project name.
    pub name: String,
    /// Canonical base URL.
    pub canonical_base: String,
    /// FHIR version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fhir_version: Option<FhirVersion>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PackageDependency>>,
}

/// Request to update a project.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    /// Project name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectStatus>,
    /// Dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PackageDependency>>,
}

/// Request to add a resource/artifact to a project.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddResourceRequest {
    /// Resource ID (optional, generated from name if not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource name.
    pub name: String,
    /// Resource kind.
    pub kind: ResourceKind,
    /// Canonical URL (optional, generated if not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    /// Base definition (e.g., "Patient" for profiles).
    #[serde(alias = "baseResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Source format preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_format: Option<SourceFormat>,
    /// Description (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Context for extensions (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Purpose for valuesets (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Raw JSON content for extension/valueset (optional).
    /// If provided, will be parsed and saved directly instead of generating from template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn create_test_service() -> (ProjectService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let service = ProjectService::new(temp_dir.path());
        (service, temp_dir)
    }

    #[tokio::test]
    async fn test_create_project() {
        let (service, _temp_dir) = create_test_service().await;

        let request = CreateProjectRequest {
            id: "my-ig".to_string(),
            name: "My IG".to_string(),
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: Some(FhirVersion::R4),
            description: Some("A test IG".to_string()),
            publisher: None,
            dependencies: None,
        };

        let project = service.create_project(request).await.unwrap();
        assert_eq!(project.id, "my-ig");
        assert_eq!(project.name, "My IG");

        // Verify directory structure
        let project_dir = service.project_path("my-ig");
        assert!(project_dir.exists());
        assert!(project_dir.join("IR/resources").exists());
        assert!(project_dir.join("SD/StructureDefinition").exists());
        assert!(project_dir.join("FSH/profiles").exists());
    }

    #[tokio::test]
    async fn test_load_project() {
        let (service, _temp_dir) = create_test_service().await;

        let request = CreateProjectRequest {
            id: "my-ig".to_string(),
            name: "My IG".to_string(),
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: None,
            description: None,
            publisher: None,
            dependencies: None,
        };

        let _created = service.create_project(request).await.unwrap();
        let loaded = service.load_project("my-ig").await.unwrap();

        assert_eq!(loaded.id, "my-ig");
        assert_eq!(loaded.name, "My IG");
    }

    #[tokio::test]
    async fn test_add_resource() {
        let (service, _temp_dir) = create_test_service().await;

        // Create project
        let request = CreateProjectRequest {
            id: "my-ig".to_string(),
            name: "My IG".to_string(),
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: None,
            description: None,
            publisher: None,
            dependencies: None,
        };
        service.create_project(request).await.unwrap();

        // Add resource
        let add_request = AddResourceRequest {
            id: None,
            name: "MyPatient".to_string(),
            kind: ResourceKind::Profile,
            canonical_url: None,
            base: Some("Patient".to_string()),
            source_format: None,
            description: None,
            context: None,
            purpose: None,
            content: None,
        };

        let resource = service.add_resource("my-ig", add_request).await.unwrap();
        assert_eq!(resource.name, "MyPatient");
        assert_eq!(resource.kind, ResourceKind::Profile);
        assert!(resource.canonical_url.contains("StructureDefinition/MyPatient"));

        // Verify IR document was created
        let doc = service.load_resource_document("my-ig", &resource.id).await.unwrap();
        assert_eq!(doc.metadata.name, "MyPatient");
    }

    #[tokio::test]
    async fn test_list_resources() {
        let (service, _temp_dir) = create_test_service().await;

        // Create project
        let request = CreateProjectRequest {
            id: "my-ig".to_string(),
            name: "My IG".to_string(),
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: None,
            description: None,
            publisher: None,
            dependencies: None,
        };
        service.create_project(request).await.unwrap();

        // Add resources
        for name in ["PatientProfile", "ObservationProfile"] {
            let add_request = AddResourceRequest {
                id: None,
                name: name.to_string(),
                kind: ResourceKind::Profile,
                canonical_url: None,
                base: Some("Patient".to_string()),
                source_format: None,
                description: None,
                context: None,
                purpose: None,
                content: None,
            };
            service.add_resource("my-ig", add_request).await.unwrap();
        }

        let resources = service.list_resources("my-ig").await.unwrap();
        assert_eq!(resources.len(), 2);
    }

    #[tokio::test]
    async fn test_remove_resource() {
        let (service, _temp_dir) = create_test_service().await;

        // Create project and add resource
        let request = CreateProjectRequest {
            id: "my-ig".to_string(),
            name: "My IG".to_string(),
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: None,
            description: None,
            publisher: None,
            dependencies: None,
        };
        service.create_project(request).await.unwrap();

        let add_request = AddResourceRequest {
            id: None,
            name: "MyPatient".to_string(),
            kind: ResourceKind::Profile,
            canonical_url: None,
            base: Some("Patient".to_string()),
            source_format: None,
            description: None,
            context: None,
            purpose: None,
            content: None,
        };
        let resource = service.add_resource("my-ig", add_request).await.unwrap();

        // Remove resource
        let removed = service.remove_resource("my-ig", &resource.id).await.unwrap();
        assert_eq!(removed.id, resource.id);

        // Verify it's gone
        let resources = service.list_resources("my-ig").await.unwrap();
        assert!(resources.is_empty());
    }
}
