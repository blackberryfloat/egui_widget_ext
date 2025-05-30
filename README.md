[![Last Release Status](https://img.shields.io/github/actions/workflow/status/blackberryfloat/egui_widget_ext/release.yml?label=last%20release&style=flat-square)](https://github.com/blackberryfloat/egui_widget_ext/actions/workflows/release.yml)
[![CI Status](https://img.shields.io/github/actions/workflow/status/blackberryfloat/egui_widget_ext/main_ci.yml?label=main%20ci&style=flat-square)](https://github.com/blackberryfloat/egui_widget_ext/actions/workflows/main_ci.yml)
[![Issues](https://img.shields.io/github/issues/blackberryfloat/egui_widget_ext?style=flat-square)](https://github.com/blackberryfloat/egui_widget_ext/issues)
[![docs.rs](https://img.shields.io/docsrs/egui_widget_ext?style=flat-square)](https://docs.rs/egui_widget_ext)
[![Sponsor](https://img.shields.io/github/sponsors/blackberryfloat?style=social&label=Sponsor&logo=githubsponsors)](https://github.com/sponsors/blackberryfloat)

# Egui Widget Extension Pack

This repo is intended to expand on egui's widgets to include a host of functionality we expect to be standard in web development.

## Features

There is basically a feature flag for each widget. The idea behind this crate is to allow you to be surgical as possible about what UI functionality you bring in. 

- `all` - alias for including all widgets supported by this library
- `toggle_switch` - simple toggle switch widget
- `alert` - simple widget for displaying alerts

## License

Licensed under either of the following:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Public contribution is welcome and encouraged. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Documentation

All documentation that is associated with a specific block of code, should be documented inline per the rust documentation expectations. This documentation should be available in the auto geenrated rust docs. Documentation of more complex technical concepts surrounding this crate as a whole should be put in markdown files in the docs folder. Everything else such as crate requirements, installation instructions, etc should be documented in this README.md.

### Code Standards

- all code should be formatted per cargo's default fmt command
- code should target 80% automated code coverage
- widgets should follow a style pattern that mimics widgets produced by the core egui team

### Release Process

Releases are managed through both git tags and branches. Branches are used for convenience and tags actually trigger the relevant release actions. Whenever there is a new major or minor release a branch must be created at the relevant hash in the format v\<major\>.\<minor\> (ie v1.33). Branches with such a format are protected by a ruleset and can only be modified by admins. All release tags must point to hashes on said branch. There is also a ruleset protecting all git tags matching the semantic versioning format v*.*.\* so that only admins can add such tags.

#### Major or Minor Release

In summary, you must be an admin and complete the following steps:

- pick a hash
- confirm all automated tests have passed
- create a branch at the relevant hash in the format v\<major\>.\<minor\> (ie v1.33).
- if necessary perform any last minuted changes
- create a git tag pointing to the tip of that branch in the format v\<major\>.\<minor\>.0 (ie v1.33.0).

The git tag will kick off an automated process that deploys the crate to crates.io after validating crate version matches the tag version and all automated tests pass.
