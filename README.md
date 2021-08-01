# qrwcell - quick read-write cell

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Rust][ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/qrwcell.svg
[crates-url]: https://crates.io/crates/qrwcell
[docs-badge]: https://docs.rs/qrwcell/badge.svg
[docs-url]: https://docs.rs/qrwcell
[ci-badge]: https://github.com/cezarmathe/qrwcell/workflows/Rust/badge.svg
[ci-url]: https://github.com/cezarmathe/qrwcell/actions/workflows/rust.yml

Read-write cell that aims to reduce the amount of blocking compared to a single
read-write lock.

The cell has two slots - one for reading and one for writing. Writing alternates
the slot that is currently served to readers, thereby minimising blocking on a
reader-writer lock.

Please be aware that if a cell is not created with a value or updated at
least once attempting to get the inner value will loop forever!
