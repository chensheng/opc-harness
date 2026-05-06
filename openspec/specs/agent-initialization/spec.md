## MODIFIED Requirements

### Requirement: Initializer Agent Git configuration
The Initializer Agent SHALL configure Git user information at the project level only if global configuration is missing, and SHALL NOT attempt to initialize Git repository as this is now handled during project creation.

**Previous behavior**: Initializer Agent checked for global Git config and set project-level defaults, but did not execute `git init`.

**Updated behavior**: Initializer Agent skips Git initialization entirely (assumes already initialized), and only ensures user configuration is present.

#### Scenario: Initializer Agent runs on a project with existing Git repo
- **WHEN** Initializer Agent executes on a newly created project
- **THEN** agent verifies `.git` directory exists, checks Git user config, sets defaults if missing, and continues with other initialization tasks

#### Scenario: Initializer Agent runs when global Git config exists
- **WHEN** user has global `user.name` and `user.email` configured
- **THEN** Initializer Agent detects global config and does not set project-level defaults

#### Scenario: Initializer Agent runs when global Git config is missing
- **WHEN** user has no global Git configuration
- **THEN** Initializer Agent sets project-level `user.name` to "OPC-HARNESS User" and `user.email` to "harness@opc.local"

### Requirement: Worktree Manager Git initialization check
The Worktree Manager SHALL verify that the Git repository is initialized before creating worktrees, but SHALL NOT perform initialization as this is now guaranteed by project creation.

**Previous behavior**: Worktree Manager executed `git init` if `.git` directory was missing.

**Updated behavior**: Worktree Manager assumes Git is already initialized (guaranteed by project creation) and only validates the repository state.

#### Scenario: Create worktree on a properly initialized project
- **WHEN** Worktree Manager creates a worktree for an agent
- **THEN** manager confirms `.git` directory exists and proceeds with `git worktree add` command

#### Scenario: Worktree creation on a project without Git (edge case)
- **WHEN** Worktree Manager attempts to create a worktree but `.git` directory is missing
- **THEN** manager returns an error indicating Git repository is not initialized, suggesting manual initialization or project recreation
