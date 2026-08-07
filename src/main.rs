use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::str::FromStr;
use colored::Colorize;

fn info(msg: &str) { println!("{} {}", "[i]".bright_cyan(), msg); }
fn success(msg: &str) { println!("{} {}", "[✓]".bright_green(), msg.bright_green()); }
fn error(msg: &str) { eprintln!("{} {}", "[✗]".bright_red(), msg.bright_red()); }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    Ext2,
    Ext3,
    Ext4,
    Btrfs,
    Fat16,
    Fat32,
    Xfs,
    F2fs,
    Swap,
}

impl FromStr for FsType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "ext2" => Ok(FsType::Ext2),
            "ext3" => Ok(FsType::Ext3),
            "ext4" => Ok(FsType::Ext4),
            "btrfs" => Ok(FsType::Btrfs),
            "fat16" | "vfat16" => Ok(FsType::Fat16),
            "fat" | "fat32" | "vfat" => Ok(FsType::Fat32),
            "xfs" => Ok(FsType::Xfs),
            "f2fs" => Ok(FsType::F2fs),
            "swap" => Ok(FsType::Swap),
            _ => Err(format!("Unsupported filesystem type: '{}'", s)),
        }
    }
}

pub struct Lkfs {
    fs_type: FsType,
    device: PathBuf,
    label: Option<String>,
    force: bool,
}

impl Lkfs {
    pub fn new(fs_type: FsType, device: impl AsRef<Path>) -> Self {
        Self {
            fs_type,
            device: device.as_ref().to_path_buf(),
            label: None,
            force: false,
        }
    }
    pub fn execute(&self) -> Result<(), String> {
        if !self.device.exists() {
            return Err(format!("Device '{:?}' does not exist!", self.device));
        }
        let (binary, mut args) = match self.fs_type {
            FsType::Ext2 | FsType::Ext3 | FsType::Ext4 => {
                let fs_str = match self.fs_type {
                    FsType::Ext2 => "ext2",
                    FsType::Ext3 => "ext3",
                    FsType::Ext4 => "ext4",
                    _ => unreachable!(),
                };
                let mut a = vec!["-t".to_string(), fs_str.to_string()];
                if self.force { a.push("-F".to_string()); }
                if let Some(ref l) = self.label {
                    a.push("-L".to_string());
                    a.push(l.clone());
                }
                ("mke2fs", a)
            }
            FsType::Btrfs => {
                let mut a = Vec::new();
                if self.force { a.push("-f".to_string()); }
                if let Some(ref l) = self.label {
                    a.push("-L".to_string());
                    a.push(l.clone());
                }
                ("mkfs.btrfs", a)
            }
            FsType::Fat16 | FsType::Fat32 => {
                let mut a = Vec::new();
                let size_str = if self.fs_type == FsType::Fat16 { "16" } else { "32" };
                a.push("-F".to_string());
                a.push(size_str.to_string());
                if let Some(ref l) = self.label {
                    a.push("-n".to_string());
                    a.push(l.clone());
                }
                ("mkfs.fat", a)
            }
            FsType::Xfs => {
                let mut a = Vec::new();
                if self.force { a.push("-f".to_string()); }
                if let Some(ref l) = self.label {
                    a.push("-L".to_string());
                    a.push(l.clone());
                }
                ("mkfs.xfs", a)
            }
            FsType::F2fs => {
                let mut a = Vec::new();
                if self.force { a.push("-f".to_string()); }
                if let Some(ref l) = self.label {
                    a.push("-l".to_string());
                    a.push(l.clone());
                }
                ("mkfs.f2fs", a)
            }
            FsType::Swap => {
                let mut a = Vec::new();
                if self.force { a.push("-f".to_string()); }
                if let Some(ref l) = self.label {
                    a.push("-L".to_string());
                    a.push(l.clone());
                }
                ("mkswap", a)
            }
        };
        args.push(self.device.to_string_lossy().to_string());
        info("Starting the operation....");
        info(&format!("Selected filesystem: {} {}", binary, args.join(" ")));
        let status = Command::new(binary)
            .args(&args)
            .status()
            .map_err(|e| format!("Failed to run '{}'. Is the dependencies installed? Error message: {}", binary, e))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("Formatting failed with exit code: {:?}", status.code()))
        }
    }
}

fn print_usage() {
    println!("");
    println!("-----------------------------------------");
    println!("::: [ Liska Filesystem Tool (1.0.0) ] :::");
    println!("-----------------------------------------");
    println!("");
    println!("Usage: lkfs <fstype> [partition] [command]");
    println!("> -l | --label <name>    set filesystem label");
    println!("> -f | --force           force formatting filesystem (use with caution)");
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_usage();
        exit(0);
    }
    let fs_type = match FsType::from_str(&args[1]) {
        Ok(t) => t,
        Err(e) => {
            error(&e);
            exit(1);
        }
    };
    let device = &args[2];
    let mut lkfs = Lkfs::new(fs_type, device);
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--label" => {
                if i + 1 < args.len() {
                    lkfs.label = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-f" | "--force" => {
                lkfs.force = true;
            }
            _ => {}
        }
        i += 1;
    }
    info(&format!("Formatting {} as {:?}...", device, fs_type));
    match lkfs.execute() {
        Ok(_) => success("Operation completed successfully!"),
        Err(e) => {
            error(&e);
            exit(1);
        }
    }
}