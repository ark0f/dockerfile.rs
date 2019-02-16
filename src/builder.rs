use crate::{
    Add, Arg, Cmd, Copy, EntryPoint, Env, Expose, From, HealthCheck, Instruction, Label,
    Maintainer, OnBuild, Run, Shell, StopSignal, User, Volume, WorkDir,
};
use std::io;

pub struct DockerFile {
    from: From,
    maintainer: Option<Maintainer>,
    instructions: Vec<Box<Instruction>>,
    on_build: Vec<OnBuild>,
}

impl DockerFile {
    pub fn from(from: From) -> Self {
        Self {
            from,
            maintainer: None,
            instructions: Vec::new(),
            on_build: Vec::new(),
        }
    }

    pub fn maintainer<T: Into<Maintainer> + 'static>(mut self, maintainer: T) -> Self {
        self.maintainer = Some(maintainer.into());
        self
    }

    pub fn instruction<T: Instruction + 'static>(mut self, t: T) -> Self {
        self.instructions.push(Box::new(t));
        self
    }

    pub fn run<T: Into<Run> + 'static>(self, run: T) -> Self {
        self.instruction(run.into())
    }

    pub fn cmd<T: Into<Cmd> + 'static>(self, cmd: T) -> Self {
        self.instruction(cmd.into())
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

    pub fn entry_point<T: Into<EntryPoint> + 'static>(self, entry_point: T) -> Self {
        self.instruction(entry_point.into())
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
        self.on_build.push(on_build.into());
        self
    }

    pub fn write_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{}", self.from)?;
        writeln!(w)?;

        if let Some(maintainer) = &self.maintainer {
            writeln!(w, "{}", maintainer)?;
            writeln!(w)?;
        }

        for instruction in &self.instructions {
            writeln!(w, "{}", instruction)?;
        }

        Ok(())
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tag;
    use std::fs::File;

    #[test]
    fn builder() {
        let mut file = File::create("test.Dockerfile").unwrap();
        Dockerfile::from(From {
            image: String::from("rust"),
            tag_or_digest: Some(Tag("lastest".to_string())),
            name: None,
        })
        .maintainer("lead rustcean")
        .run(&["/bin/bash", "-c", "echo"])
        .cmd(&["echo", "Hi!"])
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
            chown: None,
        })
        .entry_point(&["cargo", "check"])
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
        .write_to(&mut file);
    }
}*/
