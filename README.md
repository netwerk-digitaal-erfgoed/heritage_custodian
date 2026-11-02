# Heritage Custodian Schemata

Archives, museums and other heritage institutions primarily publish linked data
about their collections. Other than basic information on provenance, the data
about heritage custodianship often remains obscure. This repository contains a
model for capturing the work involved in looking after collections, 
including acquisition, accessioning, preservation and access. It also expresses
the ever-changing shape of heritage organisations, their teams and their infrastructure.

The model is written in LinkML. These LinkML schemata can be mapped to
several kinds of databases from the same source files. 

## What is published here

- meta/ holds the complete model, including classes, slots,
enumerations, types, profiles, mappings and provenance records. Class
definitions link to existing standards like CIDOC CRM, Records in
Contexts (RiC-O) and Wikidata. The mappings directory also provides
crosswalks for external registers such as ISO 20275 legal forms and
ISIL institution codes.
- parsimony/ contains a smaller core model built around
CustodialEntity, Holding, Agent, Activity and a few supporting
terms, along with documentation showing how each one maps to the full
model.
- LICENSE.md contains the EUPL v1.2 licence text.

If you are new to the project, parsimony/ is the easiest place to get
a feel for the overall structure before digging into the full
specification in meta/.

## Upstream

Active development happens in the RootReal monorepo on Forgejo
(https://git.rootreal.io/textpast/rootreal) under
crates/model/symbolic/schemata/, rather than here on GitHub. Whenever
that directory changes on main, a Tekton pipeline copies meta/,
parsimony/ and the top-level files over to this repository. This sync
only runs one way, so if the two repositories ever disagree, the Forgejo
monorepo is the source of truth.

## CI and review

You can still open pull requests here on GitHub. A bot forwards each PR
to the Forgejo monorepo and leaves a comment on the GitHub thread with a
link to the upstream request. Code review, merging and CI all run on
Forgejo through RootReal's Tekton cluster (GitHub Actions is not used).
Once a change is merged upstream, it shows up here on the next publish
run.

See CONTRIBUTING.md for details on how to scope changes, how file
names and imports work and what to include in a PR description.

## Licence

The schemata and this repository are licensed under the EUPL v1.2. The
full text is in LICENSE.md, and the official versions in other
languages (including Dutch) are available at https://eupl.eu/1.2/nl/.
