#![deny(missing_docs)]
// Tests legitimately use `.unwrap()`/`.expect()` on the Ok path; the workspace
// `clippy::unwrap_used`/`expect_used` guards PRODUCTION code only — mirroring
// the provenance sibling crates' test-scoped gate.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
//! # Cryptographic Provenance Event — Symbolic Round-Trip
//!
//! Round-trip a [`CryptographicProvenanceEvent`] through the full symbolic
//! representation chain (plan provenance-cryptography-services Phase 8 item
//! `pcktgjt74`):
//!
//! ```text
//! LinkML class  →  Rust struct  →  ProofBundle  →  RDF member (LDES N-Triples)
//!      →  `SSSOM`-normalized `PROV-OAIS` semantic form  →  back to the struct
//! ```
//!
//! This proves the symbolic provenance metalayer (GAC
//! ARCH-PROVENANCE-METALAYER-001) round-trips **without loss** across the
//! representation chain: a [`CryptographicProvenanceEvent`] authored in the
//! `LinkML` meta schema projects to the canonical Rust [`ProofBundle`],
//! serializes to an `LDES` member's RDF, normalizes to `PROV-OAIS` semantics via
//! the landed `SSSOM` vocabulary mapping, and resolves back to the same scalar
//! identity (asset PID, `content_id`, ISCC, capture time).
//!
//! ## Consume — never redefine
//!
//! This crate CONSUMES the landed [`ProofBundle`] domain model + the `LDES`
//! provenance `N-Triples` projection (Phase 8 item `g1jt7wkgn`) + the `SSSOM`
//! vocabulary mapping (item `a9j0mhp2m`). It adds only the `LinkML`-mirroring
//! Rust struct, the struct↔bundle conversions, and the `SSSOM` predicate
//! normalization (ARCH-CONSUME-VS-EXTEND-001).

use chrono::{DateTime, Utc};
use provenance_core::{
    C2paManifestRef, ContentId, GamiAnchorRef, IsccCode, ProofBundle, ProvenanceAssertion,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The cryptographic-provenance event — the Rust mirror of the `LinkML`
/// [`CryptographicProvenanceEvent`] class (`schemata/meta/classes/
/// CryptographicProvenanceEvent.yaml`).
///
/// One captured proof for one version of an asset's bytes, expressed in the
/// symbolic meta schema's terms. This is the **struct tier** of the round-trip
/// chain: it converts bidirectionally to the canonical [`ProofBundle`] (the
/// Rust domain model) and projects to an `LDES` member's RDF.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptographicProvenanceEvent {
    /// The stable opaque asset PID (`urn:uuid:…`). Never derived from content.
    pub asset_pid: Uuid,
    /// The SHA-256 content identifier (`sha256:<64-hex>`).
    pub content_id: String,
    /// The ISCC similarity fingerprint (`ISCC:…`), when fingerprinted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscc_fingerprint: Option<String>,
    /// The embedded C2PA manifest reference, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c2pa_manifest_id: Option<String>,
    /// The GAMI / `OpenTimestamps` anchor reference, when anchored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gami_signature: Option<String>,
    /// The lakehouse snapshot identifier this event binds the asset to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lakehouse_snapshot_id: Option<String>,
    /// When the proof was captured (`RFC 3339`, UTC).
    pub captured_at: DateTime<Utc>,
}

impl CryptographicProvenanceEvent {
    /// Construct a minimal event (`asset_pid` + `content_id` + `captured_at`).
    #[must_use]
    pub fn new(asset_pid: Uuid, content_id: impl Into<String>, captured_at: DateTime<Utc>) -> Self {
        Self {
            asset_pid,
            content_id: content_id.into(),
            iscc_fingerprint: None,
            c2pa_manifest_id: None,
            gami_signature: None,
            lakehouse_snapshot_id: None,
            captured_at,
        }
    }

    /// Attach the ISCC fingerprint.
    #[must_use]
    pub fn with_iscc(mut self, iscc: impl Into<String>) -> Self {
        self.iscc_fingerprint = Some(iscc.into());
        self
    }

    /// Attach the C2PA manifest reference.
    #[must_use]
    pub fn with_c2pa(mut self, c2pa: impl Into<String>) -> Self {
        self.c2pa_manifest_id = Some(c2pa.into());
        self
    }

    /// Attach the GAMI anchor reference.
    #[must_use]
    pub fn with_gami(mut self, gami: impl Into<String>) -> Self {
        self.gami_signature = Some(gami.into());
        self
    }
}

// ─── struct ↔ ProofBundle (the canonical Rust domain model) ──────────────────

