use std::path::Path;

use anyhow::{Context, Result, bail};
use rustc_public::CompilerError;
use rustc_public::mir::mono::Instance;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Compiles a Rust source file and interprets its entry function.
    pub fn interpret(&self, source_path: impl AsRef<Path>) -> Result<()> {
        let source_path = source_path.as_ref();
        let source_path = source_path
            .canonicalize()
            .with_context(|| format!("failed to resolve source path: {}", source_path.display()))?;

        if !source_path.is_file() {
            bail!("source path is not a file: {}", source_path.display());
        }

        let source_arg = source_path
            .to_str()
            .context("source path must be valid UTF-8")?
            .to_owned();

        let rustc_args = vec![
            "chukio-rustc".to_owned(),
            "--crate-name=chukio_input".to_owned(),
            "--edition=2024".to_owned(),
            "-Copt-level=0".to_owned(),
            "-Zmir-opt-level=0".to_owned(),
            "-Coverflow-checks=yes".to_owned(),
            source_arg,
        ];

        let compiler_result = rustc_public::run!(&rustc_args, || {
            std::ops::ControlFlow::<Result<()>, ()>::Break(run_internal())
        });

        match compiler_result {
            Err(CompilerError::Interrupted(result)) => result,
            Err(CompilerError::Failed) => {
                bail!("rustc failed to compile {}", source_path.display())
            }
            Err(CompilerError::Skipped) => {
                bail!("rustc skipped compiling {}", source_path.display())
            }
            Ok(()) => bail!("rustc completed without interpreting MIR"),
        }
    }
}

/// Resolves the entry function and runs it in a new machine.
fn run_internal() -> Result<()> {
    let entry = rustc_public::entry_fn().context("source does not have an entry function")?;
    let _entry_instance =
        Instance::try_from(entry).context("failed to resolve the entry function")?;

    Ok(())
}
