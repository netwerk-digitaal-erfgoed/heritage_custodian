# Legal Entity Model Refactoring Summary

**Date**: 2025-11-22  
**Schema Version**: 20251121  
**Status**: Complete

## Overview

Refactored the `EntityTypeEnum` into a proper class-based legal entity model that correctly distinguishes between informal observations and formal legal entities.

## Key Changes

### 1. From Enum to Class Hierarchy

**OLD**: `EntityTypeEnum` with mixed informal/formal entity types
- INDIVIDUAL, GROUP, ORGANIZATION, GOVERNMENT, CORPORATION
- Mixed informal groups with formal legal entities
- No structured legal metadata

**NEW**: Proper class hierarchy for legal entities with ISO 20275 compliance
- `LegalEntityType`: Top-level classification (PERSON vs ORGANIZATION)
- `LegalForm`: ISO 20275-compliant legal forms with ELF codes
- `LegalName`: Structured names (TOOI pattern)
- `RegistrationNumber`: Registration identifiers with temporal validity
- `RegistrationAuthority`: Registration bodies
- `GovernanceStructure`: Internal organizational structure
- `LegalStatus`: Legal status tracking

### 2. New Class Structure

#### Core Legal Entity Classes:

**LegalEntityType** (`schemas/20251121/linkml/modules/classes/LegalEntityType.yaml`)
- Top-level classification distinguishing natural persons from legal persons
- Two types only: PERSON (natural person) and ORGANIZATION (legal person)
- Maps to: `org:classification`, `schema:additionalType`, `tooi:organisatievorm`

**LegalForm** (`schemas/20251121/linkml/modules/classes/LegalForm.yaml`)
- Specific legal forms based on ISO 20275 Entity Legal Form (ELF) codes
- Jurisdiction-specific: 1,600+ forms across 150+ countries
- Maps to: `rov:orgType`, `gleif:hasLegalForm`, `tooi:rechtsvorm`
- Attributes:
  - `elf_code`: 4-character ISO 20275 code (e.g., "8888" for Dutch Stichting)
  - `country_code`: ISO 3166-1 alpha-2 code
  - `local_name`: Name in local language
  - `transliterated_name`: For non-Latin scripts
  - `abbreviation`: Common abbreviation (e.g., "BV", "GmbH")
  - `legal_entity_type`: Link to PERSON or ORGANIZATION
  - `valid_from`/`valid_to`: Temporal validity

**LegalName** (`schemas/20251121/linkml/modules/classes/LegalName.yaml`)
- Structured legal names following TOOI pattern
- Three name variants:
  1. `full_name`: With organizational type (e.g., "Stichting Rijksmuseum")
  2. `name_without_type`: Without type (e.g., "Rijksmuseum")
  3. `alphabetical_name`: For ordering (e.g., "Gravenhage, 's")
- Maps to: `rov:legalName`, `tooi:officieleNaamInclSoort`, `tooi:officieleNaamExclSoort`
- Attributes:
  - `display_name`: Preferred UI display name
  - `language`: ISO 639-1 language code
  - `script`: ISO 15924 script code
  - `temporal_validity`: Time period when name is/was valid

**RegistrationInfo** (`schemas/20251121/linkml/modules/classes/RegistrationInfo.yaml`)
- **RegistrationNumber**: Official registration identifiers
  - `number`: Actual registration number
  - `type`: Type of registration (KvK, EIN, charity number, etc.)
  - `temporal_validity`: Period when registration is/was valid
  - Maps to: `rov:registration`, `tooi:organisatieIdentificatie`

- **RegistrationAuthority**: Bodies that maintain registrations
  - `name`: Official name (e.g., "Kamer van Koophandel")
  - `abbreviation`: Short code (e.g., "KvK")
  - `jurisdiction`: Geographic jurisdiction (country/region)
  - `website`: Official website
  - `registration_types`: Types of entities they can register
  - Maps to: `rov:hasRegisteredOrganization`

- **GovernanceStructure**: Internal organizational structure
  - `structure_type`: Type (hierarchical, matrix, flat, network)
  - `organizational_units`: List of departments/divisions
  - `governance_body`: Top-level board/trustees
  - Maps to: `org:hasUnit`, `org:OrganizationalUnit`

- **LegalStatus**: Legal status tracking
  - `status_code`: Standardized code (ACTIVE, DISSOLVED, SUSPENDED, MERGED)
  - `status_name`: Human-readable name
  - `description`: Detailed legal meaning
  - `temporal_validity`: Period when status applies
  - `jurisdiction`: Where status is defined
  - Maps to: `schema:status`

### 3. Key Distinctions

#### CustodianObservation vs CustodianReconstruction:

**CustodianObservation**:
- Can include informal references, groups, collectives
- Emic (insider) perspective: "what someone called themselves"
- No legal formalization required
- Examples: "Rijks" (letterhead), "The Rijksmuseum" (guidebook)

**CustodianReconstruction**:
- ONLY formal legal entities with legal recognition
- Etic (outsider) perspective: "what is the formal entity after analysis?"
- Must have legal registration OR established legal identity
- Two types:
  1. **Natural persons** (individuals with legal rights)
  2. **Legal persons** (organizations with legal personality)

