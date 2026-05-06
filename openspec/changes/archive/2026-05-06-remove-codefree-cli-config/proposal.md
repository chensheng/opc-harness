## Why

The `.codefree-cli` directory contains outdated OpenSpec command definitions and skill files that are no longer needed. These files were part of an earlier CLI configuration approach but have been superseded by the current Lingma-based OpenSpec workflow. Removing these obsolete files will reduce repository clutter and prevent confusion about which OpenSpec tools to use.

## What Changes

- Remove `.codefree-cli/commands/` directory containing old OpenSpec command TOML definitions (opsx-apply.toml, opsx-archive.toml, opsx-explore.toml, opsx-propose.toml)
- Remove `.codefree-cli/skills/` directory containing old skill definition files (openspec-apply-change/SKILL.md, openspec-archive-change/SKILL.md, openspec-explore/SKILL.md, openspec-propose/SKILL.md)
- Total: 8 files deleted, 1244 lines removed

## Capabilities

### New Capabilities
<!-- No new capabilities introduced -->

### Modified Capabilities
<!-- No existing capability requirements are changing - this is a cleanup operation -->

## Impact

- **Affected Files**: 
  - `.codefree-cli/commands/*.toml` (4 files)
  - `.codefree-cli/skills/*/SKILL.md` (4 files)
- **Breaking Changes**: None - these are internal tooling files not used in current workflow
- **Dependencies**: No dependency changes
- **Systems**: Repository structure cleanup only; no impact on application functionality or OpenSpec workflow
