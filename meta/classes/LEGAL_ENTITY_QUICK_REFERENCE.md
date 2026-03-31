# Legal Entity Refactoring - Quick Reference

**Date**: 2025-11-22  
**Schema Version**: 20251121

## What Changed?

The `EntityTypeEnum` has been replaced with a proper class-based legal entity model.

## Key Classes

### 1. LegalEntityType
Top-level classification: PERSON or ORGANIZATION
```yaml
legal_entity_type:
  code: "ORGANIZATION"  # or "PERSON"
  label: "Legal Person"
```

### 2. LegalForm
ISO 20275 legal forms (1,600+ codes, 150+ jurisdictions)
```yaml
legal_form:
  elf_code: "8888"              # ISO 20275 code
  country_code: "NL"            # ISO 3166-1
  local_name: "Stichting"       # Local language name
  abbreviation: "St."           # Common abbreviation
```

### 3. LegalName
Structured names (TOOI pattern)
```yaml
legal_name:
  full_name: "Stichting Rijksmuseum Amsterdam"
  name_without_type: "Rijksmuseum Amsterdam"
  display_name: "Rijksmuseum"
  language: "nl"
```

### 4. RegistrationNumber
Registration identifiers with temporal validity
```yaml
registration_numbers:
  - number: "41215422"
    type: "KvK"
    temporal_validity:
      begin_of_the_begin: "1885-07-01"
```

### 5. RegistrationAuthority
Bodies that register organizations
```yaml
registration_authority:
  name: "Kamer van Koophandel"
  abbreviation: "KvK"
  jurisdiction: "NL"
```

### 6. GovernanceStructure
Internal organizational structure
```yaml
governance_structure:
  structure_type: "hierarchical"
  description: "Board of trustees with director-led departments"
```

### 7. LegalStatus
Current legal status
```yaml
legal_status:
  status_code: "ACTIVE"
  status_name: "Active"
  temporal_validity:
    begin_of_the_begin: "1885-07-01"
```

## Critical Rules

### Natural Persons (PERSON)
- ❌ Cannot have `legal_form` (N/A for individuals)
- ⚠️ May not have `registration_numbers` (unless sole proprietor)
- ✅ Identity established through biographical sources

### Legal Persons (ORGANIZATION)
- ✅ Must have `legal_form` (ISO 20275 code)
- ✅ Must have `registration_numbers`
- ✅ Must have `registration_authority`
- ✅ Governance structure documented

### Informal Groups
- ❌ **NOT** CustodianReconstruction if they lack legal status
- ✅ Remain as CustodianObservation only
- ✅ Upgrade to CustodianReconstruction if they become registered (e.g., association)

## Migration Quick Guide

| Old | New |
|-----|-----|
| `entity_type: INDIVIDUAL` | `legal_entity_type: {code: "PERSON"}` |
| `entity_type: ORGANIZATION` | `legal_entity_type: {code: "ORGANIZATION"}` |
| `entity_type: GOVERNMENT` | `legal_entity_type: {code: "ORGANIZATION"}` |
| `entity_type: CORPORATION` | `legal_entity_type: {code: "ORGANIZATION"}` |
| `entity_type: GROUP` | **Remove from CustodianReconstruction** (informal groups stay as observations) |
| `legal_name: "string"` | `legal_name: {full_name: "string", ...}` |
| `legal_form: "string"` | `legal_form: {elf_code: "8888", ...}` |
| `registration_number: "string"` | `registration_numbers: [{number: "string", ...}]` |
| `registration_authority: "string"` | `registration_authority: {name: "string", ...}` |

## Ontology Mappings

| Class | Primary Ontology Mapping |
|-------|--------------------------|
| LegalEntityType | `org:classification` |
| LegalForm | `rov:orgType` |
| LegalName | `rov:legalName` |
| RegistrationNumber | `rov:registration` |
| RegistrationAuthority | `rov:hasRegisteredOrganization` |
| GovernanceStructure | `org:hasUnit` |
| LegalStatus | `schema:status` |

## Common ISO 20275 Codes for Heritage Institutions

| Country | Code | Legal Form | Use Case |
|---------|------|------------|----------|
| NL | 8888 | Stichting | Dutch foundations (most museums) |
| NL | 54M6 | Besloten vennootschap (BV) | Dutch private companies |
| DE | QS1L | Stiftung | German foundations |
| DE | HRA1 | GmbH | German private companies |
| FR | L6L1 | Association | French associations |
| UK | CHAR | Charity | UK charities |
| US | 501C | 501(c) Nonprofit | US nonprofits |

## Files Created

```
schemas/20251121/linkml/modules/
├── classes/
│   ├── LegalEntityType.yaml          ✅ NEW
│   ├── LegalForm.yaml                 ✅ NEW
│   ├── LegalName.yaml                 ✅ NEW
│   ├── RegistrationInfo.yaml         ✅ NEW
│   ├── CustodianReconstruction.yaml  ✅ UPDATED
│   └── LEGAL_ENTITY_REFACTORING.md   ✅ NEW (detailed docs)
│
└── mappings/
    └── ISO20275_mapping.yaml          ✅ NEW

scripts/
└── parse_iso20275_codes.py            ✅ NEW
```

## See Also

- Full documentation: `schemas/20251121/linkml/modules/classes/LEGAL_ENTITY_REFACTORING.md`
- ISO 20275 CSV: `data/ontology/2023-09-28-elf-code-list-v1.5.csv`
- TOOI ontology: `data/ontology/tooiont.ttl`
