# Comply Prompts

AI prompts for the Comply phase of OpenSecKit.

## Main Commands

| Prompt | Command | Description |
|--------|---------|-------------|
| `comply.md` | `/osk-comply` | Run autonomous compliance assessment |
| `list.md` | `/osk-comply-list` | List available frameworks |
| `status.md` | `/osk-comply-status` | Multi-framework compliance dashboard |

## Usage

```bash
# Run RGPD assessment
/osk-comply rgpd

# Run RGS assessment
/osk-comply rgs

# Check status across frameworks
/osk-comply-status
```

## Framework-Specific Prompts

Each framework has its own 5-phase workflow in `frameworks/<name>/prompts/`:

**RGPD Phases:**
1. Processing inventory
2. DPIA/AIPD assessment
3. Requirements assessment
4. Gap analysis
5. Document generation

**RGS Phases:**
1. Security level assessment
2. EBIOS RM risk analysis
3. Requirements assessment
4. Gap analysis
5. Homologation dossier

See framework READMEs for detailed workflow documentation.
