# Discover Schemas

YAML schemas defining data structures for the Discover phase.

## Schemas

| Schema | Description |
|--------|-------------|
| `workflow-state.yaml` | Tracks multi-phase discovery workflow state |

## System Model Structure

The discover phase generates 11+ YAML files in `.osk/system-model/`:

| File | Purpose |
|------|---------|
| `index.yaml` | System overview, project metadata |
| `business.yaml` | Business objectives, compliance scope |
| `architecture.yaml` | Components, services, infrastructure |
| `data.yaml` | Data inventory, classification |
| `actors.yaml` | Users, systems, external entities |
| `boundaries.yaml` | Trust boundaries, security zones |
| `integrations.yaml` | External APIs, services |
| `controls.yaml` | Existing security controls |
| `tooling.yaml` | Dev tools, CI/CD, monitoring |
| `team.yaml` | Team structure, security roles |
| `gaps.yaml` | Identified coverage gaps |

Schemas follow JSON Schema draft 2020-12 format.
