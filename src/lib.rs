use std::{
    collections::HashMap,
    convert::From as StdFrom,
    fmt::{self, Debug, Display},
    hash::Hash,
};

pub trait Instruction: Debug + Display {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TagOrDigest {
    Tag(String),
    Digest(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct From {
    pub image: String,
    pub tag_or_digest: Option<TagOrDigest>,
    pub name: Option<String>,
}

impl<T> PartialEq<T> for From
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        // FROM rust:latest AS crab
        // 1    2           3  4
        // FROM rust
        // 1    2

        let vec: Vec<&str> = other.as_ref().split(' ').collect();
        let len = vec.len();
        match len {
            2 | 4 => {
                let from = vec[0].to_lowercase();
                if from != "from" {
                    return false;
                }

                let image = vec[1];
                let limage;
                if let Some(tod) = &self.tag_or_digest {
                    match tod {
                        TagOrDigest::Tag(tag) => limage = format!("{}:{}", self.image, tag),
                        TagOrDigest::Digest(digest) => {
                            limage = format!("{}@{}", self.image, digest)
                        }
                    }
                } else {
                    limage = self.image.clone();
                }
                if limage != *image {
                    return false;
                }

                if len == 4 {
                    let as_ = vec[2].to_lowercase();
                    if as_ != "as" {
                        return false;
                    }

                    let name = vec[3];
                    if let Some(lname) = &self.name {
                        if lname != name {
                            return false;
                        }
                    }
                }

                true
            }
            _ => false,
        }
    }
}

impl Display for From {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.tag_or_digest, &self.name) {
            (Some(TagOrDigest::Tag(tag)), None) => write!(f, "FROM {}:{}", self.image, tag),
            (Some(TagOrDigest::Tag(tag)), Some(name)) => {
                write!(f, "FROM {}:{} AS {}", self.image, tag, name)
            }
            (Some(TagOrDigest::Digest(digest)), None) => {
                write!(f, "FROM {}@{}", self.image, digest)
            }
            (Some(TagOrDigest::Digest(digest)), Some(name)) => {
                write!(f, "FROM {}@{} AS {}", self.image, digest, name)
            }
            (None, None) => write!(f, "FROM {}", self.image),
            (None, Some(name)) => write!(f, "FROM {} AS {}", self.image, name),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cmd {
    pub params: Vec<String>,
}

impl<T> StdFrom<T> for Cmd
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let params = s.as_ref().split(' ').map(String::from).collect();
        Cmd { params }
    }
}

impl<T> PartialEq<T> for Cmd
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.params
            .iter()
            .zip(other.as_ref().split(' '))
            .all(|(l, r)| l == r)
    }
}

impl Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CMD [{}]",
            self.params
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for Cmd {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Label {
    inner: HashMap<String, String>,
}

impl<K, V> StdFrom<HashMap<K, V>> for Label
where
    K: AsRef<str> + Eq + Hash,
    V: AsRef<str>,
{
    fn from(map: HashMap<K, V>) -> Self {
        let inner = map
            .iter()
            .map(|(k, v)| (String::from(k.as_ref()), String::from(v.as_ref())))
            .collect();
        Label { inner }
    }
}

impl<K, V> StdFrom<(K, V)> for Label
where
    K: AsRef<str> + Eq + Hash,
    V: AsRef<str>
{
    fn from((k, v): (K, V)) -> Self {
        let mut inner = HashMap::new();
        inner.insert(k.as_ref().to_string(), v.as_ref().to_string());
        Label { inner }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LABEL {}",
            self.inner
                .iter()
                .map(|(k, v)| format!(r#"{}="{}""#, k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Instruction for Label {}

/// Deprecated, use `LABEL maintainer=NAME` instead
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Maintainer {
    pub name: String,
}

impl<T> PartialEq<T> for Maintainer
where
    T: AsRef<str>
{
    fn eq(&self, other: &T) -> bool {
        self.name == other.as_ref()
    }
}

impl PartialEq<Label> for Maintainer {
    fn eq(&self, other: &Label) -> bool {
        if let Some(name) = other.inner.get("maintainer") {
            self.name == *name
        } else {
            false
        }
    }
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAINTAINER {}", self.name)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expose {
    pub port: u16,
    pub proto: String,
}

impl<T> PartialEq<T> for Expose
where
    T: AsRef<str>
{
    fn eq(&self, other: &T) -> bool {
        let vec: Vec<&str> = other.as_ref().split('/').collect();
        if vec.len() == 2 {
            let port = vec[0].parse::<u16>();
            let port = match port {
                Ok(port) => port == self.port,
                Err(_) => false,
            };

            self.proto == vec[1] && port
        } else {
            false
        }
    }
}

impl Display for Expose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EXPOSE {}/{}", self.port, self.proto)
    }
}

impl Instruction for Expose {}

#[derive(Debug)]
pub struct Env {
    inner: HashMap<String, String>,
}

impl<K, V> StdFrom<HashMap<K, V>> for Env
    where
        K: AsRef<str> + Eq + Hash,
        V: AsRef<str>,
{
    fn from(map: HashMap<K, V>) -> Self {
        let inner = map
            .iter()
            .map(|(k, v)| (String::from(k.as_ref()), String::from(v.as_ref())))
            .collect();
        Env { inner }
    }
}

impl<K, V> StdFrom<(K, V)> for Env
    where
        K: AsRef<str> + Eq + Hash,
        V: AsRef<str>
{
    fn from((k, v): (K, V)) -> Self {
        let mut inner = HashMap::new();
        inner.insert(k.as_ref().to_string(), v.as_ref().to_string());
        Env { inner }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ENV {}",
            self.inner
                .iter()
                .map(|(k, v)| format!(r#"{}="{}""#, k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Instruction for Env {}

#[derive(Debug)]
pub struct Add {
    pub src: String,
    pub dst: String,
    pub chown: Option<User>,
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.chown {
            Some(chown) => write!(f, r#"ADD {} "{}" "{}""#, chown, self.src, self.dst),
            None => write!(f, r#"ADD "{}" "{}""#, self.src, self.dst),
        }
    }
}

impl Instruction for Add {}

#[derive(Debug)]
pub struct Copy {
    pub src: String,
    pub dst: String,
    pub chown: Option<User>,
}

impl Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.chown {
            Some(chown) => write!(f, r#"COPY {} "{}" "{}""#, chown, self.src, self.dst),
            None => write!(f, r#"COPY "{}" "{}""#, self.src, self.dst),
        }
    }
}

impl Instruction for Copy {}

#[derive(Debug)]
pub struct EntryPoint {
    inner: Vec<String>,
}

impl Display for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ENTRYPOINT [{}]",
            self.inner
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for EntryPoint {}

#[derive(Debug)]
pub struct Volume {
    inner: Vec<String>,
}

impl Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VOLUME {}",
            self.inner
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for Volume {}

#[derive(Debug)]
pub struct User {
    pub user: String,
    pub group: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.group {
            Some(group) => write!(f, "USER {}:{}", self.user, group),
            None => write!(f, "USER {}", self.user),
        }
    }
}

impl Instruction for User {}

#[derive(Debug)]
pub struct WorkDir {
    pub path: String,
}

impl Display for WorkDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WORKDIR {}", self.path)
    }
}

impl Instruction for WorkDir {}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub value: Option<String>,
}

impl Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, r#"ARG {}="{}""#, self.name, value),
            None => write!(f, "ARG {}", self.name),
        }
    }
}

impl Instruction for Arg {}

#[derive(Debug)]
pub struct StopSignal {
    inner: String,
}

impl Display for StopSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STOPSIGNAL {}", self.inner)
    }
}

