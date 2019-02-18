use crate::{
    Add, Arg, Cmd, Copy, EntryPoint, Env, Expose, From, HealthCheck, Label, Maintainer, OnBuild,
    Run, Shell, StopSignal, StorageInstruction, User, Volume, WorkDir,
};
use std::fmt::{self, Display};

pub struct DockerFile {
    from: From,
    maintainer: Option<Maintainer>,
    entry_point: Option<EntryPoint>,
    cmd: Option<Cmd>,
    instructions: Vec<Box<StorageInstruction>>,
    on_builds: Vec<OnBuild>,
}

impl DockerFile {
    pub fn from(from: From) -> Self {
        Self {
            from,
            maintainer: None,
            entry_point: None,
            cmd: None,
            instructions: Vec::new(),
            on_builds: Vec::new(),
        }
    }

    pub fn maintainer<T: Into<Maintainer> + 'static>(mut self, maintainer: T) -> Self {
        self.maintainer = Some(maintainer.into());
        self
    }

    fn instruction<T: StorageInstruction + 'static>(mut self, t: T) -> Self {
        self.instructions.push(Box::new(t));
        self
    }

    pub fn entry_point<T: Into<EntryPoint> + 'static>(mut self, entry_point: T) -> Self {
        self.entry_point = Some(entry_point.into());
        self
    }

    pub fn cmd<T: Into<Cmd> + 'static>(mut self, cmd: T) -> Self {
        self.cmd = Some(cmd.into());
        self
    }

    pub fn run<T: Into<Run> + 'static>(self, run: T) -> Self {
        self.instruction(run.into())
    }

    pub fn label<T: Into<Label> + 'static>(self, label: T) -> Self {
        self.instruction(label.into())
    }

    pub fn expose<T: Into<Expose> + 'static>(self, expose: T) -> Self {
        self.instruction(expose.into())
    }

    pub fn env<T: Into<Env> + 'static>(self, env: T) -> Self {
        self.instruction(env.into())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, add: Add) -> Self {
        self.instruction(add)
    }

    pub fn copy(self, copy: Copy) -> Self {
        self.instruction(copy)
    }

    pub fn volume<T: Into<Volume> + 'static>(self, volume: T) -> Self {
        self.instruction(volume.into())
    }

    pub fn user(self, user: User) -> Self {
        self.instruction(user)
    }

    pub fn work_dir<T: Into<WorkDir> + 'static>(self, work_dir: T) -> Self {
        self.instruction(work_dir.into())
    }

    pub fn arg<T: Into<Arg> + 'static>(self, arg: T) -> Self {
        self.instruction(arg.into())
    }

    pub fn stop_signal<T: Into<StopSignal> + 'static>(self, stop_signal: T) -> Self {
        self.instruction(stop_signal.into())
    }

    pub fn health_check(self, health_check: HealthCheck) -> Self {
        self.instruction(health_check)
    }

    pub fn shell<T: Into<Shell> + 'static>(self, shell: T) -> Self {
        self.instruction(shell.into())
    }

    pub fn on_build<T: Into<OnBuild> + 'static>(mut self, on_build: T) -> Self {
        self.on_builds.push(on_build.into());
        self
    }
}

impl Display for DockerFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.from)?;

        if let Some(maintainer) = &self.maintainer {
            writeln!(f)?;
            writeln!(f, "{}", maintainer)?;
        }

        if !self.instructions.is_empty() {
            writeln!(f)?;
            for instruction in &self.instructions {
                writeln!(f, "{}", instruction)?;
            }
        }

        if !self.on_builds.is_empty() {
            writeln!(f)?;
            for on_build in &self.on_builds {
                writeln!(f, "{}", on_build)?;
            }
        }

        match (&self.entry_point, &self.cmd) {
            (Some(entry_point), Some(cmd)) => {
                writeln!(f)?;
                writeln!(f, "{}", entry_point)?;
                writeln!(f, "{}", cmd)?;
            }
            (Some(entry_point), None) => {
                writeln!(f)?;
                writeln!(f, "{}", entry_point)?;
            }
            (None, Some(cmd)) => {
                writeln!(f)?;
                writeln!(f, "{}", cmd)?;
            }
            (None, None) => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tag;

    #[test]
    fn builder() {
        let content = DockerFile::from(From {
            image: String::from("rust"),
            tag_or_digest: Some(Tag("latest".to_string())),
            name: None,
        })
        .maintainer("lead rustcean")
        .run(&["/bin/bash", "-c", "echo"])
        .label(("key", "value"))
        .expose(80)
        .env(("RUST", "1.0.0"))
        .add(Add {
            src: "/var/run".to_string(),
            dst: "/home".to_string(),
            chown: None,
        })
        .copy(Copy {
            src: "/var/run".to_string(),
            dst: "/home".to_string(),
            from: None,
            chown: None,
        })
        .volume(&["/var/run", "/var/www"])
        .user(User {
            user: "rustcean".to_string(),
            group: None,
        })
        .work_dir("/home/rustcean")
        .arg(("build", "yes"))
        .stop_signal("SIGKILL")
        .health_check(HealthCheck::None)
        .shell(&["/bin/bash", "-c"])
        .on_build(OnBuild::from(Cmd::from(&[
            "echo",
            "This is the ONBUILD command",
        ])))
        .entry_point(&["cargo", "check"])
        .cmd(&["echo", "Hi!"])
        .to_string();
        assert_eq!(
            content,
            r#"FROM rust:latest

MAINTAINER lead rustcean

RUN ["/bin/bash", "-c", "echo"]
LABEL key="value"
EXPOSE 80/tcp
ENV RUST="1.0.0"
ADD "/var/run" "/home"
COPY "/var/run" "/home"
VOLUME ["/var/run", "/var/www"]
USER rustcean
WORKDIR "/home/rustcean"
ARG build="yes"
STOPSIGNAL SIGKILL
HEALTHCHECK NONE
SHELL ["/bin/bash", "-c"]

ONBUILD CMD ["echo", "This is the ONBUILD command"]

ENTRYPOINT ["cargo", "check"]
CMD ["echo", "Hi!"]
"#
        );
    }
}
