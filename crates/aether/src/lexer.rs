#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String), Int(i64), Float(f64), Str(String),
    Let, Mut, Fn, If, Else, While, For, In, Loop, Break, Continue, Return, True, False,
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Bang, BangEq, Lt, Le, Gt, Ge, AndAnd, OrOr,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Semicolon, Arrow,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token { pub kind: TokenKind, pub line: usize, pub column: usize }

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect(); let mut out=Vec::new(); let mut i=0; let mut line=1; let mut col=1;
    while i<chars.len(){let c=chars[i]; if c.is_whitespace(){if c=='\n'{line+=1;col=1}else{col+=1}i+=1;continue} if c=='/'&&chars.get(i+1)==Some(&'/'){while i<chars.len()&&chars[i]!='\n'{i+=1;col+=1}continue}
        let sl=line;let sc=col;let kind=match c{
            '('=>{i+=1;col+=1;TokenKind::LParen},')'=>{i+=1;col+=1;TokenKind::RParen},'{'=>{i+=1;col+=1;TokenKind::LBrace},'}'=>{i+=1;col+=1;TokenKind::RBrace},
            '['=>{i+=1;col+=1;TokenKind::LBracket},']'=>{i+=1;col+=1;TokenKind::RBracket},','=>{i+=1;col+=1;TokenKind::Comma},';'=>{i+=1;col+=1;TokenKind::Semicolon},
            '+'=>{i+=1;col+=1;TokenKind::Plus},'*'=>{i+=1;col+=1;TokenKind::Star},'%'=>{i+=1;col+=1;TokenKind::Percent},
            '-'=>{i+=1;col+=1;if chars.get(i)==Some(&'>'){i+=1;col+=1;TokenKind::Arrow}else{TokenKind::Minus}},
            '/'=>{i+=1;col+=1;TokenKind::Slash},
            '='=>{i+=1;col+=1;if chars.get(i)==Some(&'='){i+=1;col+=1;TokenKind::EqEq}else{TokenKind::Eq}},
            '!'=>{i+=1;col+=1;if chars.get(i)==Some(&'='){i+=1;col+=1;TokenKind::BangEq}else{TokenKind::Bang}},
            '<'=>{i+=1;col+=1;if chars.get(i)==Some(&'='){i+=1;col+=1;TokenKind::Le}else{TokenKind::Lt}},
            '>'=>{i+=1;col+=1;if chars.get(i)==Some(&'='){i+=1;col+=1;TokenKind::Ge}else{TokenKind::Gt}},
            '&'=>{i+=1;col+=1;if chars.get(i)==Some(&'&'){i+=1;col+=1;TokenKind::AndAnd}else{return Err(format!("unexpected '&' at {sl}:{sc}"))}},
            '|'=>{if chars.get(i+1)==Some(&'|'){i+=2;col+=2;TokenKind::OrOr}else{return Err(format!("unexpected '|' at {sl}:{sc}"))}},
            '"'=>{i+=1;col+=1;let mut s=String::new();while i<chars.len()&&chars[i]!='"'{if chars[i]=='\\'{i+=1;col+=1;if i>=chars.len(){return Err("unterminated escape".into())}s.push(match chars[i]{'n'=>'\n','r'=>'\r','t'=>'\t','"'=>'"','\\'=>'\\',x=>x})}else{s.push(chars[i])}i+=1;col+=1}if i>=chars.len(){return Err(format!("unterminated string at {sl}:{sc}"))}i+=1;col+=1;TokenKind::Str(s)},
            c if c.is_ascii_digit()=>{let b=i;let mut dot=false;while i<chars.len()&&(chars[i].is_ascii_digit()||(!dot&&chars[i]=='.')){if chars[i]=='.'{dot=true}i+=1;col+=1}let text:String=chars[b..i].iter().collect();if dot{TokenKind::Float(text.parse().map_err(|_|format!("invalid float at {sl}:{sc}"))?)}else{TokenKind::Int(text.parse().map_err(|_|format!("invalid integer at {sl}:{sc}"))?)}},
            c if c.is_ascii_alphabetic()||c=='_'=>{let b=i;while i<chars.len()&&(chars[i].is_ascii_alphanumeric()||chars[i]=='_'){i+=1;col+=1}match chars[b..i].iter().collect::<String>().as_str(){"let"=>TokenKind::Let,"mut"=>TokenKind::Mut,"fn"=>TokenKind::Fn,"if"=>TokenKind::If,"else"=>TokenKind::Else,"while"=>TokenKind::While,"for"=>TokenKind::For,"in"=>TokenKind::In,"loop"=>TokenKind::Loop,"break"=>TokenKind::Break,"continue"=>TokenKind::Continue,"return"=>TokenKind::Return,"true"=>TokenKind::True,"false"=>TokenKind::False,x=>TokenKind::Ident(x.into())}},
            _=>return Err(format!("unexpected '{c}' at {sl}:{sc}")),
        };out.push(Token{kind,line:sl,column:sc})}
    out.push(Token{kind:TokenKind::Eof,line,column:col});Ok(out)
}
