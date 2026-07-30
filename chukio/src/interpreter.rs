use std::path::Path;

use anyhow::{Context, Result, bail};
use rustc_public::CompilerError;

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
    use rustc_hash::FxHashMap;
    use rustc_public::mir::mono::Instance;

    let entry = rustc_public::entry_fn().context("source does not have an entry function")?;
    let entry_instance =
        Instance::try_from(entry).context("failed to resolve the entry function")?;
    let entry_body = entry_instance
        .body()
        .context("failed to resolve the entry function body")?;

    let mut living_storage: FxHashMap<usize, Storage> = FxHashMap::default();
    let first_block = entry_body
        .blocks
        .first()
        .context("the first entry point block is none")?;

    // execute the first block
    execute_block(&first_block, &entry_body, &mut living_storage)?;

    Ok(())
}

/// execute MIR body
fn execute_block(
    block: &rustc_public::mir::BasicBlock,
    body: &rustc_public::mir::Body,
    living_storage: &mut rustc_hash::FxHashMap<usize, Storage>,
) -> anyhow::Result<()> {
    use rustc_public::mir::StatementKind;

    // execute statements
    for statement in block.statements.iter() {
        match &statement.kind {
            StatementKind::Assign(place, rvalue) => {
                println!("{rvalue:#?}");
                todo!()
            }
            StatementKind::FakeRead(fake_read_cause, place) => todo!(),
            StatementKind::SetDiscriminant {
                place,
                variant_index,
            } => todo!(),
            StatementKind::StorageLive(idx) => {
                if living_storage.contains_key(&idx) {
                    bail!("storage of the {idx}th is already living");
                }
                // get variable type
                let decl = body
                    .local_decl(*idx)
                    .context("local decl does not exists")?;
                assert!(
                    living_storage
                        .insert(*idx, Storage::new(&decl.ty.kind()))
                        .is_none()
                ); // should not contain
            }
            StatementKind::StorageDead(_) => todo!(),
            StatementKind::PlaceMention(place) => todo!(),
            StatementKind::AscribeUserType {
                place,
                projections,
                variance,
            } => todo!(),
            StatementKind::Coverage(opaque) => todo!(),
            StatementKind::Intrinsic(non_diverging_intrinsic) => todo!(),
            StatementKind::ConstEvalCounter => todo!(),
            StatementKind::Nop => todo!(),
        }
    }

    Ok(())
}

enum Storage {
    Bool(bool),
    I32(i32),
}

impl Storage {
    // im lazy
    fn new(ty_kind: &rustc_public::ty::TyKind) -> Self {
        use rustc_public::ty::{IntTy, RigidTy, TyKind};

        match ty_kind {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::Bool => todo!(),
                RigidTy::Char => todo!(),
                RigidTy::Int(int_ty) => match int_ty {
                    IntTy::Isize | IntTy::I32 => Self::I32(0),
                    IntTy::I8 => todo!(),
                    IntTy::I16 => todo!(),
                    IntTy::I64 => todo!(),
                    IntTy::I128 => todo!(),
                },
                RigidTy::Uint(uint_ty) => todo!(),
                RigidTy::Float(float_ty) => todo!(),
                RigidTy::Adt(adt_def, generic_args) => todo!(),
                RigidTy::Foreign(foreign_def) => todo!(),
                RigidTy::Str => todo!(),
                RigidTy::Array(ty, ty_const) => todo!(),
                RigidTy::Pat(ty, pattern) => todo!(),
                RigidTy::Slice(ty) => todo!(),
                RigidTy::RawPtr(ty, mutability) => todo!(),
                RigidTy::Ref(region, ty, mutability) => todo!(),
                RigidTy::FnDef(fn_def, generic_args) => todo!(),
                RigidTy::FnPtr(binder) => todo!(),
                RigidTy::Closure(closure_def, generic_args) => todo!(),
                RigidTy::Coroutine(coroutine_def, generic_args) => todo!(),
                RigidTy::CoroutineClosure(coroutine_closure_def, generic_args) => todo!(),
                RigidTy::Dynamic(binders, region) => todo!(),
                RigidTy::Never => todo!(),
                RigidTy::Tuple(items) => todo!(),
                RigidTy::CoroutineWitness(coroutine_witness_def, generic_args) => todo!(),
            },
            TyKind::Alias(alias_kind, alias_ty) => todo!(),
            TyKind::Param(param_ty) => todo!(),
            TyKind::Bound(_, bound_ty) => todo!(),
        }
    }
}
