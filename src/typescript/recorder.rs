use swc::common::Span;


struct Record {
    data: Vec<&RecordDatum>
}

struct RecordDatum {
    span: &Span,
    kind: &str,
    desc: &str
}
