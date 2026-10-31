# Heritage Custodian Schemata

This directory contains non-code schema, ontology, mapping, and transform artefacts.

## Canonical Upstream

The canonical upstream is the RootReal Forgejo monorepo maintained by TextPast:

- Repo: `https://git.rootreal.io/textpast/rootreal`
- Schemata subtree: `crates/model/symbolic/schemata/`

Only the files below are exported to the public GitHub projection:

- `README.md`
- `LICENSE.md`
- `meta/`
- `parsimony/`

## Published Projection

The public GitHub repository below is a synchronized projection of the two
schema families below:

- `https://github.com/netwerk-digitaal-erfgoed/heritage_custodian`

- `meta/`
- `parsimony/`

This GitHub repository exists to make these schemata easier to find, share,
and review with external partners. Accepted changes are integrated in Forgejo
first and then synchronized back to GitHub.

## CI And Review

GitHub pull requests are welcome on the public projection repository, but CI for
those contributions runs in RootReal's external Woodpecker infrastructure rather
than in GitHub Actions. GitHub receives the resulting external status updates;
GitHub Actions is not part of the intended workflow for this projection repo.

## Licensing

The published projection and the exported schemata in `meta/` and `parsimony/`
are licensed under EUPL v1.2.

- Repository license file: `LICENSE.md`
- Official Dutch text: `https://eupl.eu/1.2/nl/`
