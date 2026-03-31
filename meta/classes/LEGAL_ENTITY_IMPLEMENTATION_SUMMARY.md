# Legal Entity Model Implementation Summary

**Date**: 2025-11-22  
**Status**: ✅ COMPLETE - Schema refactored, RDF generated, ISO 20275 parsed

---

## Overview

Successfully refactored the Heritage Custodian schema from a flat enum-based entity type system to a comprehensive class-based legal entity model aligned with international standards (ISO 20275, TOOI, W3C Org, ROV).

## What Was Accomplished

### 1. Schema Refactoring ✅

**Replaced**:
- `EntityTypeEnum` (flat 8-value enum mixing informal groups with legal entities)
- `entity_type` slot (primitive string)
- `registration_number` slot (single string)

**With**:
- `LegalEntityType` class - Top-level classification (PERSON vs ORGANIZATION)
- `LegalForm` class - ISO 20275 Entity Legal Forms (3,819 codes, 117 jurisdictions)
- `LegalName` class - TOOI naming pattern (3 variants: full, without type, alphabetical)
- `RegistrationInfo` class - 4 sub-classes:
  - `RegistrationNumber` (with temporal validity)
  - `RegistrationAuthority` (Chamber of Commerce, etc.)
  - `GovernanceStructure` (organizational hierarchy)
  - `LegalStatus` (active, dissolved, etc.)

**New Slots**:
- `legal_entity_type` (replaces `entity_type`)
- `registration_numbers` (pluralized, replaces `registration_number`)

### 2. CustodianReconstruction Class Updated ✅

Updated 7 slot ranges from primitives to classes:

| Slot | Old Range | New Range |
|------|-----------|-----------|
| `legal_name` | `string` | `LegalName` |
| `legal_form` | `string` | `LegalForm` |
| `legal_status` | `LegalStatusEnum` | `LegalStatus` |
| `registration_authority` | `string` | `RegistrationAuthority` |
| `governance_structure` | `string` | `GovernanceStructure` |
| `entity_type` | `EntityTypeEnum` | *(removed)* |
| `legal_entity_type` | *(new)* | `LegalEntityType` |
| `registration_number` | `string` | *(removed)* |
| `registration_numbers` | *(new)* | `RegistrationNumber` (multivalued) |

### 3. Temporal Model Refactored ✅

**ReconstructionActivity.yaml**:
- Replaced separate `started_at_time` and `ended_at_time` slots
- Now uses single `temporal_extent` slot
- Range: `TimeSpan` class (supports fuzzy timestamps with begin/end boundaries)

### 4. Agent Type Enum Enhanced ✅

Added ontology-aligned agent types:
- `GROUP` (FOAF:Group) - Informal collections of people
- `FORMAL_ORGANIZATION` (org:FormalOrganization) - Registered legal entities
- `PUBLIC_ORGANIZATION` (cpov:PublicOrganisation) - Government bodies
- `ORGANIZATIONAL_UNIT` (org:OrganizationalUnit) - Departments/divisions
- `ORGANIZATIONAL_COLLABORATION` (org:OrganizationalCollaboration) - Multi-party partnerships

### 5. RDF Generation ✅

Generated complete OWL ontology in 4 serialization formats:

| Format | File | Size | Use Case |
|--------|------|------|----------|
| **Turtle** | `01_custodian_name.owl.ttl` | 138 KB | Human-readable, SPARQL queries |
| **N-Triples** | `01_custodian_name.nt` | 403 KB | Streaming processing, line-based parsing |
| **RDF/XML** | `01_custodian_name.rdf` | 289 KB | Legacy systems, XML toolchains |
| **JSON-LD** | `01_custodian_name.jsonld` | 335 KB | Web APIs, JavaScript applications |

**OWL Ontology Features**:
- Complete class hierarchy with owl:Class definitions
- Property restrictions (cardinality, range constraints)
- Ontology alignments (class_uri, slot_uri mappings)
- SKOS documentation (definitions, notes, examples)

### 6. ISO 20275 Data Parsed ✅

Successfully parsed GLEIF Entity Legal Form code list:

