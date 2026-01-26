# Discover Prompts

AI prompts for the Discover phase of OpenSecKit.

## Main Commands

| Prompt | Command | Description |
|--------|---------|-------------|
| `discover.md` | `/osk-discover` | Build or update system model + generate docs |
| `validate.md` | `/osk-discover-validate` | Validate model integrity |

## Orchestrated Phases

The discover workflow runs through 6 internal phases (not exposed as commands):

1. `01-product-context.md` - Product/business context extraction
2. `02-architecture.md` - Architecture analysis
3. `03-domain-model.md` - Domain model discovery
4. `04-ecosystem.md` - Integrations and ecosystem mapping
5. `05-operations.md` - Operations and deployment analysis
6. `06-synthesis.md` - System model synthesis

These phases are orchestrated by `discover.md` and should not be run directly.

See the [Discover Phase documentation](../README.md) for workflow details.
