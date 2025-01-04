use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    path::Path,
};

/// Represents the different kinds of link search paths used by the Rust compiler.
pub enum LinkSearchKind {
    /// Only search for transitive dependencies in this directory
    Dependency,
    /// Only search for this crate's direct dependencies in this directory
    Crate,
    /// Only search for native libraries in this directory
    Native,
    /// Only search for macOS frameworks in this directory
    Framework,
    /// Search for all library kinds in this directory, except frameworks.
    /// This is the default if `kind` is not specified
    All,
}

impl Display for LinkSearchKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LinkSearchKind::Dependency => "dependency",
            LinkSearchKind::Crate => "crate",
            LinkSearchKind::Native => "native",
            LinkSearchKind::Framework => "framework",
            LinkSearchKind::All => "all",
        };

        write!(f, "{s}")
    }
}

/// Provides utilities for interacting with the Rust compiler through Cargo build instructions.
pub struct Rustc(Infallible);

impl Rustc {
    /// Passes a single linker argument to the Rust compiler.
    pub fn link_arg(flag: impl AsRef<str>) {
        let flag = flag.as_ref();
        println!("cargo::rustc-link-arg={flag}");
    }

    /// Passes a linker argument for a specific binary target.
    pub fn link_arg_bin(bin: impl AsRef<str>, flag: impl AsRef<str>) {
        let bin = bin.as_ref();
        let flag = flag.as_ref();
        println!("cargo::rustc-link-arg-bin={bin}={flag}");
    }

    /// Passes a linker argument for all binary targets.
    pub fn link_arg_bins(flag: impl AsRef<str>) {
        let flag = flag.as_ref();
        println!("cargo::rustc-link-arg-bins={flag}");
    }

    /// Links a library with the specified name.
    pub fn link_lib(lib: impl AsRef<str>) {
        let lib = lib.as_ref();
        println!("cargo::rustc-link-lib={lib}");
    }

    /// Passes a linker argument specifically for test builds.
    pub fn link_arg_tests(flag: impl AsRef<str>) {
        let flag = flag.as_ref();
        println!("cargo::rustc-link-arg-tests={flag}");
    }

    /// Passes a linker argument specifically for example builds.
    pub fn link_arg_examples(flag: impl AsRef<str>) {
        let flag = flag.as_ref();
        println!("cargo::rustc-link-arg-examples={flag}");
    }

    /// Specifies a directory for the Rust compiler to search for libraries.
    pub fn link_search(path: impl AsRef<Path>, kind: impl Into<Option<LinkSearchKind>>) {
        let path = path.as_ref().display();
        let kind = kind.into();

        match kind {
            Some(kind) => println!("cargo::rustc-link-search={kind}={path}"),
            None => println!("carg::rustc-link-search={path}"),
        }
    }

    /// Passes additional compiler flags to Rust compiler.
    pub fn flags(flags: impl AsRef<str>) {
        let flags = flags.as_ref();
        println!("cargo::rustc-flags={flags}");
    }

    /// Configures a conditional compilation flag with an optional value.
    pub fn cfg<'a>(key: impl AsRef<str>, value: impl Into<Option<&'a str>>) {
        let key = key.as_ref();
        let value = value.into();

        match value {
            Some(value) => println!("cargo::rustc-cfg={key}=\"{value}\""),
            None => println!("cargo::rustc-cfg={key}"),
        }
    }

    /// Checks the validity of a conditional compilation flag.
    pub fn check_cfg(cfg: impl AsRef<str>) {
        let cfg = cfg.as_ref();
        println!("cargo::rustc-check-cfg={cfg}");
    }

    /// Sets an environment variable for the build script.
    pub fn env(var: impl AsRef<str>, value: impl AsRef<str>) {
        let var = var.as_ref();
        let value = value.as_ref();
        println!("cargo::rustc-env={var}={value}");
    }

    /// Passes a linker argument specifically for `cdylib` builds.
    pub fn cdylib_link_arg(flag: impl AsRef<str>) {
        let flag = flag.as_ref();
        println!("cargo::rustc-cdylib-link-arg={flag}");
    }
}
