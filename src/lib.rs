mod builder;

pub mod macros;

pub use builder::DockerFile;

use std::{
    collections::HashMap,
    convert::From as StdFrom,
    fmt::{self, Display},
    hash::Hash,
};

pub trait Instruction: Display {}

trait StorageInstruction: Instruction {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TagOrDigest {
    Tag(String),
    Digest(String),
}

pub use TagOrDigest::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct From {
    pub image: String,
    pub tag_or_digest: Option<TagOrDigest>,
    pub name: Option<String>,
}

impl Display for From {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.tag_or_digest, &self.name) {
            (Some(Tag(tag)), None) => write!(f, "FROM {}:{}", self.image, tag),
            (Some(Tag(tag)), Some(name)) => write!(f, "FROM {}:{} AS {}", self.image, tag, name),
            (Some(Digest(digest)), None) => write!(f, "FROM {}@{}", self.image, digest),
            (Some(Digest(digest)), Some(name)) => {
                write!(f, "FROM {}@{} AS {}", self.image, digest, name)
            }
            (None, None) => write!(f, "FROM {}", self.image),
            (None, Some(name)) => write!(f, "FROM {} AS {}", self.image, name),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Run {
    pub params: Vec<String>,
}

impl<I, S> StdFrom<I> for Run
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(iter: I) -> Self {
        let params = iter.into_iter().map(|i| i.as_ref().to_string()).collect();
        Run { params }
    }
}

impl Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RUN [{}]",
            self.params
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for Run {}
impl StorageInstruction for Run {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cmd {
    pub params: Vec<String>,
}

impl<I, S> StdFrom<I> for Cmd
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(iter: I) -> Self {
        let params = iter.into_iter().map(|i| i.as_ref().to_string()).collect();
        Cmd { params }
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
            .map(|(k, v)| (String::from(k.as_ref()), v.as_ref().replace('\n', "\\\n")))
            .collect();
        Label { inner }
    }
}

impl<K, V> StdFrom<(K, V)> for Label
where
    K: AsRef<str> + Eq + Hash,
    V: AsRef<str>,
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
                .join(" \\\n      ")
        )
    }
}

impl Instruction for Label {}
impl StorageInstruction for Label {}

/// Deprecated, use [`Label`] with `maintainer` key instead
///
/// [`Label`]: struct.Label.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Maintainer {
    pub name: String,
}

impl<T> StdFrom<T> for Maintainer
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let name = s.as_ref().to_string();
        Maintainer { name }
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
    pub proto: Option<String>,
}

impl StdFrom<u16> for Expose {
    fn from(port: u16) -> Self {
        Expose { port, proto: None }
    }
}

impl Display for Expose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EXPOSE {}{}",
            self.port,
            self.proto
                .clone()
                .map(|s| format!("/{}", s))
                .unwrap_or_default()
        )
    }
}

impl Instruction for Expose {}
impl StorageInstruction for Expose {}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    V: AsRef<str>,
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
impl StorageInstruction for Env {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Add {
    pub src: String,
    pub dst: String,
    pub chown: Option<User>,
}

impl<K, V> StdFrom<(K, V)> for Add
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from((k, v): (K, V)) -> Self {
        let src = k.as_ref().to_string();
        let dst = v.as_ref().to_string();
        Add {
            src,
            dst,
            chown: None,
        }
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.chown {
            Some(chown) => write!(
                f,
                r#"ADD --chown={}{} "{}" "{}""#,
                chown.user,
                chown
                    .group
                    .clone()
                    .map(|s| format!(":{}", s))
                    .unwrap_or_default(),
                self.src,
                self.dst
            ),
            None => write!(f, r#"ADD "{}" "{}""#, self.src, self.dst),
        }
    }
}

impl Instruction for Add {}
impl StorageInstruction for Add {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Copy {
    pub src: String,
    pub dst: String,
    pub from: Option<String>,
    pub chown: Option<User>,
}

impl<K, V> StdFrom<(K, V)> for Copy
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from((k, v): (K, V)) -> Self {
        let src = k.as_ref().to_string();
        let dst = v.as_ref().to_string();
        Copy {
            src,
            dst,
            from: None,
            chown: None,
        }
    }
}

impl Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.from, &self.chown) {
            (Some(from), Some(chown)) => write!(
                f,
                r#"COPY --from={} --chown={}{} "{}" "{}""#,
                from,
                chown.user,
                chown
                    .group
                    .clone()
                    .map(|s| format!(":{}", s))
                    .unwrap_or_default(),
                self.src,
                self.dst
            ),
            (Some(from), None) => {
                write!(f, r#"COPY --from={} "{}" "{}""#, from, self.src, self.dst)
            }
            (None, Some(chown)) => write!(
                f,
                r#"COPY --chown={}{} "{}" "{}""#,
                chown.user,
                chown
                    .group
                    .clone()
                    .map(|group| format!(":{}", group))
                    .unwrap_or_default(),
                self.src,
                self.dst
            ),
            (None, None) => write!(f, r#"COPY "{}" "{}""#, self.src, self.dst),
        }
    }
}

