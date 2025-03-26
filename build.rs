use build_print::{error, info, warn};
use std::{
    error::Error,
    fs::read_to_string,
    num::ParseIntError,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

// The environmental variables that are usually set by R. These might be needed
// to set manually if we compile libR-sys outside of an R session.
// c.f., https://stat.ethz.ch/R-manual/R-devel/library/base/html/EnvVar.html
const ENVVAR_R_HOME: &str = "R_HOME";
const ENVVAR_R_INCLUDE_DIR: &str = "R_INCLUDE_DIR";

struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(3, ".").collect::<Vec<_>>();
        Ok(Self {
            major: parts[0].parse::<u8>()?,
            minor: parts[1].parse::<u8>()?,
            patch: parts[2].parse::<u8>()?,
        })
    }
}

fn read_r_ver(path: &Path) -> Result<Version, Box<dyn Error>> {
    let ver_h = path.join("Rversion.h");
    info!("Attempting to read version from {}", ver_h.display());
    let content = read_to_string(ver_h).map_err(|e| {
        error!("Unable to read Rversion.h: {}", e.to_string());
        e
    })?;

    let major = content
        .lines()
        .find(|line| line.starts_with("#define R_MAJOR"))
        .and_then(|line| line.split_whitespace().nth(2))
        .and_then(|v| v.trim_matches('"').to_string().into())
        .unwrap_or_default();

    let minor = content
        .lines()
        .find(|line| line.starts_with("#define R_MINOR"))
        .and_then(|line| line.split_whitespace().nth(2))
        .and_then(|v| v.trim_matches('"').to_string().into())
        .unwrap_or_default();

    let version = Version::from_str(&format!("{major}.{minor}")).map_err(|e| {
        error!("Unable to parse R version: {}", e.to_string());
        e
    })?;

    info!(
        "R version found is: {}.{}.{}",
        version.major, version.minor, version.patch
    );

    Ok(version)
}

impl Version {
    fn try_new() -> Result<Self, Box<dyn Error>> {
        let include_dir = match std::env::var(ENVVAR_R_INCLUDE_DIR) {
            Ok(v) => v,
            Err(_) => {
                warn!("R_INCLUDE_DIR not found. Likely being built outside of R.");
                let r_cmd = match cfg!(windows) {
                    true => "R.exe",
                    false => "R",
                };

                let mut cmd = Command::new(r_cmd);
                // print the include dir from R
                cmd.args([
                    "--vanilla",
                    "-s",
                    "-e",
                    "cat(normalizePath(R.home('include')))",
                ]);

                info!("Running R command: {:?}", cmd);

                let out = cmd.output()?.stdout;

                let res = String::from_utf8(out).map_err(|e| {
                    error!("Unable to parse R output");
                    e
                })?;

                info!("R command output: {:?}", res);
                res
            }
        };

        let r_ver = read_r_ver(&Path::new(&include_dir))?;

        Ok(r_ver)
    }
}

struct InstallationPaths {
    r_home: PathBuf,
    version: Version,
}

impl InstallationPaths {
    fn try_new() -> Result<Self, Box<dyn Error>> {
        // If R_HOME is unset then we try and call R directly.
        let r_home = match std::env::var(ENVVAR_R_HOME) {
            Err(_) => {
                warn!("R_HOME not found. Trying to fetch from R directly.");
                let r_cmd = match cfg!(windows) {
                    true => "R.exe",
                    false => "R",
                };

                let mut cmd = Command::new(r_cmd);
                cmd.args(["-s", "-e", "cat(normalizePath(R.home()))"]);
                info!("Running R command: {cmd:?}");
                let res = String::from_utf8(cmd.output()?.stdout)?;
                info!("R_HOME found at {res}");
                res
            }
            Ok(v) => v,
        };

        Ok(InstallationPaths {
            r_home: Path::new(&r_home).to_path_buf(),
            version: Version::try_new()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-check-cfg=cfg(r_4_4)");
    println!("cargo:rustc-check-cfg=cfg(r_4_5)");

    // Fetch R_HOME and R version
    let r_paths = InstallationPaths::try_new()?;

    // Used by extendr-engine becomes DEP_R_R_HOME for clients
    println!("cargo:r_home={}", r_paths.r_home.display());
    println!("cargo::rustc-env=R_HOME={}", r_paths.r_home.display());
    // used by extendr-api
    println!("cargo:r_version_major={}", r_paths.version.major);
    println!("cargo:r_version_minor={}", r_paths.version.minor);
    println!("cargo:r_version_patch={}", r_paths.version.patch);

    let pkg_target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let libdir = match (cfg!(windows), pkg_target_arch.as_str()) {
        // For Windows
        (true, "x86_64") => Path::new(&r_paths.r_home).join("bin").join("x64"),
        (true, "x86") => Path::new(&r_paths.r_home).join("bin").join("i386"),
        (true, _) => panic!("Unknown architecture"),
        // For Unix-alike
        (false, _) => Path::new(&r_paths.r_home).join("lib"),
    };

    info!("R library paths determined to be at: {libdir:?}");

    if let Ok(r_library) = libdir.canonicalize() {
        println!("cargo:rustc-link-search={}", r_library.display());
    }

    println!("cargo:rustc-link-lib=dylib=R");

    // Set R version specfic config flags
    // use r_4_4 config
    if (r_paths.version.major, r_paths.version.minor) >= (4, 4) {
        println!("cargo:rustc-cfg=r_4_4")
    }

    // use r_4_5 config
    if (r_paths.version.major, r_paths.version.minor) >= (4, 5) {
        println!("cargo:rustc-cfg=r_4_5")
    }

    // Only re-run if the include directory changes
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");

    build_print::custom_println!(" ✅ Success:", green, "libR-sys has been built!");
    Ok(())
}
