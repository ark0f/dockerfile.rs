#![allow(non_snake_case)]

/// ```rust,no_run
/// # use dockerfile_rs::FROM;
/// let from = FROM!(rust:latest);
/// assert_eq!(from.to_string(), "FROM rust:latest");
/// ```
#[macro_export]
macro_rules! FROM {
    ($image:ident) => {{
        use $crate::From;
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: None,
            name: None,
        }
    }};
    ($image:ident AS $name:ident) => {{
        use $crate::From;
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: None,
            name: Some(stringify!($name).to_string()),
        }
    }};
    ($image:ident:$tag:ident) => {{
        use $crate::{From, Tag};
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: Some(Tag(stringify!($tag).to_string())),
            name: None,
        }
    }};
    ($image:ident:$tag:ident AS $name:ident) => {{
        use $crate::{From, Tag};
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: Some(Tag(stringify!($tag).to_string())),
            name: Some(stringify!($name).to_string()),
        }
    }};
    ($image:ident@$digest:ident) => {{
        use $crate::{Digest, From};
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: Some(Digest(stringify!($digest).to_string())),
            name: None,
        }
    }};
    ($image:ident@$digest:ident AS $name:ident) => {{
        use $crate::{Digest, From};
        From {
            image: stringify!($image).to_string(),
            tag_or_digest: Some(Digest(stringify!($digest).to_string())),
            name: Some(stringify!($name).to_string()),
        }
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::RUN;
/// let run = RUN!["echo", "Hello, world!"];
/// assert_eq!(run.to_string(), r#"RUN ["echo", "Hello, world!"]"#);
/// ```
#[macro_export]
macro_rules! RUN {
    ($($x:expr), +) => {{
        use $crate::Run;
        Run::from(vec![$($x), +])
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::CMD;
/// let cmd = CMD!["echo", "Hello, world!"];
/// assert_eq!(cmd.to_string(), r#"CMD ["echo", "Hello, world!"]"#);
/// ```
#[macro_export]
macro_rules! CMD {
    ($($x:expr), +) => {{
        use $crate::Cmd;
        Cmd::from(vec![$($x), +])
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::LABEL;
/// let label = LABEL!["key" => "value"];
/// assert_eq!(label.to_string(), r#"LABEL key="value""#);
/// ```
#[macro_export]
macro_rules! LABEL {
    ($($x:expr => $y:expr), +) => {{
        use $crate::Label;
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert($x, $y);
        )+
        Label::from(map)
    }};
}

/// Deprecated, use [`LABEL!`] with `maintainer` key instead
/// # Example
/// ```rust,no_run
/// # use dockerfile_rs::MAINTAINER;
/// let maintainer = MAINTAINER!("Rustacean");
/// assert_eq!(maintainer.to_string(), r#"MAINTAINER Rustacean"#);
/// ```
///
///// [`LABEL!`]: macro.LABEL.html
#[macro_export]
macro_rules! MAINTAINER {
    ($name:expr) => {{
        use $crate::Maintainer;
        Maintainer::from($name)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::EXPOSE;
/// let expose = EXPOSE!(5757/udp);
/// assert_eq!(expose.to_string(), "EXPOSE 5757/udp");
/// ```
#[macro_export]
macro_rules! EXPOSE {
    ($port:tt/$proto:ident) => {{
        use $crate::Expose;
        Expose {
            port: $port,
            proto: Some(stringify!($proto).to_string()),
        }
    }};
    ($port:expr) => {{
        use $crate::Expose;
        Expose::from($port)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::ENV;
/// let env = ENV!["BUILD" => "true"];
/// assert_eq!(env.to_string(), r#"ENV BUILD="true""#);
/// ```
#[macro_export]
macro_rules! ENV {
    ($($x:expr => $y:expr), +) => {{
        use $crate::Env;
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert($x, $y);
        )+
        Env::from(map)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::ADD;
/// let add = ADD!("/var/run" "/home");
/// assert_eq!(add.to_string(), r#"ADD "/var/run" "/home""#);
/// ```
#[macro_export]
macro_rules! ADD {
    (--chown=$user:ident:$group:ident $src:tt $dst:tt) => {{
        use $crate::{Add, User};
        Add {
            src: $src.to_string(),
            dst: $dst.to_string(),
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: Some(stringify!($group).to_string()),
            }),
        }
    }};
    (--chown=$user:ident $src:tt $dst:tt) => {{
        use $crate::{Add, User};
        Add {
            src: $src.to_string(),
            dst: $dst.to_string(),
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: None,
            }),
        }
    }};
    ($src:tt $dst:tt) => {{
        use $crate::Add;
        Add {
            src: $src.to_string(),
            dst: $dst.to_string(),
            chown: None,
        }
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::COPY;
/// let copy = COPY!("." ".");
/// assert_eq!(copy.to_string(), r#"COPY "." ".""#);
/// ```
#[macro_export]
macro_rules! COPY {
    (--from=$name:ident --chown=$user:ident:$group:ident $src:tt $dst:tt) => {{
        use $crate::{Copy, User};
        Copy {
            src: $src.to_string(),
            dst: $dst.to_string(),
            from: Some(stringify!($from).to_string()),
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: Some(stringify!($group).to_string()),
            }),
        }
    }};
    (--from=$name:ident --chown=$user:ident $src:tt $dst:tt) => {{
        use $crate::{Copy, User};
        Copy {
            src: $src.to_string(),
            dst: $dst.to_string(),
            from: Some(stringify!($from).to_string()),
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: None,
            }),
        }
    }};
    (--chown=$user:ident:$group:ident $src:tt $dst:tt) => {{
        use $crate::{Copy, User};
        Copy {
            src: $src.to_string(),
            dst: $dst.to_string(),
            from: None,
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: Some(stringify!($group).to_string()),
            }),
        }
    }};
    (--chown=$user:ident $src:tt $dst:tt) => {{
        use $crate::{Copy, User};
        Copy {
            src: $src.to_string(),
            dst: $dst.to_string(),
            from: None,
            chown: Some(User {
                user: stringify!($user).to_string(),
                group: None,
            }),
        }
    }};
    ($src:tt $dst:tt) => {{
        use $crate::Copy;
        Copy {
            src: $src.to_string(),
            dst: $dst.to_string(),
            from: None,
            chown: None,
        }
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::ENTRYPOINT;
/// let entry_point = ENTRYPOINT!["/bin/bash/", "-c", "echo"];
/// assert_eq!(entry_point.to_string(), r#"ENTRYPOINT ["/bin/bash/", "-c", "echo"]"#);
/// ```
#[macro_export]
macro_rules! ENTRYPOINT {
    ($($x:expr), +) => {{
        use $crate::EntryPoint;
        EntryPoint::from(vec![$($x), +])
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::VOLUME;
/// let volume = VOLUME!("/var/run", "/var/www");
/// assert_eq!(volume.to_string(), r#"VOLUME ["/var/run", "/var/www"]"#);
/// ```
#[macro_export]
macro_rules! VOLUME {
    ($($x:expr), +) => {{
        use $crate::Volume;
        Volume::from(vec![$($x), +])
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::USER;
/// let user = USER!(rustacean);
/// assert_eq!(user.to_string(), r#"USER rustacean"#);
/// ```
#[macro_export]
macro_rules! USER {
    ($user:ident:$group:ident) => {{
        use $crate::User;
        User {
            user: stringify!($user).to_string(),
            group: Some(stringify!($group).to_string()),
        }
    }};
    ($user:ident) => {{
        use $crate::User;
        User {
            user: stringify!($user).to_string(),
            group: None,
        }
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::WORKDIR;
/// let work_dir = WORKDIR!("/home/container001");
/// assert_eq!(work_dir.to_string(), r#"WORKDIR "/home/container001""#);
/// ```
#[macro_export]
macro_rules! WORKDIR {
    ($dir:expr) => {{
        use $crate::WorkDir;
        WorkDir::from($dir)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::ARG;
/// let arg = ARG!("key" => "value");
/// assert_eq!(arg.to_string(), r#"ARG key="value""#);
/// ```
#[macro_export]
macro_rules! ARG {
    ($x:expr => $y:expr) => {{
        use $crate::Arg;
        Arg::from(($x, $y))
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::STOPSIGNAL;
/// let signal = STOPSIGNAL!("SIGKILL");
/// assert_eq!(signal.to_string(), "STOPSIGNAL SIGKILL");
/// ```
#[macro_export]
macro_rules! STOPSIGNAL {
    ($signal:expr) => {{
        use $crate::StopSignal;
        StopSignal::from($signal)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::HEALTHCHECK;
/// let health_check = HEALTHCHECK!(NONE);
/// assert_eq!(health_check.to_string(), "HEALTHCHECK NONE");
/// ```
#[macro_export]
macro_rules! HEALTHCHECK {
    (NONE) => {{
        use $crate::HealthCheck;
        HealthCheck::None
    }};
    (CMD $cmd:expr) => {{
        use $crate::{Cmd, HealthCheck};
        HealthCheck::Check {
            cmd: Cmd::from($cmd),
            interval: None,
            timeout: None,
            start_period: None,
            retries: None,
        }
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::SHELL;
/// let shell = SHELL!["/bin/bash", "-c"];
/// assert_eq!(shell.to_string(), r#"SHELL ["/bin/bash", "-c"]"#);
/// ```
#[macro_export]
macro_rules! SHELL {
    ($($x:expr), +) => {{
        use $crate::Shell;
        Shell::from(vec![$($x), +])
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::ONBUILD;
/// # use dockerfile_rs::ENV;
/// let on_build = ONBUILD!(ENV!["key" => "value"]);
/// assert_eq!(on_build.to_string(), r#"ONBUILD ENV key="value""#);
/// ```
#[macro_export]
macro_rules! ONBUILD {
    ($x:expr) => {{
        use $crate::OnBuild;
        OnBuild::from($x)
    }};
}

/// ```rust,no_run
/// # use dockerfile_rs::COMMENT;
/// let comment = COMMENT!("Hello!");
/// assert_eq!(comment.to_string(), "# Hello!");
/// ```
#[macro_export]
macro_rules! COMMENT {
    ($x:expr) => {{
        use $crate::Comment;
        Comment::from($x)
    }};
}

mod tests {
    #[test]
    fn from() {
        let _ = FROM!(rust);
        let _ = FROM!(rust AS crab);
        let _ = FROM!(rust: latest);
        let _ = FROM!(rust:latest AS crab);
        let _ = FROM!(rust@digest);
        let _ = FROM!(rust@digest AS crab);
    }

    #[test]
    fn run() {
        let _ = RUN!["/bin/bash", "-c", "echo"];
    }

    #[test]
    fn cmd() {
        let _ = CMD!["echo", "Hello, world!"];
    }

    #[test]
    fn label() {
        let _ = LABEL!["key" => "value"];
        let _ = LABEL!["key" => "value", "hello" => "world"];
    }

    #[test]
    fn maintainer() {
        let _ = MAINTAINER!("Funny Rustacean");
    }

    #[test]
    fn expose() {
        let _ = EXPOSE!(80 / tcp);
        let _ = EXPOSE!(443);
    }

    #[test]
    fn env() {
        let _ = ENV!["key" => "value"];
        let _ = ENV!["key" => "value", "hello" => "world"];
    }

    #[test]
    fn add() {
        let _ = ADD!(--chown=rustacean:root "/var/run" "/home");
        let _ = ADD!(--chown=rustacean "/var/run" "/home");
        let _ = ADD!("/var/run" "/home");
    }

    #[test]
    fn copy() {
        let _ = COPY!(--from=crab --chown=rustacean:root "/var/run" "/home");
        let _ = COPY!(--from=crab --chown=rustacean "/var/run" "/home");
        let _ = COPY!(--chown=rustacean:root "/var/run" "/home");
        let _ = COPY!(--chown=rustacean "/var/run" "/home");
        let _ = COPY!("/var/run" "/home");
    }

    #[test]
    fn entry_point() {
        let _ = ENTRYPOINT!["echo", "Hello, world!"];
    }

    #[test]
    fn volume() {
        let _ = VOLUME!["/var/run", "/home"];
    }

    #[test]
    fn user() {
        let _ = USER!(rustacean: root);
        let _ = USER!(rustacean);
    }

    #[test]
    fn work_dir() {
        let _ = WORKDIR!("/home/rustacean");
    }

    #[test]
    fn arg() {
        let _ = ARG!("key" => "value");
    }

    #[test]
    fn stop_signal() {
        let _ = STOPSIGNAL!("SIGKILL");
    }

    #[test]
    fn health_check() {
        let _ = HEALTHCHECK!(NONE);
        let _ = HEALTHCHECK!(CMD vec!["curl", "-v", "https://rust-lang.org"]);
    }

    #[test]
    fn shell() {
        let _ = SHELL!["/bin/bash", "-c"];
    }

    #[test]
    fn on_build() {
        let on_build = ONBUILD!(CMD!["echo", "Hello, world!"]);
        assert_eq!(
            on_build.to_string(),
            r#"ONBUILD CMD ["echo", "Hello, world!"]"#
        );
    }

    #[test]
    fn comment() {
        let _ = COMMENT!("Hello, world!");
    }
}