impl Instruction for Copy {}
impl StorageInstruction for Copy {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntryPoint {
    params: Vec<String>,
}

impl<I, S> StdFrom<I> for EntryPoint
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(iter: I) -> Self {
        let params = iter.into_iter().map(|i| i.as_ref().to_string()).collect();
        EntryPoint { params }
    }
}

impl Display for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ENTRYPOINT [{}]",
            self.params
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for EntryPoint {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Volume {
    pub paths: Vec<String>,
}

impl<I, S> StdFrom<I> for Volume
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(iter: I) -> Self {
        let paths = iter.into_iter().map(|i| i.as_ref().to_string()).collect();
        Volume { paths }
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VOLUME [{}]",
            self.paths
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for Volume {}
impl StorageInstruction for Volume {}

#[derive(Debug, Clone, Eq, PartialEq)]
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
impl StorageInstruction for User {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkDir {
    pub path: String,
}

impl<T> StdFrom<T> for WorkDir
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let path = s.as_ref().to_string();
        WorkDir { path }
    }
}

impl Display for WorkDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"WORKDIR "{}""#, self.path)
    }
}

impl Instruction for WorkDir {}
impl StorageInstruction for WorkDir {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Arg {
    pub name: String,
    pub value: Option<String>,
}

impl<K, V> StdFrom<(K, V)> for Arg
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from((k, v): (K, V)) -> Self {
        let name = k.as_ref().to_string();
        let value = v.as_ref().to_string();
        Arg {
            name,
            value: Some(value),
        }
    }
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
impl StorageInstruction for Arg {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StopSignal {
    pub signal: String,
}

impl<T> StdFrom<T> for StopSignal
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let signal = s.as_ref().to_string();
        StopSignal { signal }
    }
}

impl Display for StopSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STOPSIGNAL {}", self.signal)
    }
}

impl Instruction for StopSignal {}
impl StorageInstruction for StopSignal {}

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl Instruction for HealthCheck {}
impl StorageInstruction for HealthCheck {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Shell {
    pub params: Vec<String>,
}

impl<I, S> StdFrom<I> for Shell
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(iter: I) -> Self {
        let params = iter.into_iter().map(|i| i.as_ref().to_string()).collect();
        Shell { params }
    }
}

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SHELL [{}]",
            self.params
                .iter()
                .map(|i| format!(r#""{}""#, i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Instruction for Shell {}
impl StorageInstruction for Shell {}

pub struct OnBuild {
    inner: Box<Instruction>,
}

impl<I> StdFrom<I> for OnBuild
where
    I: Instruction + 'static,
{
    fn from(i: I) -> Self {
        let inner = Box::new(i);
        OnBuild { inner }
    }
}

impl Display for OnBuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ONBUILD {}", self.inner)
    }
}

pub struct Comment {
    pub comment: String,
}

impl<T> StdFrom<T> for Comment
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let comment = s.as_ref().to_string();
        Comment { comment }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "# {}", self.comment)
    }
}

impl Instruction for Comment {}
impl StorageInstruction for Comment {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let image = String::from("rust");
        let tag = Some(Tag("latest".into()));
        let digest = Some(Digest("digest".into()));
        let name = Some(String::from("crab"));

