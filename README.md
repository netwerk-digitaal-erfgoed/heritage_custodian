# Heritage Custodian Schemata

Data custodians in the heritage field, archives and museums among them,
manage, maintain and offer linked data about their collections. Before
records from different custodian systems can be mapped and processed
together, they need a shared model. This repository publishes that model
as LinkML schemata: the vocabulary that describes held material and the
custody work around it, from acquisition and accession to access and
preservation.

Because the model is LinkML, it converts to more than one database
family. The standard LinkML generators produce RDF, OWL and SHACL for
triplestores. The same
source compiles to property-graph models for multigraph databases, and to
meta schemata for a hypergraph database such as TypeDB, where a class
becomes an entity type and a slot becomes a relation or attribute type.

## What is published here

- `meta/`, the full model: classes, slots, enumerations, types, profiles,
  mappings and provenance records. Class definitions carry references to
  established standards in the field, among them CIDOC CRM, Records in
  Contexts (RiC-O) and Wikidata, and the mappings directory holds
  crosswalks for source registers such as ISO 20275 legal forms and ISIL
  institution codes.
- `parsimony/`, a deliberately small core built with a parsimony method:
  CustodialEntity, Holding, Agent, Activity and a short tail of
  supporting terms, each documented with the way it maps into the full
  model.
- `LICENSE.md`, the EUPL v1.2 licence text.

Start in `parsimony/` for the shape of the model; reach for `meta/` when
you need the detail.

## Upstream

Development does not happen on GitHub. The canonical source is the
RootReal monorepo, https://git.rootreal.io/textpast/rootreal, under
`crates/model/symbolic/schemata/`. A Tekton pipeline publishes `meta/`,
`parsimony/` and the top-level files to this repository whenever the
schemata tree changes on the canonical `main`. The projection is one-way;
when the two copies disagree, the Forgejo side is the one that counts.

## CI and review

Pull requests are welcome here. A bot forwards each one to a linked pull
request in the Forgejo monorepo and comments on the GitHub request with
the link. Review and merge happen on the Forgejo side, and
so does CI: the pipelines run on RootReal's Tekton cluster at
https://ci.rootreal.io, not on GitHub Actions, which plays no part in
this repository's workflow. A merged change comes back to GitHub on the
next publish.

CONTRIBUTING.md spells out the practical rules: what belongs in a change,
how imports and file names are treated, and what a useful description
contains.

## Licence

The published schemata and this projection are licensed under the EUPL,
version 1.2. The licence file is `LICENSE.md`; the official texts,
including the Dutch one, live at https://eupl.eu/1.2/nl/.
