# Contributing

This repository is a public projection. The canonical source lives in the
RootReal Forgejo monorepo, https://git.rootreal.io/textpast/rootreal,
under `crates/model/symbolic/schemata/`; what you see on GitHub is
published from there. Contributions are welcome here all the same, and the
flow below carries them into the canonical repo.

## Opening a pull request

Open the request against `main`, and keep the change inside the published
set: `meta/`, `parsimony/`, or top-level collaboration files such as
`README.md` and this one. A bot forwards your request to a linked pull
request in Forgejo and leaves a comment on yours with the link. Review and
CI happen on that side, on RootReal's Tekton cluster rather than on
GitHub Actions. TextPast and NDE maintainers review the change, and the
merge happens in Forgejo; the next publish then brings it back to
GitHub.

Please do not push straight to `main`; the sync automation owns that
branch, and a direct push would be overwritten on the next publish.

## What makes a change easy to review

The schema files import each other by relative path, so keep imports
working when you split or move a file. Names are load-bearing in the
toolchain built around these schemata: treat a rename as a breaking
change and say in the request what moved, and why. Beyond the mechanics,
write a few sentences on what the ontology is meant to express. When you
change an existing term, note the compatibility impact; ranges,
permissible values and slot owners all have downstream consumers.