        // tag and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: tag.clone(),
            name: None,
        };
        assert_eq!(from.to_string(), "FROM rust:latest");

        // tag and name
        let from = From {
            image: image.clone(),
            tag_or_digest: tag.clone(),
            name: name.clone(),
        };
        assert_eq!(from.to_string(), "FROM rust:latest AS crab");

        // digest and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: digest.clone(),
            name: None,
        };
        assert_eq!(from.to_string(), "FROM rust@digest");

        // digest and name
        let from = From {
            image: image.clone(),
            tag_or_digest: digest.clone(),
            name: name.clone(),
        };
        assert_eq!(from.to_string(), "FROM rust@digest AS crab");

        // no tag or digest and no name
        let from = From {
            image: image.clone(),
            tag_or_digest: None,
            name: None,
        };
        assert_eq!(from.to_string(), "FROM rust");

        // no tag or digest and name
        let from = From {
            image: image.clone(),
            tag_or_digest: None,
            name: name.clone(),
        };
        assert_eq!(from.to_string(), "FROM rust AS crab");
    }

    #[test]
    fn run() {
        let curl = &["curl", "-v", "https://rust-lang.org"];
        let run = Run::from(curl);
        assert_eq!(run.params, ["curl", "-v", "https://rust-lang.org"]);
        assert_eq!(
            run.to_string(),
            r#"RUN ["curl", "-v", "https://rust-lang.org"]"#
        )
    }

    #[test]
    fn cmd() {
        let curl = &["curl", "-v", "https://rust-lang.org"];
        let cmd = Cmd::from(curl);
        assert_eq!(cmd.params, ["curl", "-v", "https://rust-lang.org"]);
        assert_eq!(
            cmd.to_string(),
            r#"CMD ["curl", "-v", "https://rust-lang.org"]"#
        )
    }

    #[test]
    fn label() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        let label = Label::from(map);
        assert_eq!(label.to_string(), r#"LABEL key="value""#);

        let mut map = HashMap::new();
        map.insert("key", "1\n2\n3");
        let label = Label::from(map);
        assert_eq!(
            label.to_string(),
            r#"LABEL key="1\
2\
3""#
        );

        let mut map = HashMap::new();
        map.insert("key", "value");
        map.insert("hello", "world");
        let label = Label::from(map);
        let label = label.to_string();
        assert!(
            label
                == r#"LABEL hello="world" \
      key="value""#
                || label
                    == r#"LABEL key="value" \
      hello="world""#
        );
    }

    #[test]
    fn maintainer() {
        let name = String::from("Someone Rustacean");
        let maintainer = Maintainer::from(name.clone());
        assert_eq!(maintainer.to_string(), "MAINTAINER Someone Rustacean");
        assert_eq!(maintainer, Label::from(("maintainer", name)))
    }

    #[test]
    fn expose() {
        let port = 80;
        let proto = Some(String::from("tcp"));

        // without proto
        let expose = Expose { port, proto: None };
        assert_eq!(expose.to_string(), "EXPOSE 80");

        // with proto
        let expose = Expose { port, proto };
        assert_eq!(expose.to_string(), "EXPOSE 80/tcp")
    }

    #[test]
    fn env() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        let label = Env::from(map.clone());
        assert_eq!(label.to_string(), r#"ENV key="value""#);
    }

    #[test]
    fn add() {
        let chown = User {
            user: "rustacean".to_string(),
            group: None,
        };
        let src = "/home/container001".to_string();
        let dst = "/".to_string();

        // with chown
        let add = Add {
            src: src.clone(),
            dst: dst.clone(),
            chown: Some(chown),
        };
        assert_eq!(
            add.to_string(),
            r#"ADD --chown=rustacean "/home/container001" "/""#
        );

        // without chown
        let add = Add::from((src.clone(), dst.clone()));
        assert_eq!(add.to_string(), r#"ADD "/home/container001" "/""#);
    }

    #[test]
    fn copy() {
        let from = Some("crab".to_string());
        let chown = Some(User {
            user: "rustacean".to_string(),
            group: Some("root".to_string()),
        });
        let src = "/home/container001".to_string();
        let dst = "/".to_string();

        // with from and with chown
        let copy = Copy {
            src: src.clone(),
            dst: dst.clone(),
            from: from.clone(),
            chown: chown.clone(),
        };
        assert_eq!(
            copy.to_string(),
            r#"COPY --from=crab --chown=rustacean:root "/home/container001" "/""#
        );

        // with from
        let copy = Copy {
            src: src.clone(),
            dst: dst.clone(),
            from: from.clone(),
            chown: None,
        };
        assert_eq!(
            copy.to_string(),
            r#"COPY --from=crab "/home/container001" "/""#
        );

        // with chown
        let copy = Copy {
            src: src.clone(),
            dst: dst.clone(),
            from: None,
            chown: chown.clone(),
        };
        assert_eq!(
            copy.to_string(),
            r#"COPY --chown=rustacean:root "/home/container001" "/""#
        );

        // without from and without chown
        let copy = Copy::from((src.clone(), dst.clone()));
        assert_eq!(copy.to_string(), r#"COPY "/home/container001" "/""#);
    }

    #[test]
    fn entrypoint() {
        let curl = &["curl", "-v", "https://rust-lang.org"];
        let point = EntryPoint::from(curl);
        assert_eq!(point.params, ["curl", "-v", "https://rust-lang.org"]);
        assert_eq!(
            point.to_string(),
            r#"ENTRYPOINT ["curl", "-v", "https://rust-lang.org"]"#
        )
    }

    #[test]
    fn volume() {
        let paths = vec!["/var/run"];
        let volume = Volume::from(paths);
        assert_eq!(volume.to_string(), r#"VOLUME ["/var/run"]"#);
    }

    #[test]
    fn user() {
        let user = "rustacean".to_string();
        let group = Some("root".to_string());

        // with group
        let usr = User {
            user: user.clone(),
            group,
        };
        assert_eq!(usr.to_string(), "USER rustacean:root");

        // without group
        let usr = User { user, group: None };
        assert_eq!(usr.to_string(), "USER rustacean");
    }

    #[test]
    fn workdir() {
        let path = "/var/run";
        let dir = WorkDir::from(path);
        assert_eq!(dir.to_string(), r#"WORKDIR "/var/run""#)
    }

    #[test]
    fn arg() {
        let name = "name".to_string();
        let value = Some("value".to_string());

        // with value
        let arg = Arg {
            name: name.clone(),
            value,
        };
        assert_eq!(arg.to_string(), r#"ARG name="value""#);

        // without value
        let arg = Arg { name, value: None };
        assert_eq!(arg.to_string(), r#"ARG name"#);
    }

    #[test]
    fn stopsignal() {
        let signal = "SIGKILL".to_string();
        let signal = StopSignal::from(signal);
        assert_eq!(signal.to_string(), "STOPSIGNAL SIGKILL");
    }

    #[test]
    fn healthcheck() {
        // with params
        let cmd = Cmd::from(&["curl", "-v", "https://rust-lang.org"]);
        let check = HealthCheck::Check {
            cmd,
            interval: Some(0),
            timeout: Some(3600),
            start_period: Some(123),
            retries: Some(2),
        };
        assert_eq!(check.to_string(), r#"HEALTHCHECK --interval=0 --timeout=3600 --start-period=123 --retries=2 CMD ["curl", "-v", "https://rust-lang.org"]"#);

        // without params
        let check = HealthCheck::None;
        assert_eq!(check.to_string(), "HEALTHCHECK NONE");
    }

    #[test]
    fn shell() {
        let bash = &["bash", "-c"];
        let shell = Shell::from(bash);
        assert_eq!(shell.params, ["bash", "-c"]);
        assert_eq!(shell.to_string(), r#"SHELL ["bash", "-c"]"#)
    }

    #[test]
    fn onbuild() {
        let cmd = Cmd::from(&["curl", "-v", "https://rust-lang.org"]);
        let onbuild = OnBuild::from(cmd);
        assert_eq!(
            onbuild.to_string(),
            r#"ONBUILD CMD ["curl", "-v", "https://rust-lang.org"]"#
        );
    }

    #[test]
    fn comment() {
        let comment = "This is an example comment";
        let comment = Comment::from(comment);
        assert_eq!(comment.to_string(), "# This is an example comment");
    }
}
