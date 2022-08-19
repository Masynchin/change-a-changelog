pub struct Changelog {
    link: Link,
}

impl Changelog {
    pub fn new() -> Self {
        Self {
            link: Link::new(
                "Unreleased".into(),
                "olivierlacan/keep-a-changelog".into(),
                "0.1.0".into(),
                "HEAD".into(),
            ),
        }
    }

    pub fn as_string(&self) -> String {
        let mut title: String = "# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]\n"
            .into();

        title.push_str("\n");
        title.push_str(self.link.as_string().as_str());
        title.push_str("\n");

        title
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

    pub fn as_string(&self) -> String {
        format!(
            "[{}]: https://github.com/{}/compare/{}...{}",
            self.tag, self.repo, self.left, self.right
        )
    }
}
