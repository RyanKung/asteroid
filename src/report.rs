use swc::common::Span;
use swc::common::SourceMap;
use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::sync::Arc;


#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum Level {
    Default,
    Warning,
    Critical
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum Kind {
    FnCall,
    MethodCall,
    SyntaxError,
    Super
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    pub code: String,
    pub lineno_lo: usize,
    pub lineno_hi: usize
}

impl Source {
    pub fn new(span: &Span, cm: Arc<SourceMap>) -> Option<Self> {
        let code = cm.span_to_snippet(*span).ok();
        if let Some(c) = code {
            let loc_lo = cm.lookup_char_pos(span.lo()).line;
            let loc_hi = cm.lookup_char_pos(span.hi()).line;
            Some(Self {
                code: c,
                lineno_lo: loc_lo,
                lineno_hi: loc_hi
            })
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Datum {
    pub span: Option<Span>,
    pub level: Level,
    pub content: Value,
    pub context: Value,
    pub source: Option<Source>,
    pub debug_info: Option<Value>,
    pub description: Option<String>
}

type ReportData = HashMap<Kind, Vec<Datum>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Reporter {
    pub data: ReportData
}


impl Reporter {
    pub fn new() -> Self {
        let data: ReportData = HashMap::new();
        Reporter {
            data: data
        }
    }
    pub fn report(
        &mut self,
        kind: Kind,
        span: Option<Span>,
        level: Level,
        context: Value,
        content: Value,
        source: Option<Source>,
        debug: Option<Value>,
        desc: Option<String>
    ) {
        let datum: Datum = Datum {
            span: span,
            level: level,
            content: content,
            context: context,
            source: source,
            debug_info: debug,
            description: desc
        };
        self.data.entry(kind)
            .or_insert_with(|| vec![datum.clone()])
            .push(datum);
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
