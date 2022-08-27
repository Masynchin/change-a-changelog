pub struct Changelog {
    versions: Versions,
    repo: String,
}

impl Changelog {
    pub fn new(repo: String) -> Self {
        Self {
            versions: Versions::new(repo.clone()),
            repo,
        }
    }

    pub fn as_string(&self) -> String {
        let title: String = "# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)."
            .into();

        format!(
            "{}\n\n{}\n",
            title,
            self.versions.as_string()
        )
    }
}

struct Versions {
    unreleased: Link,
}

impl Versions {
    fn new(repo: String) -> Self {
        Self {
            unreleased: Link::new("Unreleased".into(), repo, "0.1.0".into(), "HEAD".into()),
        }
    }

    fn as_string(&self) -> String {
        format!(
            "{}\n\n{}",
            self.unreleased.title(),
            self.unreleased.as_string()
        )
    }
}

struct Link {
    tag: String,
    repo: String,
    left: String,
    right: String,
}

impl Link {
    pub fn new(tag: String, repo: String, left: String, right: String) -> Self {
        Self {
            tag,
            repo,
            left,
            right,
        }
    }

    pub fn title(&self) -> String {
        "[Unreleased]".to_string()
    }

    pub fn as_string(&self) -> String {
        format!(
            "[{}]: https://github.com/{}/compare/{}...{}",
            self.tag, self.repo, self.left, self.right
        )
    }
}