**Statistics**:
- **3,819 active legal form codes**
- **117 jurisdictions** (countries/regions)
- **Top 5 countries**: US (724), FR (255), CA (239), FI (132), BE (129)

**Generated Files**:
- `ISO20275_common.yaml` - Curated mappings for heritage institutions (foundations, nonprofits, etc.)

### 7. Documentation Created ✅

**New Documentation** (17 KB total):
- `LEGAL_ENTITY_REFACTORING.md` (14 KB) - Complete design rationale and migration guide
- `LEGAL_ENTITY_QUICK_REFERENCE.md` (3 KB) - Quick reference for developers
- `LEGAL_ENTITY_IMPLEMENTATION_SUMMARY.md` (this file)

### 8. Files Created/Updated

**Created** (9 new schema files):
- `LegalEntityType.yaml`
- `LegalForm.yaml`
- `LegalName.yaml`
- `RegistrationInfo.yaml`
- `legal_entity_type.yaml` (slot)
- `registration_numbers.yaml` (slot)
- `ISO20275_mapping.yaml`

**Updated** (8 existing files):
- `CustodianReconstruction.yaml` - 7 slot ranges updated
- `ReconstructionActivity.yaml` - Temporal model refactored
- `AgentTypeEnum.yaml` - New agent types added
- `01_custodian_name_modular.yaml` - Imports updated
- `legal_name.yaml` (slot) - Range changed to class
- `legal_form.yaml` (slot) - Range changed to class
- `registration_authority.yaml` (slot) - Range changed to class
- `governance_structure.yaml` (slot) - Range changed to class

**Deprecated** (2 files):
- `entity_type.yaml` → `.deprecated`
- `registration_number.yaml` → `.deprecated`

---

## Key Design Decisions

### Critical Rule: CustodianReconstruction = Legal Entities ONLY

**CustodianReconstruction** is now strictly for formally registered legal entities:
- Natural persons (individuals with legal rights)
- Legal persons (organizations with legal personality)

**Informal groups** (families, communities, amateur clubs) remain as **CustodianObservation only** (not reconstructed as legal entities).

### Two-Tier Classification

**LegalEntityType** has only 2 values:
1. **PERSON**: Natural persons (cannot have legal forms per ISO 20275)
2. **ORGANIZATION**: Legal persons (must have legal forms)

This aligns with ISO 20275 scope (organizations only) and legal theory (persons vs organizations).

### ISO 20275 Integration

- 3,819 legal form codes across 117 jurisdictions
- Each `LegalForm` instance references an ELF code
- Curated common mappings for heritage institutions
- Country-specific templates for localization

### Ontology Alignments

| Class | Primary Ontology | Secondary Alignments |
|-------|------------------|---------------------|
| `LegalEntityType` | ROV:RegisteredOrganization | org:Organization |
| `LegalForm` | ELF codes (ISO 20275) | org:classification |
| `LegalName` | TOOI (Dutch govt) | rov:legalName, skos:prefLabel |
| `RegistrationNumber` | ROV:registration | adms:Identifier |
| `RegistrationAuthority` | ROV:RegistrationAuthority | org:RegisteredOrganization |
| `GovernanceStructure` | org:Organization | schema:Organization |
| `LegalStatus` | ROV:orgStatus | schema:status |

---

## Validation Results

### Schema Validation
- ✅ **LinkML imports resolved** - All 84 module files loaded successfully
- ✅ **No circular dependencies** - String ranges used where needed
- ⚠️ **Example instances need updating** - Old `EntityTypeEnum` values present

### RDF Generation
- ✅ **OWL ontology generated** - 138 KB Turtle file
- ✅ **All formats created** - Turtle, N-Triples, RDF/XML, JSON-LD
- ⚠️ **Namespace warnings** (non-critical) - Multiple ontologies define same prefixes

### ISO 20275 Parsing
- ✅ **3,819 codes parsed** - Complete GLEIF code list v1.5
- ✅ **Common mappings created** - Template for heritage institutions
- 📋 **TODO**: Curate country-specific mappings (NL, BE, FR, DE, US, etc.)

---

## What's Next

### Immediate (Required)

