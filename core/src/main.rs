use std::env;
use std::fs;
use std::process::ExitCode;

use fridge_core::lexer::lex;

const VERSION: &str = "FRIDGE 0.1.0-alpha (language spec v0.2.1)";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("version");

    match cmd {
        "version" => { println!("{VERSION}"); ExitCode::SUCCESS }
        "lex" => lex_file(args.get(1), false),
        // M2 scope: lexical diagnostics only. Parser diagnostics arrive in M3.
        "check" => lex_file(args.get(1), args.iter().any(|a| a == "--output")),
        other => {
            eprintln!("fridge: unknown command '{other}'");
            eprintln!("available: version | lex <file.frg> | check <file.frg> [--output json]");
            ExitCode::from(2)
        }
    }
}

fn lex_file(path: Option<&String>, json: bool) -> ExitCode {
    let Some(path) = path else {
        eprintln!("usage: fridge <lex|check> <file.frg>");
        return ExitCode::from(64);
    };
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: FRG-IO-001\nFile: {path}\n\nCannot read file: {e}");
            return ExitCode::FAILURE;
        }
    };
    let (tokens, errors) = lex(&src);

    if json {
        println!("{{\"diagnostics\":[");
        let mut first = true;
        for e in &errors {
            if !first { println!(","); }
            first = false;
            print!(
                "{{\"file\":\"{}\",\"line\":{},\"column\":{},\"severity\":\"error\",\"code\":\"{}\",\"message\":\"{}\",\"explanation\":\"\",\"suggestion\":\"\",\"fixes\":[]}}",
                escape(path), e.line, e.col, e.code, escape(&e.message)
            );
        }
        println!("]}}");
    } else if errors.is_empty() {
        for t in &tokens {
            println!("{:>4}:{:<3} {:?}", t.line, t.col, t.kind);
        }
    } else {
        for e in &errors {
            println!("ERROR: {}\nFile: {}\nLine: {}\n\n{}\n", e.code, path, e.line, e.message);
        }
    }

    if errors.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}
