use crate::lexer::{TokenIter, Token, JsonKind};

#[test]
fn test_general_json() {
	let data = "{
	\"_id\": \"63f28771280d1dd3925d29ce\",
	\"index\": 3,
	\"guid\": \"07fc61f4-bfe7-4ec0-a9d2-413f9f906fd2\",
	\"isActive\": true,
	\"latitude\": -64.431585,
	\"tags\": [
		\"sit\",
		\"aute\",
		\"ea\"
	],
	\"friends\": [
		{
			\"id\": 0,
			\"name\": \"Araceli Shaffer\"
		},
		{
			\"id\": 1,
			\"name\": \"Glass Hancock\"
		}
	]
}";

let test_data = vec![
    Token::new(JsonKind::BeginObject, 1, "{".to_string()),
    Token::new(JsonKind::LineFeed, 1, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 2, "\t".to_string()),
    Token::new(JsonKind::StringVal, 2, "\"_id\"".to_string()),
    Token::new(JsonKind::NameSeperator, 2, ":".to_string()),
    Token::new(JsonKind::Space, 2, " ".to_string()),
    Token::new(JsonKind::StringVal, 2, "\"63f28771280d1dd3925d29ce\"".to_string()),
    Token::new(JsonKind::ValueSeperator, 2, ",".to_string()),
    Token::new(JsonKind::LineFeed, 2, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 3, "\t".to_string()),
    Token::new(JsonKind::StringVal, 3, "\"index\"".to_string()),
    Token::new(JsonKind::NameSeperator, 3, ":".to_string()),
    Token::new(JsonKind::Space, 3, " ".to_string()),
    Token::new(JsonKind::Digit, 3, "3".to_string()),
    Token::new(JsonKind::ValueSeperator, 3, ",".to_string()),
    Token::new(JsonKind::LineFeed, 3, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 4, "\t".to_string()),
    Token::new(JsonKind::StringVal, 4, "\"guid\"".to_string()),
    Token::new(JsonKind::NameSeperator, 4, ":".to_string()),
    Token::new(JsonKind::Space, 4, " ".to_string()),
    Token::new(JsonKind::StringVal, 4, "\"07fc61f4-bfe7-4ec0-a9d2-413f9f906fd2\"".to_string()),
    Token::new(JsonKind::ValueSeperator, 4, ",".to_string()),
    Token::new(JsonKind::LineFeed, 4, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 5, "\t".to_string()),
    Token::new(JsonKind::StringVal, 5, "\"isActive\"".to_string()),
    Token::new(JsonKind::NameSeperator, 5, ":".to_string()),
    Token::new(JsonKind::Space, 5, " ".to_string()),
    Token::new(JsonKind::True, 5, "true".to_string()),
    Token::new(JsonKind::ValueSeperator, 5, ",".to_string()),
    Token::new(JsonKind::LineFeed, 5, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 6, "\t".to_string()),
    Token::new(JsonKind::StringVal, 6, "\"latitude\"".to_string()),
    Token::new(JsonKind::NameSeperator, 6, ":".to_string()),
    Token::new(JsonKind::Space, 6, " ".to_string()),
    Token::new(JsonKind::Minus, 6, "-".to_string()),
    Token::new(JsonKind::Digit, 6, "6".to_string()),
    Token::new(JsonKind::Digit, 6, "4".to_string()),
    Token::new(JsonKind::DecimalPoint, 6, ".".to_string()),
    Token::new(JsonKind::Digit, 6, "4".to_string()),
    Token::new(JsonKind::Digit, 6, "3".to_string()),
    Token::new(JsonKind::Digit, 6, "1".to_string()),
    Token::new(JsonKind::Digit, 6, "5".to_string()),
    Token::new(JsonKind::Digit, 6, "8".to_string()),
    Token::new(JsonKind::Digit, 6, "5".to_string()),
    Token::new(JsonKind::ValueSeperator, 6, ",".to_string()),
    Token::new(JsonKind::LineFeed, 6, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 7, "\t".to_string()),
    Token::new(JsonKind::StringVal, 7, "\"tags\"".to_string()),
    Token::new(JsonKind::NameSeperator, 7, ":".to_string()),
    Token::new(JsonKind::Space, 7, " ".to_string()),
    Token::new(JsonKind::BeginArray, 7, "[".to_string()),
    Token::new(JsonKind::LineFeed, 7, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 8, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 8, "\t".to_string()),
    Token::new(JsonKind::StringVal, 8, "\"sit\"".to_string()),
    Token::new(JsonKind::ValueSeperator, 8, ",".to_string()),
    Token::new(JsonKind::LineFeed, 8, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 9, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 9, "\t".to_string()),
    Token::new(JsonKind::StringVal, 9, "\"aute\"".to_string()),
    Token::new(JsonKind::ValueSeperator, 9, ",".to_string()),
    Token::new(JsonKind::LineFeed, 9, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 10, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 10, "\t".to_string()),
    Token::new(JsonKind::StringVal, 10, "\"ea\"".to_string()),
    Token::new(JsonKind::LineFeed, 10, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 11, "\t".to_string()),
    Token::new(JsonKind::EndArray, 11, "]".to_string()),
    Token::new(JsonKind::ValueSeperator, 11, ",".to_string()),
    Token::new(JsonKind::LineFeed, 11, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 12, "\t".to_string()),
    Token::new(JsonKind::StringVal, 12, "\"friends\"".to_string()),
    Token::new(JsonKind::NameSeperator, 12, ":".to_string()),
    Token::new(JsonKind::Space, 12, " ".to_string()),
    Token::new(JsonKind::BeginArray, 12, "[".to_string()),
    Token::new(JsonKind::LineFeed, 12, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 13, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 13, "\t".to_string()),
    Token::new(JsonKind::BeginObject, 13, "{".to_string()),
    Token::new(JsonKind::LineFeed, 13, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 14, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 14, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 14, "\t".to_string()),
    Token::new(JsonKind::StringVal, 14, "\"id\"".to_string()),
    Token::new(JsonKind::NameSeperator, 14, ":".to_string()),
    Token::new(JsonKind::Space, 14, " ".to_string()),
    Token::new(JsonKind::Zero, 14, "0".to_string()),
    Token::new(JsonKind::ValueSeperator, 14, ",".to_string()),
    Token::new(JsonKind::LineFeed, 14, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 15, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 15, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 15, "\t".to_string()),
    Token::new(JsonKind::StringVal, 15, "\"name\"".to_string()),
    Token::new(JsonKind::NameSeperator, 15, ":".to_string()),
    Token::new(JsonKind::Space, 15, " ".to_string()),
    Token::new(JsonKind::StringVal, 15, "\"Araceli Shaffer\"".to_string()),
    Token::new(JsonKind::LineFeed, 15, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 16, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 16, "\t".to_string()),
    Token::new(JsonKind::EndObject, 16, "}".to_string()),
    Token::new(JsonKind::ValueSeperator, 16, ",".to_string()),
    Token::new(JsonKind::LineFeed, 16, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 17, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 17, "\t".to_string()),
    Token::new(JsonKind::BeginObject, 17, "{".to_string()),
    Token::new(JsonKind::LineFeed, 17, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 18, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 18, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 18, "\t".to_string()),
    Token::new(JsonKind::StringVal, 18, "\"id\"".to_string()),
    Token::new(JsonKind::NameSeperator, 18, ":".to_string()),
    Token::new(JsonKind::Space, 18, " ".to_string()),
    Token::new(JsonKind::Digit, 18, "1".to_string()),
    Token::new(JsonKind::ValueSeperator, 18, ",".to_string()),
    Token::new(JsonKind::LineFeed, 18, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 19, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 19, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 19, "\t".to_string()),
    Token::new(JsonKind::StringVal, 19, "\"name\"".to_string()),
    Token::new(JsonKind::NameSeperator, 19, ":".to_string()),
    Token::new(JsonKind::Space, 19, " ".to_string()),
    Token::new(JsonKind::StringVal, 19, "\"Glass Hancock\"".to_string()),
    Token::new(JsonKind::LineFeed, 19, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 20, "\t".to_string()),
    Token::new(JsonKind::HorizontalTab, 20, "\t".to_string()),
    Token::new(JsonKind::EndObject, 20, "}".to_string()),
    Token::new(JsonKind::LineFeed, 20, "\n".to_string()),

    Token::new(JsonKind::HorizontalTab, 21, "\t".to_string()),
    Token::new(JsonKind::EndArray, 21, "]".to_string()),
    Token::new(JsonKind::LineFeed, 21, "\n".to_string()),

    Token::new(JsonKind::EndObject, 22, "}".to_string()),
    ];