impl Instruction for StopSignal {}

#[derive(Debug)]
pub enum HealthCheck {
    Check {
        cmd: Cmd,
        interval: Option<i32>,
        timeout: Option<i32>,
        start_period: Option<i32>,
        retries: Option<i32>,
    },
    None,
}

impl Display for HealthCheck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthCheck::Check {
                cmd,
                interval,
                timeout,
                start_period,
                retries,
            } => {
                write!(f, "HEALTHCHECK ")?;
                if let Some(interval) = interval {
                    write!(f, "--interval={} ", interval)?;
                }
                if let Some(timeout) = timeout {
                    write!(f, "--timeout={} ", timeout)?;
                }
                if let Some(period) = start_period {
                    write!(f, "--start-period={} ", period)?;
                }
                if let Some(retries) = retries {
                    write!(f, "--retries={} ", retries)?;
                }
                write!(f, "{}", cmd)
            }
            HealthCheck::None => write!(f, "HEALTHCHECK NONE"),
        }
    }
}

#[derive(Debug)]
pub struct Shell {
    inner: Vec<String>,
}

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SHELL [{}]",
            self.inner
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct OnBuild {
    inner: Box<Instruction>,
}

impl Display for OnBuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ONBUILD {}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let image = String::from("rust");
        let tag = Some(TagOrDigest::Tag("latest".into()));
        let digest = Some(TagOrDigest::Digest("digest".into()));
        let name = Some(String::from("crab"));

        // tag and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: tag.clone(),
            name: None,
        };
        assert_eq!(from, "FROM rust:latest");

        // tag and name
        let from = From {
            image: image.clone(),
            tag_or_digest: tag.clone(),
            name: name.clone(),
        };
        assert_eq!(from, "FROM rust:latest AS crab");

        // digest and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: digest.clone(),
            name: None,
        };
        assert_eq!(from, "FROM rust@digest");

        // digest and name
        let from = From {
            image: image.clone(),
            tag_or_digest: digest.clone(),
            name: name.clone(),
        };
        assert_eq!(from, "FROM rust@digest AS crab");

        // no tag or digest and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: None,
            name: None,
        };
        assert_eq!(from, "FROM rust");

        // no tag or digest and name
        let from = From {
            image: image.clone(),
            tag_or_digest: None,
            name: name.clone(),
        };
        assert_eq!(from, "FROM rust AS crab");

        assert_ne!(from, "some_string")
    }

    #[test]
    fn cmd() {
        let curl = "curl -v https://rust-lang.org";
        let cmd = Cmd::from(curl);
        assert_eq!(cmd.params, ["curl", "-v", "https://rust-lang.org"]);
        assert_eq!(cmd, curl);
        assert_eq!(
            cmd.to_string(),
            r#"CMD ["curl", "-v", "https://rust-lang.org"]"#
        )
    }

    #[test]
    fn label() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        map.insert("hello", "world");
        let label = Label::from(map.clone());
        assert_eq!(label.to_string(), r#"LABEL hello="world" key="value""#);
    }

    #[test]
    fn maintainer() {
        let name = String::from("Someone Rustcean");
        let maintainer = Maintainer { name: name.clone() };
        assert_eq!(maintainer, name);
        assert_eq!(maintainer.to_string(), "MAINTAINER Someone Rustcean");
        assert_eq!(maintainer, Label::from(("maintainer", name)))
    }

    #[test]
    fn expose() {
        let port = 80;
        let proto = String::from("tcp");
        let expose = Expose { port, proto };
        assert_eq!(expose, "80/tcp");
        assert_eq!(expose.to_string(), "EXPOSE 80/tcp")
    }

    #[test]
    fn env() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        map.insert("hello", "world");
        let label = Env::from(map.clone());
        assert_eq!(label.to_string(), r#"ENV hello="world" key="value""#);
    }
}
