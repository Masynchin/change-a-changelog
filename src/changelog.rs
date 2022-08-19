pub struct Changelog {}

impl Changelog {
    pub fn new() -> Self {
        Self {}
    }

    pub fn as_bytes(&self) -> &[u8] {
        "# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.1.0...HEAD"
            .as_bytes()
    }
}
