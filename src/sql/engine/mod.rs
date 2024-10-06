//! # Engine
//!
//! The SQL engine provides SQL data storage and access, as well as session and
//! transaction management. The [`Local`] engine provides node-local on-disk
//! storage, while the [`Raft`] engine submits commands through Raft consensus
//! before dispatching to the [`Local`] engine on each node.

#![allow(clippy::module_inception)]

mod engine;
mod local;
mod raft;
mod session;