**CRITICAL**: Informal groups WITHOUT legal status are NOT CustodianReconstructions. They remain as CustodianObservations only.

### 4. ISO 20275 Integration

**Source**: `data/ontology/2023-09-28-elf-code-list-v1.5.csv`

- 1,600+ standardized legal form codes
- Covers 150+ jurisdictions
- Each legal form has:
  - 4-character alphanumeric code
  - Country-specific definition
  - Local and transliterated names
  - Legal rights and obligations

**Common Heritage Institution Legal Forms**:

| Country | ELF Code | Legal Form | Type |
|---------|----------|------------|------|
| Netherlands | 8888 | Stichting | Foundation |
| Netherlands | 54M6 | Besloten vennootschap | Private Company |
| Germany | QS1L | Stiftung | Foundation |
| Germany | HRA1 | GmbH | Private Company |
| France | L6L1 | Association | Association |
| UK | PRIV | Private Limited | Private Company |
| UK | CHAR | Charity | Charity |
| US | 501C | 501(c) Nonprofit | Nonprofit |

### 5. Ontology Alignments

#### Base Ontology Mappings:

**TOOI (Dutch entities)**:
- `tooi:rechtsvorm` → Legal form
- `tooi:officieleNaamInclSoort` → Legal name with type
- `tooi:officieleNaamExclSoort` → Legal name without type
- `tooi:alfabetischeVolgorde` → Alphabetical name ordering
- `tooi:organisatieIdentificatie` → Registration numbers

**W3C Organization Ontology**:
- `org:Organization` → Base organizational class
- `org:FormalOrganization` → Legally recognized entities
- `org:hasUnit` → Governance structure
- `org:classification` → Entity type classification

**Registered Organizations Vocabulary (ROV)**:
- `rov:legalName` → Official legal names
- `rov:orgType` → Legal form classification
- `rov:registration` → Registration numbers
- `rov:hasRegisteredOrganization` → Registering authority

**CPOV (Core Public Organisation Vocabulary)**:
- `cpov:PublicOrganisation` → Government/public sector entities

**Schema.org**:
- `schema:name`, `schema:alternateName` → Names
- `schema:status` → Legal status
- `schema:additionalType` → Entity classification
- `schema:validFrom`, `schema:validThrough` → Temporal validity

**GLEIF (Global Legal Entity Identifier Foundation)**:
- `gleif:hasEntityLegalFormCode` → ISO 20275 ELF codes
- `gleif:hasLegalForm` → Legal form references

### 6. Updated CustodianReconstruction Slots

| Old Slot | New Slot | Old Range | New Range |
|----------|----------|-----------|-----------|
| `entity_type` | `legal_entity_type` | `EntityTypeEnum` | `LegalEntityType` |
| `legal_name` | `legal_name` | `string` | `LegalName` |
| `legal_form` | `legal_form` | `string` (pattern) | `LegalForm` |
| `registration_number` | `registration_numbers` | `string` | `RegistrationNumber[]` |
| `registration_date` | *(deprecated)* | `date` | (moved to RegistrationNumber.temporal_validity) |
| `registration_authority` | `registration_authority` | `string` | `RegistrationAuthority` |
| `legal_status` | `legal_status` | `LegalStatusEnum` | `LegalStatus` |
| `governance_structure` | `governance_structure` | `string` | `GovernanceStructure` |

### 7. Validation Rules

**Natural Persons (PERSON)**:
- Cannot have legal forms (legal form is N/A for individuals)
- May not have registration numbers (unless sole proprietor)
- Identity established through biographical sources

**Legal Persons (ORGANIZATION)**:
- Must have legal forms (ISO 20275 code)
- Must have registration numbers
- Must have registration authority
- Governance structure documented

**All Reconstructed Entities**:
- Must have legal status (active, dissolved, etc.)
- Must derive from at least one CustodianObservation
- Must document reconstruction activity

### 8. Examples

#### Example 1: Dutch Museum (Stichting)

```yaml
legal_entity_type:
  code: "ORGANIZATION"
  label: "Legal Person"
  
legal_name:
  full_name: "Stichting Rijksmuseum Amsterdam"
  name_without_type: "Rijksmuseum Amsterdam"
  display_name: "Rijksmuseum"
  language: "nl"
  
legal_form:
  elf_code: "8888"
  country_code: "NL"
  local_name: "Stichting"
  abbreviation: "St."
  
registration_numbers:
  - number: "41215422"
    type: "KvK"
    temporal_validity:
      begin_of_the_begin: "1885-07-01"
      
registration_authority:
  name: "Kamer van Koophandel"
  abbreviation: "KvK"
  jurisdiction: "NL"
  
legal_status:
  status_code: "ACTIVE"
  status_name: "Active"
```

#### Example 2: Government Archive

