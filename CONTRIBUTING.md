# Contributing

## Canonical Upstream

The canonical upstream repository is:

- `https://git.rootreal.io/textpast/rootreal`

This GitHub repository is a public collaboration surface for the exported
schemata, not the authoritative source of record.

The preferred external collaboration surface is:

- `https://github.com/netwerk-digitaal-erfgoed/heritage_custodian`

An additional synchronized GitHub projection may exist under the maintainer
profile for operational bootstrap or backup visibility, but Forgejo remains the
only authoritative upstream.

## What To Change Here

Changes in this repository should stay within:

- `meta/`
- `parsimony/`
- top-level collaboration files such as `README.md` and `.github/`

## Review And Merge Flow

1. Open a GitHub pull request.
2. The forwarding automation may create or update a linked Forgejo pull request.
3. TextPast or NDE maintainers review the proposed change in the canonical repo.
4. The authoritative merge happens in Forgejo.
5. The merged result is synchronized back to GitHub.

## What To Expect

- GitHub pull requests may be linked to a Forgejo review before final merge.
- GitHub `main` is maintained by synchronization from Forgejo.
- Please avoid direct pushes to `main` unless you are operating the sync flow.

## Practical Guidance

- Preserve existing relative imports inside `meta/` and `parsimony/`.
- Keep schema file names stable unless a coordinated rename is required.
- Explain ontology intent and compatibility impact clearly in the pull request.
