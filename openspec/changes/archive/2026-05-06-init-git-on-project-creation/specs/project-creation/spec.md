## MODIFIED Requirements

### Requirement: Project creation workflow
The system SHALL create a new project with a unique ID, workspace directory, and automatically initialized Git repository with `.gitignore` file excluding `.opc-harness/` directory and default user configuration.

**Previous behavior**: System created workspace directory only, without Git initialization.

**Updated behavior**: System creates workspace directory, initializes Git repository with `.gitignore` file (ignoring `.opc-harness/`), creates initial commit, and ensures Git user configuration is set.

#### Scenario: Create project with automatic Git setup
- **WHEN** user invokes `create_project` with name "My Project" and description "Test project"
- **THEN** system generates a UUID, creates workspace at `~/.opc-harness/workspaces/{uuid}/`, executes `git init`, creates `.gitignore` file with `.opc-harness/` entry, creates initial empty commit, sets default Git user config if needed, saves project to database, and returns the project ID

#### Scenario: Project creation with existing global Git config
- **WHEN** user has global Git configuration and creates a new project
- **THEN** system uses the global Git configuration and does not override with defaults

#### Scenario: Project creation when Git is not installed
- **WHEN** user creates a project but Git is not available on the system
- **THEN** system logs a warning about Git initialization failure, still creates the project successfully, and allows manual Git initialization later via GitDetector