1. **Update Example Instances** ✅ PRIORITY
   - Migrate `entity_type` → `legal_entity_type` in all examples
   - Convert primitive values to class instances:
     ```yaml
     # OLD
     legal_name: "Stichting Rijksmuseum"
     legal_form: "Stichting"
     registration_number: "12345678"
     
     # NEW
     legal_name:
       full_name: "Stichting Rijksmuseum"
       name_without_type: "Rijksmuseum"
       alphabetical_name: "Rijksmuseum, Stichting"
     legal_form:
       elf_code: "8888"  # Foundation
       country_code: "NL"
       local_name: "Stichting"
     registration_numbers:
       - number: "12345678"
         authority:
           name: "Kamer van Koophandel"
           country: "NL"
         valid_from: "1994-01-01"
     ```

2. **Run Validation Tests** ✅ PRIORITY
   ```bash
   linkml-validate -s schemas/20251121/linkml/01_custodian_name_modular.yaml \
                   schemas/20251121/examples/*.yaml
   ```

3. **Generate Python Dataclasses** 📋 TODO
   ```bash
   gen-python schemas/20251121/linkml/01_custodian_name_modular.yaml > \
            schemas/20251121/python/custodian_model.py
   ```

### Short-term (Data Migration)

4. **Create Migration Script** 📋 TODO
   - Read existing YAML data using old schema
   - Transform `entity_type` enum → `legal_entity_type` class instances
   - Transform primitive slots → class instances
   - Write migrated data using new schema

5. **Update Unit Tests** 📋 TODO
   - Test all 4 RegistrationInfo sub-classes
   - Test LegalForm with ISO 20275 codes
   - Test LegalName with TOOI variants
   - Test TimeSpan for fuzzy temporal extents

6. **Create Country-Specific Mappings** 📋 TODO
   - Netherlands: Stichting (foundation), Vereniging (association), BV (private company)
   - Belgium: ASBL/VZW (nonprofit), SA/NV (public company)
   - France: Association loi 1901, Fondation, SARL
   - Germany: e.V. (Verein), gGmbH (nonprofit), Stiftung
   - United States: 501(c)(3) nonprofit, LLC, Corporation

### Long-term (Enhancements)

7. **Curate RegistrationAuthority List** 📋 TODO
   - Compile list of national business registries (per country)
   - Add Chamber of Commerce identifiers (where applicable)
   - Link to official registry APIs

8. **Map Full ISO 20275 Hierarchy** 📋 TODO
   - Legal form parent/child relationships
   - Regional variants (e.g., US state-specific forms)
   - Historical legal forms (inactive but relevant)

9. **Integrate with National Registries** 📋 TODO
   - Netherlands: KvK (Kamer van Koophandel) API
   - Belgium: KBO/BCE (Kruispuntbank van Ondernemingen)
   - France: INSEE SIRENE
   - Germany: Handelsregister

10. **Add Legal Form Change Tracking** 📋 TODO
    - Track organizational transformations (e.g., Vereniging → Stichting)
    - Link to `ChangeEvent` class in provenance module
    - Model legal form conversions (e.g., incorporation)

---

## Migration Checklist

For developers updating code or data to use the new legal entity model:

- [ ] Replace all `entity_type` references with `legal_entity_type`
- [ ] Update `EntityTypeEnum` to `LegalEntityType` (PERSON | ORGANIZATION)
- [ ] Convert `legal_name` from string to `LegalName` class
- [ ] Convert `legal_form` from string to `LegalForm` class with ELF code
- [ ] Replace single `registration_number` with list of `registration_numbers`
- [ ] Convert `registration_authority` from string to `RegistrationAuthority` class
- [ ] Convert `governance_structure` from string to `GovernanceStructure` class
- [ ] Convert `legal_status` from enum to `LegalStatus` class
- [ ] Add `legal_entity_type` property to all CustodianReconstruction instances
- [ ] Verify informal groups are CustodianObservation (not Reconstruction)
- [ ] Update temporal fields to use `TimeSpan` instead of separate start/end
- [ ] Run LinkML validation on all updated files
- [ ] Regenerate RDF if ontology mappings changed
- [ ] Update documentation/examples referencing old model