let iter = TokenIter { chars: data.chars().peekable(), curr_line: 1 };
let mut collect: Vec<Token> = Vec::new();

for val in iter {
    collect.push(val);
}

assert_eq!(collect.len(), test_data.len(), "generated token data length does not match static test token data length");

for (d, td) in collect.iter().zip(test_data.iter()) {
    assert_eq!(d.kind, td.kind, "token kind is incorrect");
    assert_eq!(d.text, td.text, "token text is incorrect");
    assert_eq!(d.line, td.line, "token line number is incorrect");
}
}

#[test]
fn test_strings_pass() {
    let data_table = vec![
"\"Yo, does this string work?\"",
"\"Yo, does this \\\" string work?\"",
"{
    \"Yo, does this string work?\"
}",
"[
    \"Yo, does this string work?\"
]",
    ];

let expected_table = vec![
    vec![
        Token { kind: JsonKind::StringVal, line: 1, text: "\"Yo, does this string work?\"".to_string() }, 
    ],
    vec![
        Token { kind: JsonKind::StringVal, line: 1, text: "\"Yo, does this \\\" string work?\"".to_string() }, 
    ],
    vec![
        Token { kind: JsonKind::BeginObject, line: 1, text: "{".to_string() }, 
        Token { kind: JsonKind::LineFeed, line: 1, text: "\n".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::StringVal, line: 2, text: "\"Yo, does this string work?\"".to_string() }, 
        Token { kind: JsonKind::LineFeed, line: 2, text: "\n".to_string() }, 
        Token { kind: JsonKind::EndObject, line: 3, text: "}".to_string() }, 
    ],
    vec![
        Token { kind: JsonKind::BeginArray, line: 1, text: "[".to_string() }, 
        Token { kind: JsonKind::LineFeed, line: 1, text: "\n".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::Space, line: 2, text: " ".to_string() }, 
        Token { kind: JsonKind::StringVal, line: 2, text: "\"Yo, does this string work?\"".to_string() }, 
        Token { kind: JsonKind::LineFeed, line: 2, text: "\n".to_string() }, 
        Token { kind: JsonKind::EndArray, line: 3, text: "]".to_string() }, 
    ],
    ];

