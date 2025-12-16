# Changelog

All notable changes to this project will be documented in this file.

## 0.12.2 - 2025-12-16

### Features

* Add hurd to target_os lists

### CI

* Increase minimum Rust build version in CI to 1.68.
* Split into unit and integration test and add more platforms.
* Bump actions/checkout from 4 to 6

### Testing

* Add test_all_users and test_all_groups

## 0.12.1 - 2024-08-03

### Bug Fixes
* Fix disabling the cache feature by @viperML in https://github.com/rustadopt/uzers-rs/pull/22

### Updates
* Update serial_test requirement from ^2.0 to ^3.1
* Update env_logger requirement from 0.7 to 0.11


## 0.12.0 - 2024-04-23

### What's Changed
- Added explicit references to os module in docs index
- Add `AllUsers` and `AllGroups`
- Test CI with nss_preload
- Add GECOS field on UNIX systems
- Add conventional commits workflow
- Split general workflow into testing and linting


## [0.11.3] - 2023-09-11

### Bug Fixes
- Fix unaligned pointer in base::members function

### Features
- Add haiku support

### CI
- Bump actions/checkout from 3 to 4


## [0.11.2] - 2023-08-25
This just updates the repository URL as we moved to the newly created
rustadopt organization on Github.


## [0.11.1] - 2023-08-21

This is the first version of uzers which continues the unmaintained
users v0.11.0.

### Bug Fixes
- Fix group listing: don't add root every time

### Features
- Allow iterating all groups in the system
- Add redox and illumos support

### Refactor
- Reformat entire code base

### Documentation
- Rename to uzers
- Add this changelog

### CI
- Add Github workflows
