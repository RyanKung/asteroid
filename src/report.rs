use swc::common::Span;
use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};

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
pub struct Datum {
    pub span: Option<Span>,
    pub level: Level,
    pub content: Value,
    pub context: Value,
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
        debug: Option<Value>,
        desc: Option<String>
    ) {
        let datum: Datum = Datum {
            span: span,
            level: level,
            content: content,
            context: context,
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
