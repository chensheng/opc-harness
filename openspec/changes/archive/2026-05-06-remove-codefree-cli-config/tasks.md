## 1. Preparation and Verification

- [x] 1.1 Verify no active references to `.codefree-cli/` files in codebase
- [x] 1.2 Confirm OpenSpec workflow functions with `.lingma/skills/` only
- [x] 1.3 Document current Git status for audit trail

## 2. File Removal

- [x] 2.1 Remove `.codefree-cli/commands/` directory (4 TOML files)
- [x] 2.2 Remove `.codefree-cli/skills/` directory (4 SKILL.md files)
- [x] 2.3 Verify `.codefree-cli/` directory is completely removed

## 3. Git Commit

- [x] 3.1 Stage deleted files: `git rm -r .codefree-cli/`
- [x] 3.2 Create commit with descriptive message referencing this change
- [x] 3.3 Verify commit includes only intended deletions

## 4. Post-Removal Validation

- [x] 4.1 Run `openspec list` to confirm workflow still functional
- [x] 4.2 Test one OpenSpec command (e.g., `openspec --help`)
- [x] 4.3 Verify no broken references in documentation

## 5. Documentation Update

- [x] 5.1 Update this tasks.md to mark all completed tasks
- [x] 5.2 Prepare quality-check.md assessment
- [x] 5.3 Prepare runtime-check.md validation
