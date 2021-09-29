use std::str::FromStr;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Arch {
    X86_64,
}

impl Default for Arch {
    fn default() -> Self {
        Self::X86_64
    }
}

impl FromStr for Arch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_64" => Ok(Self::X86_64),
            // TODO: Error handling
            _ => panic!("Invalid Arch: {}", s)
        }
    }
}

impl ToString for Arch {
    fn to_string(&self) -> String {
        match self {
            Self::X86_64 => "x86_64".to_string()
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Target {
    Bios(Arch),
}

impl Default for Target {
    fn default() -> Self {
        Self::Bios(Default::default())
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(arch_str) = s.split("bios-").nth(1) {
            let arch = Arch::from_str(arch_str)?;
            return Ok(Self::Bios(arch));
        }
        // TODO: Error handling
        panic!("Invalid Target: {}", s)
    }
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Self::Bios(arch) => format!("bios-{}", arch.to_string())
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BuildProfile {
    Debug,
    Release,
}

impl Default for BuildProfile {
    fn default() -> Self {
        Self::Debug
    }
}

impl ToString for BuildProfile {
    fn to_string(&self) -> String {
        match self {
            Self::Debug => "debug".to_string(),
            Self::Release => "release".to_string()
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OsBuilder {
    target: Target,
    profile: BuildProfile,
}

impl OsBuilder {
    pub fn target(&mut self, target: Target) -> &mut Self {
        self.target = target;
        self
    }

    pub fn profile(&mut self, profile: BuildProfile) -> &mut Self {
        self.profile = profile;
        self
    }

    fn workspace_dir(&self) -> PathBuf {
        const BUILDER_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
        let mut dir = PathBuf::from_str(BUILDER_MANIFEST_DIR).unwrap();
        dir.pop();
        dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut dir = self.workspace_dir();
        dir.push("build");
        dir
    }

    fn target_name(&self) -> &str {
        match self.target {
            Target::Bios(arch) => match arch {
                Arch::X86_64 => "x86_64-z_os"
            }
        }
    }

    fn boot_package(&self) -> &str {
        match self.target {
            Target::Bios(_) => "boot-bios"
        }
    }

    pub fn build(&self) {
        let target_name = self.target_name();
        let boot_package_name = self.boot_package();
        let build_command = match self.target {
            Target::Bios(_) => "bootimage"
        };

        // Run cargo build
        let build_exit_code = Command::new("cargo")
            .arg(build_command)
            .args(&["--target", format!("./{}/{}.json", boot_package_name, target_name).as_str()])
            .args(&["--package", boot_package_name])
            .current_dir(self.workspace_dir())
            // Clear env variables that might affect cargo or rustc
            .env("RUSTFLAGS", "")
            .env("CARGO_UNSTABLE_BUILD_STD", "core,compiler_builtins,alloc")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .code()
            .unwrap();
        if build_exit_code != 0 {
            panic!("Cargo build failed with code {}", build_exit_code);
        }

        let build_dir = self.build_dir();
        std::fs::remove_dir_all(&build_dir).ok();
        std::fs::create_dir_all(&build_dir).unwrap();

        match self.target {
            Target::Bios(_) => {
                // TODO
            }
        }
    }

    pub fn run(&self) {
        let mut command: Command;
        // qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

        let kernel_bin = match self.target {
            Target::Bios(_) => "bootimage-boot-bios.bin"
        };
        let kernel_bin = {
            let mut file = self.workspace_dir();
            file.push("target");
            file.push(self.target_name());
            file.push(self.profile.to_string());
            file.push(kernel_bin);
            file
        };

        match self.target {
            Target::Bios(arch) => {
                command = Command::new(format!("qemu-system-{}", arch.to_string()));
                command.current_dir(self.workspace_dir())
                    .args(&["-drive", format!("format=raw,file={}", kernel_bin.display()).as_str()])
                    .args(&["-serial", "stdio"]);
            }
        }

        let exit_code = command.spawn()
            .unwrap()
            .wait()
            .unwrap()
            .code()
            .unwrap();
        if exit_code != 0 {
            panic!("Qemu failed with code {}", exit_code);
        }
    }
}
