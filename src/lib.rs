//! A library to parse network packets from a certain turn based anime game!

#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "resource")]
pub mod resource;

#[cfg(all(not(feature = "network"), feature = "proto-limited"))]
compile_error!("feature `proto-limited` has no effect without feature `network`");

#[cfg(all(not(feature = "network"), feature = "proto-rqa"))]
compile_error!("feature `proto-rqa` has no effect without feature `network`");

#[cfg(all(feature = "proto-rqa", feature = "proto-limited"))]
compile_error!("feature `proto-rqa` cannot be used together with feature `proto-limited`");