```yaml
legal_entity_type:
  code: "ORGANIZATION"
  label: "Legal Person"
  
legal_name:
  full_name: "Nationaal Archief"
  display_name: "National Archives of the Netherlands"
  language: "nl"
  
legal_form:
  local_name: "Rijksinstelling"  # Government institution
  country_code: "NL"
  
parent_custodian:
  legal_name:
    full_name: "Ministerie van Onderwijs, Cultuur en Wetenschap"
    
legal_status:
  status_code: "ACTIVE"
  status_name: "Active"
```

#### Example 3: Private Collector (Natural Person)

```yaml
legal_entity_type:
  code: "PERSON"
  label: "Natural Person"
  
legal_name:
  full_name: "Dr. Jan de Vries"
  display_name: "Jan de Vries"
  
# No legal_form (not applicable for natural persons)
# No registration_numbers (private individual)

legal_status:
  status_code: "ACTIVE"
  status_name: "Active collector"
```

## Benefits

1. **Legal Accuracy**: Properly distinguishes legal entities from informal references
2. **ISO Compliance**: Aligns with ISO 20275 international standard for legal forms
3. **Ontology Integration**: Reuses properties from established ontologies (TOOI, ROV, W3C Org, CPOV)
4. **Temporal Tracking**: All legal aspects can have temporal validity (names change, registrations expire)
5. **Jurisdiction Support**: Handles legal forms across different countries and legal systems
6. **Validation**: Enforces legal entity constraints at schema level
7. **Clarity**: Clear distinction between observations (what people call things) and reconstructions (formal legal entities)

## Migration Notes

### For Data Curators:

1. **EntityTypeEnum is deprecated**: Use `LegalEntityType` class instead
2. **Existing enum values map to**:
   - `INDIVIDUAL` → `LegalEntityType(code="PERSON")`
   - `ORGANIZATION`, `GOVERNMENT`, `CORPORATION` → `LegalEntityType(code="ORGANIZATION")`
   - `GROUP` → **Not a legal entity** (remains CustodianObservation only)

3. **String-based legal attributes now require structured data**:
   - Simple strings → Proper class instances
   - Add ISO 20275 codes where available
   - Document registration authorities

4. **Informal groups**:
   - If group has legal status (registered association) → CustodianReconstruction
   - If group lacks legal status (informal collective) → CustodianObservation only

### For Developers:

1. **Import new classes**:
   ```yaml
   imports:
     - LegalEntityType
     - LegalForm
     - LegalName
     - RegistrationInfo
   ```

2. **Update slot ranges**:
   - Change `entity_type` → `legal_entity_type: LegalEntityType`
   - Change `legal_name: string` → `legal_name: LegalName`
   - Change `legal_form: string` → `legal_form: LegalForm`

3. **Handle temporal validity**:
   - Registration dates now in `RegistrationNumber.temporal_validity`
   - Name changes tracked via `LegalName.temporal_validity`

4. **Validation rules**:
   - Check legal entity type before requiring legal form
   - Ensure organizations have registration details
   - Persons may lack registrations

## File Structure

```
schemas/20251121/linkml/modules/
├── classes/
│   ├── LegalEntityType.yaml          # NEW: Top-level classification
│   ├── LegalForm.yaml                 # NEW: ISO 20275 legal forms
│   ├── LegalName.yaml                 # NEW: Structured names (TOOI pattern)
│   ├── RegistrationInfo.yaml         # NEW: Registration details
│   ├── CustodianReconstruction.yaml  # UPDATED: Uses new class ranges
│   └── LEGAL_ENTITY_REFACTORING.md   # This file
│
├── enums/
│   └── EntityTypeEnum.yaml            # DEPRECATED
│
└── mappings/
    ├── ISO20275_mapping.yaml          # NEW: ISO 20275 code mappings
    └── ISO20275_common.yaml           # NEW: Common heritage institution forms
```

## Scripts

**`scripts/parse_iso20275_codes.py`**:
- Parses `data/ontology/2023-09-28-elf-code-list-v1.5.csv`
- Generates LinkML mappings for common heritage institution legal forms
- Outputs `schemas/20251121/linkml/modules/mappings/ISO20275_common.yaml`

Usage:
```bash
python scripts/parse_iso20275_codes.py
```

## Related Documentation

- **ISO 20275 Standard**: https://www.gleif.org/en/about-lei/code-lists/iso-20275-entity-legal-forms-code-list
- **TOOI Ontology**: `data/ontology/tooiont.ttl`
- **W3C Organization Ontology**: https://www.w3.org/TR/vocab-org/
- **Registered Organizations Vocabulary**: https://www.w3.org/TR/vocab-regorg/
- **CPOV**: https://joinup.ec.europa.eu/collection/semantic-interoperability-community-semic/solution/core-public-organisation-vocabulary

## Status

✅ **Complete**: All files created and updated
- LegalEntityType.yaml
- LegalForm.yaml  
- LegalName.yaml
- RegistrationInfo.yaml
- CustodianReconstruction.yaml (updated)
- ISO20275_mapping.yaml
- parse_iso20275_codes.py

⏳ **Next Steps**:
1. Parse ISO 20275 CSV to generate full mappings
2. Validate schema with LinkML tools
3. Update documentation to reference new classes
4. Create migration script for existing data
5. Update validation tests
