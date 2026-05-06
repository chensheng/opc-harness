## Context

The project currently uses Lingma-based OpenSpec workflow with skills located in `.lingma/skills/`. The `.codefree-cli` directory contains legacy command definitions and skill files from a previous CLI tooling approach. These files are no longer referenced or used by the current OpenSpec workflow, which is managed through the `@fission-ai/openspec` npm package and Lingma integration.

Current state:
- Active OpenSpec workflow: `.lingma/skills/openspec-*` (4 skill files)
- Legacy files to remove: `.codefree-cli/commands/` and `.codefree-cli/skills/` (8 files total)
- No code references these legacy files

Constraints:
- Must ensure no active dependencies on these files before removal
- Git history will preserve the files for future reference if needed
- No migration path required as these were never part of production functionality

## Goals / Non-Goals

**Goals:**
- Remove all files under `.codefree-cli/` directory
- Clean up repository structure to reflect current tooling
- Maintain Git history for audit trail

**Non-Goals:**
- No changes to active `.lingma/skills/` directory
- No modifications to OpenSpec configuration or workflows
- No impact on application code or tests

## Decisions

### Decision 1: Direct Deletion vs. Deprecation Period

**Choice**: Direct deletion without deprecation period

**Rationale**: 
- These files are internal tooling configurations, not public APIs
- No evidence of active usage in current workflow
- Lingma-based skills in `.lingma/skills/` are the active implementation
- Keeping deprecated files creates confusion about which tools to use

**Alternatives Considered**:
- Add deprecation notices: Rejected - adds maintenance burden for unused files
- Move to archive directory: Rejected - Git history already serves as archive

### Decision 2: Directory Removal Strategy

**Choice**: Remove entire `.codefree-cli/` directory tree

**Rationale**:
- All files under this directory are obsolete
- Cleaner than selective file deletion
- Prevents orphaned empty directories

**Implementation**:
```bash
git rm -r .codefree-cli/
```

## Risks / Trade-offs

**[Risk] Accidental removal of active files** → Mitigation: Verified via `git status` that only `.codefree-cli/` files are staged for deletion; cross-referenced with active workflow documentation

**[Risk] Future need for reference** → Mitigation: Git history preserves all deleted files; can restore via `git checkout <commit> -- .codefree-cli/` if needed

**[Trade-off] Loss of historical context** → Accepted: Files represent superseded approach; current workflow documented in AGENTS.md and openspec/specs/

## Migration Plan

No migration required. This is a cleanup operation:

1. **Pre-deployment**: None - files are not in active use
2. **Deployment**: Commit deletion to Git
3. **Post-deployment**: Verify OpenSpec workflow still functions with `.lingma/skills/`

**Rollback Strategy**: If issues arise (unlikely), restore from Git:
```bash
git revert <commit-hash>
```

## Open Questions

None - this is a straightforward cleanup with no architectural decisions pending.
