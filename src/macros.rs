#![allow(non_snake_case)]

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

#[macro_export]
macro_rules! RUN {
    ($($x:expr), +) => {{
        use $crate::Run;
        Run::from(&[$($x), +])
    }};
}

#[macro_export]
macro_rules! CMD {
    ($($x:expr), +) => {{
        use $crate::Cmd;
        Cmd::from(&[$($x), +])
    }};
}

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

#[macro_export]
macro_rules! MAINTAINER {
    ($name:expr) => {{
        use $crate::Maintainer;
        Maintainer::from($name)
    }};
}

#[macro_export]
macro_rules! EXPOSE {
    ($port:tt/$proto:ident) => {{
        use $crate::Expose;
        Expose {
            port: $port,
            proto: stringify!($proto).to_string(),
        }
    }};
    ($port:expr) => {{
        use $crate::Expose;
        Expose::from($port)
    }};
}

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

#[macro_export]
macro_rules! ENTRYPOINT {
    ($($x:expr), +) => {{
        use $crate::EntryPoint;
        EntryPoint::from(&[$($x), +])
    }};
}

#[macro_export]
macro_rules! VOLUME {
    ($($x:expr), +) => {{
        use $crate::Volume;
        Volume::from(&[$($x), +])
    }};
}

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

#[macro_export]
macro_rules! WORKDIR {
    ($dir:expr) => {{
        use $crate::WorkDir;
        WorkDir::from($dir)
    }};
}

#[macro_export]
macro_rules! ARG {
    ($x:expr => $y:expr) => {{
        use $crate::Arg;
        Arg::from(($x, $y))
    }};
}

#[macro_export]
macro_rules! STOPSIGNAL {
    ($signal:expr) => {{
        use $crate::StopSignal;
        StopSignal::from($signal);
    }};
}

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

#[macro_export]
macro_rules! SHELL {
    ($($x:expr), +) => {{
        use $crate::Shell;
        Shell::from(&[$($x), +])
    }};
}

#[macro_export]
macro_rules! ONBUILD {
    ($x:expr) => {{
        use $crate::OnBuild;
        OnBuild::from($x)
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
        let _ = MAINTAINER!("Funny Rustcean");
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
        let _ = ADD!(--chown=rustcean:root "/var/run" "/home");
        let _ = ADD!(--chown=rustcean "/var/run" "/home");
        let _ = ADD!("/var/run" "/home");
    }

    #[test]
    fn copy() {
        let _ = COPY!(--from=crab --chown=rustcean:root "/var/run" "/home");
        let _ = COPY!(--from=crab --chown=rustcean "/var/run" "/home");
        let _ = COPY!(--chown=rustcean:root "/var/run" "/home");
        let _ = COPY!(--chown=rustcean "/var/run" "/home");
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
        let _ = USER!(rustcean: root);
        let _ = USER!(rustcean);
    }

    #[test]
    fn work_dir() {
        let _ = WORKDIR!("/home/rustcean");
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
        let _ = HEALTHCHECK!(CMD & ["curl", "-v", "https://rust-lang.org"]);
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
}
