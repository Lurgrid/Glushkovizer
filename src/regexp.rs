use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("reg.l");
lrpar_mod!("reg.y");

#[derive(Debug)]
pub enum RegExp<T> {
    Epsilon,
    Symbol(T),
    Times(Box<RegExp<T>>),
    Concat(Box<RegExp<T>>, Box<RegExp<T>>),
    Or(Box<RegExp<T>>, Box<RegExp<T>>),
}

impl TryFrom<&str> for RegExp<char> {
    type Error = String;

    fn try_from(regexp: &str) -> Result<RegExp<char>, Self::Error> {
        let lexerdef = reg_l::lexerdef();
        let lexer = lexerdef.lexer(regexp);
        let (res, errs) = reg_y::parse(&lexer);
        let mut err = String::new();
        for e in &errs {
            err.push_str(&format!("{}\n", e.pp(&lexer, &reg_y::token_epp)));
        }
        match res {
            Some(Ok(r)) if errs.is_empty() => Ok(r),
            _ => {
                err.push_str("Unable to evaluate expression.");
                Err(err)
            }
        }
    }
}
