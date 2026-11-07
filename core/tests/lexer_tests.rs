use fridge_core::lexer::lex;
use fridge_core::token::TokenKind;
use fridge_core::token::TokenKind::*;

fn kinds(src: &str) -> Vec<TokenKind> {
    let (tokens, errors) = lex(src);
    assert!(errors.is_empty(), "unexpected lex errors: {:?}", errors);
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn remember_with_text_literal() {
    assert_eq!(
        kinds("remember name as \"Hamsan\""),
        vec![Remember, Ident("name".into()), As, Str("Hamsan".into()), Eof]
    );
}

#[test]
fn integer_and_float_literals() {
    let k = kinds("remember x as 42\nremember y as 3.5");
    assert!(k.contains(&Int(42)));
    assert!(k.contains(&Float(3.5)));
}

#[test]
fn official_shortcut_aliases() {
    assert_eq!(kinds("else"), vec![Otherwise, Eof]);
    assert_eq!(kinds("loop"), vec![Repeat, Eof]);
    assert_eq!(kinds("print"), vec![Show, Eof]);
    assert_eq!(kinds("start"), vec![Starts, Eof]);
    assert_eq!(kinds("x becomes 5"), vec![Ident("x".into()), Becomes, Int(5), Eof]);
}

#[test]
fn noise_words_are_skipped() {
    assert_eq!(
        kinds("create a function called greet"),
        vec![Create, Function, Called, Ident("greet".into()), Eof]
    );
}

#[test]
fn comments_produce_no_tokens() {
    assert_eq!(kinds("// only a comment"), vec![Eof]);
    assert_eq!(kinds("show 1 /* mid */ plus? no"), vec![Show, Int(1), Ident("plus".into()) /*…*/].as_slice().to_vec().into_iter().take(3).collect::<Vec<_>>().into_iter().chain(vec![Eof]).collect::<Vec<_>>());
}
