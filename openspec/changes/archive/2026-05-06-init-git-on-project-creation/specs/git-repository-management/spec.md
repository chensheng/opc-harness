## ADDED Requirements

### Requirement: Git repository status check
The system SHALL provide a command to check the Git repository status of a given directory, including whether it is a Git repository, current branch, commit count, and working directory state.

#### Scenario: Check status of a valid Git repository
- **WHEN** user invokes `check_git_status` with a path containing a Git repository
- **THEN** system returns `isGitRepo: true`, current branch name, commit count, and dirty state

#### Scenario: Check status of a non-Git directory
- **WHEN** user invokes `check_git_status` with a path that is not a Git repository
- **THEN** system returns `isGitRepo: false` with null values for branch and commit count

#### Scenario: Check status of a non-existent directory
- **WHEN** user invokes `check_git_status` with a path that does not exist
- **THEN** system returns an error message indicating the directory does not exist

### Requirement: Git repository initialization
The system SHALL provide a command to initialize a new Git repository in a specified directory, creating a `.git` folder, a `.gitignore` file with OPC-HARNESS specific ignore rules, and an initial empty commit.

#### Scenario: Initialize Git in a new directory
- **WHEN** user invokes `init_git_repo` with a valid directory path and branch name
- **THEN** system creates a `.git` directory, creates a `.gitignore` file ignoring `.opc-harness/` directory, sets the initial branch, and creates an empty initial commit

#### Scenario: Initialize Git in an existing Git repository
- **WHEN** user invokes `init_git_repo` on a directory that already contains a `.git` folder
- **THEN** system returns an error indicating the repository is already initialized

#### Scenario: Initialize Git with custom branch name
- **WHEN** user invokes `init_git_repo` with branch name "main"
- **THEN** system sets the default branch to "main" instead of the system default

#### Scenario: Verify .gitignore content after initialization
- **WHEN** user invokes `init_git_repo` on a new directory
- **THEN** system creates `.gitignore` file containing `.opc-harness/` entry to exclude OPC-HARNESS context files from version control

### Requirement: Git configuration management
The system SHALL provide commands to get, set, and retrieve all Git configuration values for a specific repository or globally.

#### Scenario: Set a Git configuration value
- **WHEN** user invokes `set_git_config` with path, key "user.name", and value "John Doe"
- **THEN** system sets the local Git configuration `user.name` to "John Doe" and returns success

#### Scenario: Get a specific Git configuration value
- **WHEN** user invokes `get_git_config` with path and key "user.email"
- **THEN** system returns the configured email address or null if not set

#### Scenario: Get all Git configuration values
- **WHEN** user invokes `get_all_git_config` with a repository path
- **THEN** system returns an object containing `userName` and `userEmail` fields with their configured values

#### Scenario: Set Git config in a non-Git directory
- **WHEN** user invokes `set_git_config` on a directory without a Git repository
- **THEN** system returns an error indicating Git is not initialized

### Requirement: Automatic Git initialization on project creation
The system SHALL automatically initialize a Git repository when a new project is created, immediately after creating the workspace directory.

#### Scenario: Create a new project with automatic Git initialization
- **WHEN** user invokes `create_project` with project name and description
- **THEN** system creates the workspace directory, initializes Git repository, and returns project ID

#### Scenario: Git initialization failure during project creation
- **WHEN** Git initialization fails during `create_project` (e.g., Git not installed)
- **THEN** system logs a warning but still creates the project successfully, allowing manual Git initialization later

#### Scenario: Verify Git initialization after project creation
- **WHEN** user calls `check_git_status` on a newly created project's workspace directory
- **THEN** system confirms the directory is a Git repository with an initial commit

### Requirement: Git initialization on project open
The system SHALL check and initialize Git repository when loading or opening an existing project if the workspace directory exists but is not a Git repository.

#### Scenario: Open project without Git repository
- **WHEN** user opens an existing project (via `get_project_by_id` or navigation to project page)
- **AND** the workspace directory exists but does not contain a `.git` folder
- **THEN** system automatically initializes Git repository, creates `.gitignore` file, and logs the initialization

#### Scenario: Open project with existing Git repository
- **WHEN** user opens an existing project that already has a `.git` folder
- **THEN** system skips Git initialization and proceeds normally

#### Scenario: Open project with missing workspace directory
- **WHEN** user opens a project but the workspace directory does not exist
- **THEN** system recreates the workspace directory and initializes Git repository

#### Scenario: Git initialization failure on project open
- **WHEN** Git initialization fails while opening a project
- **THEN** system logs a warning but allows the project to be opened, enabling manual Git initialization via GitDetector

### Requirement: Git user configuration defaults
The system SHALL ensure Git user configuration (name and email) is set at the project level if global configuration is not available, using sensible defaults.

#### Scenario: Global Git config exists
- **WHEN** user has global `user.name` and `user.email` configured
- **THEN** system uses the global configuration and does not set project-level defaults

#### Scenario: Global Git config missing
- **WHEN** user has no global Git configuration
- **THEN** system sets project-level `user.name` to "OPC-HARNESS User" and `user.email` to "harness@opc.local"

#### Scenario: Partial global Git config
- **WHEN** user has global `user.name` but not `user.email`
- **THEN** system only sets the missing `user.email` at project level