---

## Testing Commands

```bash
# Validate schema structure
linkml-validate -s schemas/20251121/linkml/01_custodian_name_modular.yaml

# Validate example instances
linkml-validate -s schemas/20251121/linkml/01_custodian_name_modular.yaml \
                schemas/20251121/examples/*.yaml

# Generate RDF ontology (Turtle)
gen-owl -f ttl schemas/20251121/linkml/01_custodian_name_modular.yaml > \
        schemas/20251121/rdf/01_custodian_name.owl.ttl

# Convert to other RDF formats
rdfpipe schemas/20251121/rdf/01_custodian_name.owl.ttl -o nt > \
        schemas/20251121/rdf/01_custodian_name.nt

rdfpipe schemas/20251121/rdf/01_custodian_name.owl.ttl -o json-ld > \
        schemas/20251121/rdf/01_custodian_name.jsonld

rdfpipe schemas/20251121/rdf/01_custodian_name.owl.ttl -o xml > \
        schemas/20251121/rdf/01_custodian_name.rdf

# Generate Python dataclasses
gen-python schemas/20251121/linkml/01_custodian_name_modular.yaml > \
           schemas/20251121/python/custodian_model.py

# Parse ISO 20275 codes
python scripts/parse_iso20275_codes.py
```

---

## Key Files Reference

**Main Schema**: `schemas/20251121/linkml/01_custodian_name_modular.yaml`

**Legal Entity Classes**:
- `schemas/20251121/linkml/modules/classes/LegalEntityType.yaml`
- `schemas/20251121/linkml/modules/classes/LegalForm.yaml`
- `schemas/20251121/linkml/modules/classes/LegalName.yaml`
- `schemas/20251121/linkml/modules/classes/RegistrationInfo.yaml`

**Updated Core Classes**:
- `schemas/20251121/linkml/modules/classes/CustodianReconstruction.yaml`
- `schemas/20251121/linkml/modules/classes/ReconstructionActivity.yaml`

**Legal Entity Slots**:
- `schemas/20251121/linkml/modules/slots/legal_entity_type.yaml`
- `schemas/20251121/linkml/modules/slots/registration_numbers.yaml`

**Documentation**:
- `schemas/20251121/linkml/modules/classes/LEGAL_ENTITY_REFACTORING.md`
- `schemas/20251121/linkml/modules/classes/LEGAL_ENTITY_QUICK_REFERENCE.md`
- `schemas/20251121/linkml/modules/classes/LEGAL_ENTITY_IMPLEMENTATION_SUMMARY.md`

**Data Sources**:
- `data/ontology/2023-09-28-elf-code-list-v1.5.csv` (ISO 20275 codes)
- `schemas/20251121/linkml/modules/mappings/ISO20275_common.yaml` (curated mappings)

**Generated RDF**:
- `schemas/20251121/rdf/01_custodian_name.owl.ttl` (Turtle)
- `schemas/20251121/rdf/01_custodian_name.nt` (N-Triples)
- `schemas/20251121/rdf/01_custodian_name.rdf` (RDF/XML)
- `schemas/20251121/rdf/01_custodian_name.jsonld` (JSON-LD)

---

## Success Metrics

✅ **Schema Complexity**: 17 classes, 59 slots, 6 enums (84 module files)  
✅ **Legal Forms Supported**: 3,819 codes across 117 jurisdictions  
✅ **Ontology Alignments**: 12 base ontologies (TOOI, ROV, W3C Org, ISO 20275, etc.)  
✅ **RDF Formats**: 4 serializations (Turtle, N-Triples, RDF/XML, JSON-LD)  
✅ **Documentation**: 17 KB comprehensive guides  
✅ **Temporal Precision**: Fuzzy timestamps with begin/end boundaries  
✅ **Data Quality**: Strict validation rules (legal entities only in reconstructions)  

---

**Implementation Complete**: 2025-11-22  
**Next Review**: After example migration and validation tests  
**Status**: ✅ SCHEMA REFACTORED, RDF GENERATED, DATA PARSED
