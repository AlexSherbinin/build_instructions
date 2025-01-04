use std::{
    convert::Infallible,
    env::VarError,
    path::{Path, PathBuf},
};

macro_rules! define_env_getter {
    ($($(#[$meta: meta])* $name: ident: $result: ty => $env: literal;)*) => {
        $(
            $(#[$meta])*
            pub fn $name() -> Result<$result, std::env::VarError> {
                std::env::var($env).map(Into::into)
            }
        )*
    };
}

/// The `Cargo` struct serves as a utility for interacting with Cargo-specific environment variables and commands.
pub struct Cargo(Infallible);

impl Cargo {
    /// Specifies to Cargo that a build script should be re-run if the specified file changes.
    pub fn rerun_if_changed(path: impl AsRef<Path>) {
        let path = path.as_ref().display();
        println!("cargo::rerun-if-changed={path}");
    }

    /// Specifies to Cargo that a build script should be re-run if the specified environment variable changes.
    pub fn rerun_if_env_changed(env: impl AsRef<str>) {
        let env = env.as_ref();
        println!("cargo::rerun-if-env-changed={env}");
    }

    /// Prints a warning message during the build process.
    pub fn warning(message: impl AsRef<str>) {
        let message = message.as_ref();
        println!("cargo::warning={message}");
    }

    /// Sets metadata that can be accessed by downstream tools or build scripts.
    pub fn metadata(key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = key.as_ref();
        let value = value.as_ref();
        println!("cargo::metadata={key}={value}");
    }

    /// Fetches the path to the binary executable for a specified binary name from the environment variables.
    pub fn binary_executable_path(binary_name: impl AsRef<str>) -> Result<PathBuf, VarError> {
        let binary_name = binary_name.as_ref();
        std::env::var(format!("CARGO_BIN_EXE_{binary_name}")).map(Into::into)
    }

    /// Checks if the package being built is the primary package.
    pub fn is_primary_package() -> bool {
        std::env::var("CARGO_PRIMARY_PACKAGE").is_ok()
    }

    define_env_getter! {
        /// Path to the `cargo` binary performing the build
        binary_path: PathBuf => "CARGO";
        /// The directory containing the manifest of your package
        manifest_dir: PathBuf => "CARGO_MANIFEST_DIR";
        /// The path to the manifest of your package
        manifest_path: PathBuf => "CARGO_MANIFEST_PATH";
        /// The full version of your package
        pkg_version: String => "CARGO_PKG_VERSION";
        /// The major version of your package
        pkg_version_major: String => "CARGO_PKG_VERSION_MAJOR";
        /// The minor version of your package
        pkg_version_minor: String => "CARGO_PKG_VERSION_MINOR";
        /// The patch version of your package
        pkg_version_patch: String => "CARGO_PKG_VERSION_PATCH";
        /// The pre-release version of your package
        pkg_version_pre: String => "CARGO_PKG_VERSION_PRE";
        /// Colon separated list of authors from the manifest of your package
        pkg_authors: String => "CARGO_PKG_AUTHORS";
        /// The name of your package
        pkg_name: String => "CARGO_PKG_NAME";
        /// The description from the manifest of your package
        pkg_description: String => "CARGO_PKG_DESCRIPTION";
        /// The home page from the manifest of your package
        pkg_homepage: String => "CARGO_PKG_HOMEPAGE";
        /// The repository from the manifest of your package
        pkg_repository: String => "CARGO_PKG_REPOSITORY";
        /// The license from the manifest of your package
        pkg_license: String => "CARGO_PKG_LICENSE";
        /// The license file from the manifest of your package
        pkg_license_file: PathBuf => "CARGO_PKG_LICENSE_FILE";
        /// The Rust version from the manifest of your package.
        /// Note that this is the minimum Rust version supported by the package, not the current Rust version
        pkg_rust_version: String => "CARGO_PKG_RUST_VERSION";
        /// Path to the README file of your package
        pkg_readme: PathBuf => "CARGO_PKG_README";
        /// The name of the crate that is currently being compiled.
        /// It is the name of the Cargo target with - converted to _,
        /// such as the name of the library, binary, example, integration test, or benchmark
        crate_name: String => "CARGO_CRATE_NAME";
        /// The name of the binary that is currently being compiled.
        /// Only set for binaries or binary examples.
        /// This name does not include any file extension, such as .exe
        bin_name: String => "CARGO_BIN_NAME";
        /// If the package has a build script, this is set to the folder where the build script should place its output.
        /// See below for more information. (Only set during compilation.)
        out_dir: PathBuf => "OUT_DIR";
        /// Only set when building integration test or benchmark code.
        /// This is a path to a directory inside the target directory
        /// where integration tests or benchmarks are free to put any data needed by the tests/benches.
        /// Cargo initially creates this directory but doesn’t manage its content in any way,
        /// this is the responsibility of the test code
        target_tmpdir: PathBuf => "CARGO_TARGET_TMPDIR";
        /// This is a path that rustc is invoked from (nightly only)
        rustc_current_dir: PathBuf => "CARGO_RUSTC_CURRENT_DIR";
    }
}
