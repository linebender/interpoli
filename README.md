<div align="center">

# Interpoli

**It's the journey, not the destination.**

<!-- TODO: Fix [![Linebender Zulip, #kurbo stream](https://img.shields.io/badge/Linebender-%23kurbo-red?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/260979-kurbo) -->
[![dependency status](https://deps.rs/repo/github/linebender/interpoli/status.svg)](https://deps.rs/repo/github/linebender/interpoli)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![Build status](https://github.com/linebender/interpoli/workflows/CI/badge.svg)](https://github.com/linebender/interpoli/actions)
[![Crates.io](https://img.shields.io/crates/v/interpoli.svg)](https://crates.io/crates/interpoli)
[![Docs](https://docs.rs/interpoli/badge.svg)](https://docs.rs/interpoli)

</div>

The Interpoli library provides functionality for animating values.

The name "Interpoli" is Esperanto for "interpolate" which encompasses some of the core functionality of this library.

## Minimum supported Rust Version (MSRV)

This version of Interpoli has been verified to compile with **Rust 1.75** and later.

Future versions of Interpoli might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Interpoli's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```
</details>

## Community

<!-- TODO: Fix [![Linebender Zulip, #kurbo stream](https://img.shields.io/badge/Linebender-%23kurbo-red?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/260979-kurbo) -->

Discussion of Interpoli development happens in the Linebender Zulip at <https://xi.zulipchat.com/>, but there is not yet an established channel.
All public content can be read without logging in

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AUTHORS]: ./AUTHORS