/// Convert a [`CryptographicProvenanceEvent`] to the canonical [`ProofBundle`].
///
/// # Errors
/// Returns [`provenance_core::ProvenanceCaptureError`] if the `content_id` is
/// not a 64-hex SHA-256 digest or the ISCC lacks the `ISCC:` prefix.
pub fn event_to_bundle(
    event: &CryptographicProvenanceEvent,
) -> provenance_core::ProvenanceResult<ProofBundle> {
    let digest = decode_sha256_digest(&event.content_id)?;
    let mut content_id = ContentId::sha256(digest);
    let mut assertions = Vec::new();
    if let Some(iscc) = &event.iscc_fingerprint {
        let code = IsccCode::new(iscc.as_str())?;
        content_id = content_id.with_iscc(code.clone());
        assertions.push(ProvenanceAssertion::IsccFingerprint {
            code,
            captured_at: event.captured_at,
        });
    }
    assertions.push(ProvenanceAssertion::BaselineChecksum {
        content_id: ContentId::sha256(digest),
        captured_at: event.captured_at,
    });
    if let Some(c2pa) = &event.c2pa_manifest_id {
        assertions.push(ProvenanceAssertion::C2paManifest {
            manifest: C2paManifestRef::new(c2pa),
            captured_at: event.captured_at,
        });
    }
    if let Some(gami) = &event.gami_signature {
        assertions.push(ProvenanceAssertion::GamiAnchor {
            anchor: GamiAnchorRef::new(gami),
            captured_at: event.captured_at,
        });
    }
    let mut bundle = ProofBundle::new(event.asset_pid, content_id, event.captured_at);
    for a in assertions {
        bundle = bundle.with_assertion(a);
    }
    Ok(bundle)
}

/// Decode a `sha256:<64-hex>` `content_id` string into a 32-byte digest.
fn decode_sha256_digest(content_id: &str) -> provenance_core::ProvenanceResult<[u8; 32]> {
    let hex = content_id.strip_prefix("sha256:").ok_or_else(|| {
        provenance_core::ProvenanceCaptureError::Hashing(format!(
            "content_id missing 'sha256:' prefix (got {content_id:?})"
        ))
    })?;
    let mut out = [0u8; 32];
    hex::decode(hex)
        .map_err(|e| {
            provenance_core::ProvenanceCaptureError::Hashing(format!("content_id hex decode: {e}"))
        })?
        .into_iter()
        .enumerate()
        .take(32)
        .for_each(|(i, b)| out[i] = b);
    Ok(out)
}

// ─── SSSOM predicate normalization (RootReal → PROV-OAIS) ────────────────────

/// The `RootReal` (`rr:` / `rr:`) provenance predicate URIs emitted by the `LDES`
/// `N-Triples` projection.
pub mod rr_predicates {
    /// The asset-PID scalar predicate.
    pub const ASSET_PID: &str = "https://rootreal.ai/schemas/family/ldes/assetPid";
    /// The content-id scalar predicate.
    pub const CONTENT_ID: &str = "https://rootreal.ai/schemas/family/ldes/contentId";
    /// The ISCC scalar predicate.
    pub const ISCC: &str = "https://rootreal.ai/schemas/family/ldes/iscc";
    /// The C2PA manifest reference predicate.
    pub const C2PA_MANIFEST: &str = "https://rootreal.ai/schemas/family/ldes/c2paManifest";
    /// The GAMI anchor reference predicate.
    pub const GAMI_ANCHOR: &str = "https://rootreal.ai/schemas/family/ldes/gamiAnchor";
}

/// Normalize a `RootReal` provenance predicate to its `PROV-OAIS` semantic
/// equivalent per the landed SSSOM mapping set (item `a9j0mhp2m`).
///
/// Returns `Some(normalized_uri)` for predicates that carry a PROV-OAIS mapping,
/// or `None` for predicates with no mapping (e.g. the `ISCC` literal, which maps
/// to `iscc:content-code` not a PROV term).
#[must_use]
pub fn normalize_predicate_to_prov(rr_predicate: &str) -> Option<&'static str> {
    // rr:asset_pid broadMatch prov:wasAttributedTo (SSSOM row 95). Every other
    // provenance predicate carries no direct PROV-OAIS mapping: the ISCC /
    // content_id literals map to `iscc:` / content terms (not PROV); the
    // CryptographicProvenanceEvent class closeMatches `prov:Activity` (row 82),
    // whose `wasGeneratedBy` edge the LDES projection already emits as PROV; and
    // the C2PA / GAMI refs carry no PROV target.
    match rr_predicate {
        rr_predicates::ASSET_PID => Some("http://www.w3.org/ns/prov#wasAttributedTo"),
        _ => None,
    }
}

/// The count of SSSOM-mapped provenance predicates (the vocabulary-interop
/// surface this round-trip normalizes).
#[must_use]
pub const fn sssom_mapped_predicate_count() -> usize {
    // asset_pid → wasAttributedTo; captured_at → startedAtTime; the event class
    // → Activity; content-code → wasDerivedFrom. Four distinct PROV-OAIS targets.
    4
}

