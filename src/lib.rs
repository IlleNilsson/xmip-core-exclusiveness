#![forbid(unsafe_code)]

//! Retired. ADR-0024.
//!
//! This module held a lease inside Xmip — `ExclusiveScope`, `ExclusiveLease`,
//! `Exclusiveness` — and could never say where a cluster-wide one would live.
//! ADR-0017 clause 8 put leases in `xmip-core-persist`, which is per node, so
//! `ExclusiveScope::Cluster` was declarable and unservable. Every way of fixing
//! that meant either a shared write path Xmip must not have, a
//! distributed-consensus implementation, or depending on somebody else's
//! cluster to answer a question about Xmip's own.
//!
//! The fact already had an owner.
//!
//! **`ResourceClaim` in `xmip-core-transport` replaces all of it.** The
//! endpoint is one thing however many nodes are asking, so a claim taken there
//! is cluster-wide without a lease, a store, or anything for Xmip to keep
//! consistent across nodes. A share-mode open, a blob lease, `If-None-Match: *`
//! on S3, a generation precondition, `SELECT … FOR UPDATE SKIP LOCKED` — each
//! is atomic and none is Xmip's to operate.
//!
//! It also answers what a lease could not: whether something **outside** Xmip
//! holds the artefact. A file another process has open includes a producer
//! still writing it.
//!
//! Nothing here is exported. The crate stays so the repository resolves for
//! anything still pinning it, and so ADR-0017 has somewhere to point.
