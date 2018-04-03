use super::InferResult;
use types::{TyCon, Type};

pub fn cast_check(expr: &Type, to: &Type) -> InferResult<()> {
    match *to {
        Type::App(ref tycon, _) => {
            check_tycon(tycon)?;
        }
        _ => return Err(()),
    }
    match *expr {
        Type::App(ref tycon, _) => {
            check_tycon(tycon)?;
        }
        _ => return Err(()),
    }

    Ok(())
}

fn check_tycon(tycon: &TyCon) -> InferResult<()> {
    match *tycon {
        TyCon::Bool | TyCon::Char => Ok(()),
        TyCon::Int(_, _) => Ok(()),
        _ => Err(()),
    }
}
