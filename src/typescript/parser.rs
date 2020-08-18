use std::sync::Arc;
use std::path::Path;
use swc::common::errors;
use swc::common::SourceMap;
use swc::common::FileName;
use swc::ecmascript::parser::{lexer::Lexer, Capturing, Parser, Session, SourceFileInput, Syntax};
use swc::ecmascript::ast::Script;

pub fn parse_file(filepath: &Path) -> (Script, Arc<SourceMap>) {
    let cm: Arc<SourceMap> = Default::default();
    let handler = errors::Handler::with_tty_emitter(errors::ColorConfig::Auto, true, false, Some(cm.clone()));
    let session = Session { handler: &handler };
    let fm = cm
        .load_file(&filepath)
        .expect(format!("failed to load file: {:?}", filepath.to_str().unwrap()).as_str());

    let syntax = Syntax::Typescript(Default::default());
    let lexer = Lexer::new(
        session,
        syntax,
        Default::default(),
        SourceFileInput::from(&*fm),
        None,
    );
    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(session, capturing);
    let script = parser
        .parse_script()
        .map_err(|mut e| {
            e.emit();
        })
        .expect("Failed to parse module.");

    (script, cm)
}


pub fn parse_code(code: &str) -> (Script, Arc<SourceMap>) {
    let cm: Arc<SourceMap> = Default::default();
    let handler = errors::Handler::with_tty_emitter(errors::ColorConfig::Auto, true, false, Some(cm.clone()));
    let session = Session { handler: &handler };
    let fm = cm.new_source_file(
            FileName::Custom("temp.js".into()),
            code.into(),
        );
    let syntax = Syntax::Typescript(Default::default());
    let lexer = Lexer::new(
        session,
        syntax,
        Default::default(),
        SourceFileInput::from(&*fm),
        None,
    );
    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(session, capturing);
    let script = parser
        .parse_script()
        .map_err(|mut e| {
            e.emit();
        })
        .expect("Failed to parse module.");

    (script, cm)
}