for (i, data) in data_table.iter().enumerate() {

    let iter = TokenIter { chars: data.chars().peekable(), curr_line: 1 };
    let mut collect: Vec<Token> = Vec::new();
    let expected = &expected_table[i];

    for val in iter {
        collect.push(val);
    };

    assert_eq!(collect.len(), expected.len(), "generated token data length does not match static test token data length");

    for (d, td) in collect.iter().zip(expected.iter()) {
        assert_eq!(d.kind, td.kind, "token kind is incorrect");
        assert_eq!(d.text, td.text, "toke text is incorrect");
    }
}
}

#[test]
#[should_panic(expected = "Nonterminating string found")]
fn test_string_nonterm_fail() {
    let data_table = vec![
        "\"Yo, does this string not work?"
    ];

    let iter = TokenIter { chars: data_table[0].chars().peekable(), curr_line: 1 };
    let mut collect: Vec<Token> = Vec::new();

    for val in iter {
        collect.push(val);
    };
}

#[test]
#[should_panic(expected = "Multiline string detected")]
fn test_string_newline_fail() {
    let data_table = vec![
        "\"Yo, does this string not work?
        "
    ];

    let iter = TokenIter { chars: data_table[0].chars().peekable(), curr_line: 1 };
    let mut collect: Vec<Token> = Vec::new();

    for val in iter {
        collect.push(val);
    };
}

#[test]
fn test_keywords_pass() {
    let data_table = vec![
        "true",
        "false",
        "null"
    ];

    let expected_table = vec![
        vec![
            Token::new(JsonKind::True, 1, "true".to_string()), 
        ],
        vec![
            Token::new(JsonKind::False, 1, "false".to_string())
        ],
        vec![
            Token::new(JsonKind::Null, 1, "null".to_string())
        ],
    ];

        for (i, data) in data_table.iter().enumerate() {
            let iter = TokenIter { chars: data.chars().peekable(), curr_line: 1 };
            let mut collect: Vec<Token> = Vec::new();
            let expected = &expected_table[i];

            for val in iter {
                collect.push(val);
            };

            assert_eq!(collect.len(), expected.len(), "generated token data length does not match static test token data length");

            for (d, td) in collect.iter().zip(expected.iter()) {
                assert_eq!(d.kind, td.kind, "token kind is incorrect");
            }
        }
}

#[test]
#[should_panic(expected = "Unknown keyword detected")]
fn test_keyword_fails() {
    let data_table = vec![
        "woooop"
    ];

    let expected_table = vec![
        Token::new(JsonKind::Zero, 1, "woooop".to_string())
    ];

    let iter = TokenIter { chars: data_table[0].chars().peekable(), curr_line: 1 };
    let mut collect: Vec<Token> = Vec::new();
    let expected = expected_table;

    for val in iter {
        collect.push(val);
    };

    assert_eq!(collect.len(), expected.len(), "generated token data length does not match static test token data length");

    for (d, td) in collect.iter().zip(expected.iter()) {
        assert_eq!(d.kind, td.kind, "token kind is incorrect");
    }
}
