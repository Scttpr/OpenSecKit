# Secure Knowledge Base

Reference materials and guides for implementing the 7 Security Principles.

## Structure

```
knowledge/
├── principles/           # 7 Security Principles documentation
│   ├── I-threat-modeling/
│   ├── II-risk-analysis/
│   ├── III-security-design/
│   ├── IV-security-testing/
│   ├── V-secrets-management/
│   ├── VI-audit-logging/
│   └── VII-patch-management/
├── libraries/            # Reusable threat and control libraries
│   ├── threats/
│   └── controls/
└── mappings/             # Framework mappings (ASVS, etc.)
```

## The 7 Security Principles

1. **Threat Modeling** - STRIDE, DFD, Attack Trees
2. **Risk Analysis** - Risk scoring, registers
3. **Security by Design** - OWASP ASVS, secure requirements
4. **Security Testing** - SAST, DAST, SCA
5. **Secrets Management** - Vault, detection, rotation
6. **Audit Logging** - Centralized, immutable, SIEM-ready
7. **Patch Management** - Dependency scanning, SLA enforcement

See each principle's README for detailed guides.
