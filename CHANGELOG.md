# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 - 2024-11-21

### Added

- `Serializer` now supports fields of sequence type by serializing one key-value pair for each
  sequence element.

### Removed

- `Serializer` no longer allows map (e.g. `HashMap`) or sequence-of-tuple (e.g. `Vec<String, T>`)
  types. Only named structs are supported.
