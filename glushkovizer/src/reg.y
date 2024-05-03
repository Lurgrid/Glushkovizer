%start Expr
%avoid_insert "SYM"
%avoid_insert "EPSILON"
%left '+'
%left '.'
%left '*'
%%
Expr -> Result<RegExp<char>, ()>:
    Expr '*' { Ok(RegExp::Repeat(Box::new($1?))) }
    | Expr '+' Expr { Ok(RegExp::Or(Box::new($1?), Box::new($3?))) }
    | Expr '.' Expr {  Ok(RegExp::Concat(Box::new($1?), Box::new($3?))) }
    | '(' Expr ')' { Ok($2?) } 
    | 'EPSILON' { Ok(RegExp::Epsilon) }
    | 'SYM' { 
        let v = $1.map_err(|_| ())?;
        Ok(RegExp::Symbol($lexer.span_str(v.span()).chars().next().ok_or(())?))       
    } 
    ;
%%
use crate::regexp::RegExp;