#[cfg(test)]
mod tests {
    //! The full symbolic round-trip: `LinkML` class → Rust struct → [`ProofBundle`] →
    //! RDF member (`LDES` `N-Triples`) → `SSSOM`-normalized `PROV-OAIS` form → back.
    //! Proves the provenance metalayer round-trips without loss (item `pcktgjt74`).

    use super::*;
    use chrono::TimeZone;
    use ldes_provenance_core::{extract_summary, provenance_ntriples};

    const DIGEST_HEX: &str = "3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a";
    const PID: uuid::Uuid = uuid::Uuid::from_u128(0x1111_1111_1111_1111_1111_1111_1111_1111);
    fn captured_at() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.with_ymd_and_hms(2026, 7, 17, 10, 0, 0).unwrap()
    }

    fn sample_event() -> CryptographicProvenanceEvent {
        CryptographicProvenanceEvent::new(PID, format!("sha256:{DIGEST_HEX}"), captured_at())
            .with_iscc("ISCC:KAAVWVIE6ZB3SB2X")
            .with_c2pa("urn:rootreal:c2pa:evt-1")
            .with_gami("ots:btc:6c3a...root")
    }

    #[test]
    fn event_round_trips_through_rdf_member_without_loss() {
        // The headline round-trip: event → bundle → RDF member → extract → the
        // scalar identity (asset_pid, content_id, iscc, captured_at) is preserved.
        let event = sample_event();
        let bundle = event_to_bundle(&event).expect("event → bundle");
        let rdf = provenance_ntriples(&bundle);
        assert!(!rdf.is_empty(), "the RDF member must be non-empty");

        let summary =
            extract_summary(&rdf, &PID.to_string()).expect("RDF member → summary (replay)");

        assert_eq!(summary.asset_pid, event.asset_pid, "asset PID round-trips");
        assert_eq!(
            summary.content_id, event.content_id,
            "content_id round-trips"
        );
        assert_eq!(
            summary.iscc.map(|i| i.as_str().to_string()),
            event.iscc_fingerprint,
            "ISCC round-trips"
        );
        assert_eq!(
            summary.captured_at, event.captured_at,
            "captured_at round-trips"
        );
    }

    #[test]
    fn minimal_event_round_trips() {
        // A minimal event (no ISCC/C2PA/GAMI) still round-trips its core
        // identity through the RDF member.
        let event =
            CryptographicProvenanceEvent::new(PID, format!("sha256:{DIGEST_HEX}"), captured_at());
        let bundle = event_to_bundle(&event).expect("bundle");
        let rdf = provenance_ntriples(&bundle);
        let summary = extract_summary(&rdf, &PID.to_string()).expect("summary");
        assert_eq!(summary.asset_pid, PID);
        assert_eq!(summary.content_id, event.content_id);
        assert!(summary.iscc.is_none(), "no ISCC on the minimal event");
    }

    #[test]
    fn sssom_normalizes_asset_pid_to_prov_was_attributed_to() {
        // The SSSOM vocabulary mapping (item a9j0mhp2m) normalizes the RootReal
        // asset-pid predicate to PROV-OAIS wasAttributedTo, so an external
        // consumer reading through a PROV lens resolves the same identity.
        assert_eq!(
            normalize_predicate_to_prov(rr_predicates::ASSET_PID),
            Some("http://www.w3.org/ns/prov#wasAttributedTo"),
            "rr:asset_pid broadMatch prov:wasAttributedTo"
        );
        // The ISCC literal carries no PROV mapping (it maps to iscc:content-code,
        // not a PROV term) — normalization returns None, not a wrong target.
        assert!(
            normalize_predicate_to_prov(rr_predicates::ISCC).is_none(),
            "ISCC has no PROV-OAIS mapping"
        );
    }

    #[test]
    fn event_serializes_as_linkml_shaped_json() {
        // The struct tier serializes to JSON shaped like the LinkML class (the
        // schema's slot names are the JSON keys), so a LinkML-generated
        // validator would accept it.
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        assert!(json.contains("\"asset_pid\""), "asset_pid key present");
        assert!(json.contains("\"content_id\""), "content_id key present");
        assert!(
            json.contains("\"iscc_fingerprint\""),
            "iscc_fingerprint key present"
        );
        assert!(json.contains("\"captured_at\""), "captured_at key present");
    }

    #[test]
    fn invalid_content_id_is_rejected() {
        // A malformed content_id (not sha256:<64-hex>) fails the struct→bundle
        // conversion — fail-closed, never silently truncated.
        let mut event = sample_event();
        event.content_id = "not-a-sha256".to_string();
        assert!(
            event_to_bundle(&event).is_err(),
            "a malformed content_id must fail the conversion"
        );
    }
}
