# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]
## [1.0.9] - 2024-12-18

- bump deps

## [1.0.8] - 2024-12-12

- bump deps

## [1.0.7] - 2024-10-20

- bump rom_cache: now, more immutable ref can be held at the same time

## [1.0.6] - 2024-09-17

- doc

## [1.0.5] - 2024-08-20

- doc

## [1.0.4] - 2024-08-15

- doc

## [1.0.3] - 2024-08-13

- doc

## [1.0.2] - 2024-08-13

- doc

## [1.0.1] - 2024-08-13

- update keyring to 3.0, removing linux only features

## [0.5.1] - 2024-08-02

- update doc

## [0.5.0] - 2024-08-02

- use `rom_cache` crate for config caching

## [0.5.0-alpha3] - 2024-07-26

- dropped this version
- simplify the name of `ConfigRef` and `ConfigMut` to `CfgRef` and `CfgMut`

## [0.5.0-alpha2] - 2024-07-25

- make Config Send and Sync
- make the code suitable for testing by loom
- fix: write while reading
- fix: limit the number of `ConfigRef` to 32

## [0.5.0-alpha1] - 2024-07-24

- remove all locks
- totally like CPU cache now
- save to the source when `Config` cache dropped instead of `ConfigMut`

## [0.4.3] - 2024-07-24

- Fix import documentation version mismatch

## [0.4.2] - 2024-07-24

- improve doc
- multiple get functions for `Config` are added

## [0.4.1] - 2024-07-24

- improve doc

## [0.4.0] - 2024-07-23

- improve: make the cache behaves like native cache

## [0.3.5] - 2024-06-27

- doc: optimization suggestion

## [0.3.4] - 2024-06-26

- fix a bug when `default_config_dir` feature is enabled when macro expanded

## [0.3.3] - 2024-06-26

- fix doc error

## [0.3.2] - 2024-06-23

- improve code quality

## [0.3.1] - 2024-06-23

- fix doc and spell mistakes

## [0.3.0] - 2024-06-23

- refactor: Now, multi-config-sources can be saved and loaded through `Config` in one go. But `add_xx_source`s are removed. By the way, one can defined their own sources by implementing `Source` trait while `NormalSource` `PersistSource` `SecretSource` are still provided.

## [0.2.9] - 2024-06-21

- fix a doc mistake

## [0.2.8] - 2024-06-21

- doc: for linux doc
- remove unused error enum

## [0.2.7] - 2024-06-21

- conditional compilation for `keyring`

## [0.2.6] - 2024-06-19

- improve: (perf) speed up when reusing the Encrypter

## [0.2.5] - 2024-06-19

- fix: (bug) race condition when reusing the Encrypter

## [0.2.4] - 2024-06-19

- fix: (bug) always use the same encrypter even if different keyring entries are given
- doc: (derive macros) add doc for `PersistSource` and `SecretSource`
