Asteroid
================

## Usage

```sh
asteroid <filepath>
```

or
```sh
cargo run -- <filepath>
```


## OutPut Format

```rust
pub enum Level {
    Default,
    Warning,
    Critical
}

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
    pub source: Option<Source>,
    pub debug_info: Option<Value>,
    pub description: Option<String>
}


pub struct Reporter {
    pub data: HashMap<Kind, Vec<Datum>>,
}

```

A JSON Example


```json
{
  "data": {
    "FnCall": [
      {
        "span": {
          "start": 500,
          "end": 507,
          "ctxt": 0
        },
        "level": "Default",
        "content": {
          "call_type": "PureCallee",
          "func_name": "greeter"
        },
        "context": {
          "optional": false,
          "span": {
            "ctxt": 0,
            "end": 507,
            "start": 500
          },
          "type": "Identifier",
          "typeAnnotation": null,
          "value": "greeter"
        },
        "source": {
          "code": "greeter(user)",
          "lineno_lo": 27,
          "lineno_hi": 27
        },
        "debug_info": null,
        "description": null
      },
      {
        "span": {
          "start": 500,
          "end": 507,
          "ctxt": 0
        },
        "level": "Default",
        "content": {
          "call_type": "PureCallee",
          "func_name": "greeter"
        },
        "context": {
          "optional": false,
          "span": {
            "ctxt": 0,
            "end": 507,
            "start": 500
          },
          "type": "Identifier",
          "typeAnnotation": null,
          "value": "greeter"
        },
        "source": {
          "code": "greeter(user)",
          "lineno_lo": 27,
          "lineno_hi": 27
        },
        "debug_info": null,
        "description": null
      },
      {
        "span": {
          "start": 427,
          "end": 434,
          "ctxt": 0
        },
        "level": "Default",
        "content": {
          "call_type": "PureCallee",
          "func_name": "student"
        },
        "context": {
          "optional": false,
          "span": {
            "ctxt": 0,
            "end": 434,
            "start": 427
          },
          "type": "Identifier",
          "typeAnnotation": null,
          "value": "Student"
        },
        "source": {
          "code": "new Student(\"Jane\", \"M.\", \"User\")",
          "lineno_lo": 24,
          "lineno_hi": 24
        },
        "debug_info": null,
        "description": null
      }
    ],
    "MethodCall": [
      {
        "span": {
          "start": 458,
          "end": 462,
          "ctxt": 0
        },
        "level": "Default",
        "content": {
          "call_type": "MemberCallee",
          "func_name": "user.test"
        },
        "context": {
          "computed": false,
          "object": {
            "optional": false,
            "span": {
              "ctxt": 0,
              "end": 462,
              "start": 458
            },
            "type": "Identifier",
            "typeAnnotation": null,
            "value": "user"
          },
          "property": {
            "optional": false,
            "span": {
              "ctxt": 0,
              "end": 467,
              "start": 463
            },
            "type": "Identifier",
            "typeAnnotation": null,
            "value": "test"
          },
          "span": {
            "ctxt": 0,
            "end": 467,
            "start": 458
          },
          "type": "MemberExpression"
        },
        "source": {
          "code": "user.test()",
          "lineno_lo": 25,
          "lineno_hi": 25
        },
        "debug_info": null,
        "description": null
      },
      {
        "span": {
          "start": 458,
          "end": 462,
          "ctxt": 0
        },
        "level": "Default",
        "content": {
          "call_type": "MemberCallee",
          "func_name": "user.test"
        },
        "context": {
          "computed": false,
          "object": {
            "optional": false,
            "span": {
              "ctxt": 0,
              "end": 462,
              "start": 458
            },
            "type": "Identifier",
            "typeAnnotation": null,
            "value": "user"
          },
          "property": {
            "optional": false,
            "span": {
              "ctxt": 0,
              "end": 467,
              "start": 463
            },
            "type": "Identifier",
            "typeAnnotation": null,
            "value": "test"
          },
          "span": {
            "ctxt": 0,
            "end": 467,
            "start": 458
          },
          "type": "MemberExpression"
        },
        "source": {
          "code": "user.test()",
          "lineno_lo": 25,
          "lineno_hi": 25
        },
        "debug_info": null,
        "description": null
      }
    ]
  }
}
```
