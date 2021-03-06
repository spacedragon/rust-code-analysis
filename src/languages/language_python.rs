// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum Python {
    End = 0,
    Identifier = 1,
    Import = 2,
    DOT = 3,
    From = 4,
    Future = 5,
    LPAREN = 6,
    RPAREN = 7,
    COMMA = 8,
    As = 9,
    STAR = 10,
    Print = 11,
    GTGT = 12,
    Assert = 13,
    COLONEQ = 14,
    Return = 15,
    Del = 16,
    Raise = 17,
    Pass = 18,
    Break = 19,
    Continue = 20,
    If = 21,
    COLON = 22,
    Elif = 23,
    Else = 24,
    Async = 25,
    For = 26,
    In = 27,
    While = 28,
    Try = 29,
    Except = 30,
    Finally = 31,
    With = 32,
    Def = 33,
    DASHGT = 34,
    EQ = 35,
    STARSTAR = 36,
    Global = 37,
    Nonlocal = 38,
    Exec = 39,
    Class = 40,
    AT = 41,
    Not = 42,
    And = 43,
    Or = 44,
    PLUS = 45,
    DASH = 46,
    SLASH = 47,
    PERCENT = 48,
    SLASHSLASH = 49,
    PIPE = 50,
    AMP = 51,
    CARET = 52,
    LTLT = 53,
    TILDE = 54,
    LT = 55,
    LTEQ = 56,
    EQEQ = 57,
    BANGEQ = 58,
    GTEQ = 59,
    GT = 60,
    LTGT = 61,
    Is = 62,
    Lambda3 = 63,
    PLUSEQ = 64,
    DASHEQ = 65,
    STAREQ = 66,
    SLASHEQ = 67,
    ATEQ = 68,
    SLASHSLASHEQ = 69,
    PERCENTEQ = 70,
    STARSTAREQ = 71,
    GTGTEQ = 72,
    LTLTEQ = 73,
    AMPEQ = 74,
    CARETEQ = 75,
    PIPEEQ = 76,
    Yield2 = 77,
    LBRACK = 78,
    RBRACK = 79,
    Ellipsis = 80,
    LBRACE = 81,
    RBRACE = 82,
    EscapeSequence = 83,
    NotEscapeSequence = 84,
    FormatSpecifierToken1 = 85,
    TypeConversion = 86,
    Integer = 87,
    Float = 88,
    True = 89,
    False = 90,
    None = 91,
    Await2 = 92,
    Comment = 93,
    Semicolon = 94,
    Newline = 95,
    Indent = 96,
    Dedent = 97,
    DQUOTE = 98,
    StringContent = 99,
    DQUOTE2 = 100,
    Module = 101,
    Statement = 102,
    SimpleStatements = 103,
    ImportStatement = 104,
    ImportPrefix = 105,
    RelativeImport = 106,
    FutureImportStatement = 107,
    ImportFromStatement = 108,
    ImportList = 109,
    AliasedImport = 110,
    WildcardImport = 111,
    PrintStatement = 112,
    Chevron = 113,
    AssertStatement = 114,
    ExpressionStatement = 115,
    NamedExpression = 116,
    ReturnStatement = 117,
    DeleteStatement = 118,
    RaiseStatement = 119,
    PassStatement = 120,
    BreakStatement = 121,
    ContinueStatement = 122,
    IfStatement = 123,
    ElifClause = 124,
    ElseClause = 125,
    ForStatement = 126,
    WhileStatement = 127,
    TryStatement = 128,
    ExceptClause = 129,
    FinallyClause = 130,
    WithStatement = 131,
    WithItem = 132,
    FunctionDefinition = 133,
    Parameters = 134,
    LambdaParameters = 135,
    Parameters2 = 136,
    DefaultParameter = 137,
    TypedDefaultParameter = 138,
    ListSplat = 139,
    DictionarySplat = 140,
    GlobalStatement = 141,
    NonlocalStatement = 142,
    ExecStatement = 143,
    ClassDefinition = 144,
    ParenthesizedListSplat = 145,
    ArgumentList = 146,
    DecoratedDefinition = 147,
    Decorator = 148,
    Block = 149,
    Variables = 150,
    ExpressionList = 151,
    DottedName = 152,
    ExpressionWithinForInClause = 153,
    Expression = 154,
    PrimaryExpression = 155,
    NotOperator = 156,
    BooleanOperator = 157,
    BinaryOperator = 158,
    UnaryOperator = 159,
    ComparisonOperator = 160,
    Lambda = 161,
    Lambda2 = 162,
    Assignment = 163,
    AugmentedAssignment = 164,
    RightHandSide = 165,
    Yield = 166,
    Attribute = 167,
    Subscript = 168,
    Slice = 169,
    Call = 170,
    TypedParameter = 171,
    Type = 172,
    KeywordArgument = 173,
    List = 174,
    ComprehensionClauses = 175,
    ListComprehension = 176,
    Dictionary = 177,
    DictionaryComprehension = 178,
    Pair = 179,
    Set = 180,
    SetComprehension = 181,
    ParenthesizedExpression = 182,
    CollectionElements = 183,
    Tuple = 184,
    GeneratorExpression = 185,
    ForInClause = 186,
    IfClause = 187,
    ConditionalExpression = 188,
    ConcatenatedString = 189,
    String = 190,
    Interpolation = 191,
    FormatSpecifier = 192,
    FormatExpression = 193,
    Await = 194,
    ModuleRepeat1 = 195,
    SimpleStatementsRepeat1 = 196,
    ImportPrefixRepeat1 = 197,
    ImportListRepeat1 = 198,
    PrintStatementRepeat1 = 199,
    AssertStatementRepeat1 = 200,
    IfStatementRepeat1 = 201,
    TryStatementRepeat1 = 202,
    WithStatementRepeat1 = 203,
    ParametersRepeat1 = 204,
    GlobalStatementRepeat1 = 205,
    ArgumentListRepeat1 = 206,
    DecoratedDefinitionRepeat1 = 207,
    VariablesRepeat1 = 208,
    DottedNameRepeat1 = 209,
    ComparisonOperatorRepeat1 = 210,
    SubscriptRepeat1 = 211,
    ComprehensionClausesRepeat1 = 212,
    DictionaryRepeat1 = 213,
    SetRepeat1 = 214,
    CollectionElementsRepeat1 = 215,
    ForInClauseRepeat1 = 216,
    ConcatenatedStringRepeat1 = 217,
    StringRepeat1 = 218,
    FormatSpecifierRepeat1 = 219,
    Error = 220,
}

impl Into<&'static str> for Python {
    fn into(self) -> &'static str {
        match self {
            Python::End => "end",
            Python::Identifier => "identifier",
            Python::Import => "import",
            Python::DOT => ".",
            Python::From => "from",
            Python::Future => "__future__",
            Python::LPAREN => "(",
            Python::RPAREN => ")",
            Python::COMMA => ",",
            Python::As => "as",
            Python::STAR => "*",
            Python::Print => "print",
            Python::GTGT => ">>",
            Python::Assert => "assert",
            Python::COLONEQ => ":=",
            Python::Return => "return",
            Python::Del => "del",
            Python::Raise => "raise",
            Python::Pass => "pass",
            Python::Break => "break",
            Python::Continue => "continue",
            Python::If => "if",
            Python::COLON => ":",
            Python::Elif => "elif",
            Python::Else => "else",
            Python::Async => "async",
            Python::For => "for",
            Python::In => "in",
            Python::While => "while",
            Python::Try => "try",
            Python::Except => "except",
            Python::Finally => "finally",
            Python::With => "with",
            Python::Def => "def",
            Python::DASHGT => "->",
            Python::EQ => "=",
            Python::STARSTAR => "**",
            Python::Global => "global",
            Python::Nonlocal => "nonlocal",
            Python::Exec => "exec",
            Python::Class => "class",
            Python::AT => "@",
            Python::Not => "not",
            Python::And => "and",
            Python::Or => "or",
            Python::PLUS => "+",
            Python::DASH => "-",
            Python::SLASH => "/",
            Python::PERCENT => "%",
            Python::SLASHSLASH => "//",
            Python::PIPE => "|",
            Python::AMP => "&",
            Python::CARET => "^",
            Python::LTLT => "<<",
            Python::TILDE => "~",
            Python::LT => "<",
            Python::LTEQ => "<=",
            Python::EQEQ => "==",
            Python::BANGEQ => "!=",
            Python::GTEQ => ">=",
            Python::GT => ">",
            Python::LTGT => "<>",
            Python::Is => "is",
            Python::Lambda3 => "lambda",
            Python::PLUSEQ => "+=",
            Python::DASHEQ => "-=",
            Python::STAREQ => "*=",
            Python::SLASHEQ => "/=",
            Python::ATEQ => "@=",
            Python::SLASHSLASHEQ => "//=",
            Python::PERCENTEQ => "%=",
            Python::STARSTAREQ => "**=",
            Python::GTGTEQ => ">>=",
            Python::LTLTEQ => "<<=",
            Python::AMPEQ => "&=",
            Python::CARETEQ => "^=",
            Python::PIPEEQ => "|=",
            Python::Yield2 => "yield",
            Python::LBRACK => "[",
            Python::RBRACK => "]",
            Python::Ellipsis => "ellipsis",
            Python::LBRACE => "{",
            Python::RBRACE => "}",
            Python::EscapeSequence => "escape_sequence",
            Python::NotEscapeSequence => "_not_escape_sequence",
            Python::FormatSpecifierToken1 => "format_specifier_token1",
            Python::TypeConversion => "type_conversion",
            Python::Integer => "integer",
            Python::Float => "float",
            Python::True => "true",
            Python::False => "false",
            Python::None => "none",
            Python::Await2 => "await",
            Python::Comment => "comment",
            Python::Semicolon => "_semicolon",
            Python::Newline => "_newline",
            Python::Indent => "_indent",
            Python::Dedent => "_dedent",
            Python::DQUOTE => "\"",
            Python::StringContent => "_string_content",
            Python::DQUOTE2 => "\"",
            Python::Module => "module",
            Python::Statement => "_statement",
            Python::SimpleStatements => "_simple_statements",
            Python::ImportStatement => "import_statement",
            Python::ImportPrefix => "import_prefix",
            Python::RelativeImport => "relative_import",
            Python::FutureImportStatement => "future_import_statement",
            Python::ImportFromStatement => "import_from_statement",
            Python::ImportList => "_import_list",
            Python::AliasedImport => "aliased_import",
            Python::WildcardImport => "wildcard_import",
            Python::PrintStatement => "print_statement",
            Python::Chevron => "chevron",
            Python::AssertStatement => "assert_statement",
            Python::ExpressionStatement => "expression_statement",
            Python::NamedExpression => "named_expression",
            Python::ReturnStatement => "return_statement",
            Python::DeleteStatement => "delete_statement",
            Python::RaiseStatement => "raise_statement",
            Python::PassStatement => "pass_statement",
            Python::BreakStatement => "break_statement",
            Python::ContinueStatement => "continue_statement",
            Python::IfStatement => "if_statement",
            Python::ElifClause => "elif_clause",
            Python::ElseClause => "else_clause",
            Python::ForStatement => "for_statement",
            Python::WhileStatement => "while_statement",
            Python::TryStatement => "try_statement",
            Python::ExceptClause => "except_clause",
            Python::FinallyClause => "finally_clause",
            Python::WithStatement => "with_statement",
            Python::WithItem => "with_item",
            Python::FunctionDefinition => "function_definition",
            Python::Parameters => "parameters",
            Python::LambdaParameters => "lambda_parameters",
            Python::Parameters2 => "_parameters",
            Python::DefaultParameter => "default_parameter",
            Python::TypedDefaultParameter => "typed_default_parameter",
            Python::ListSplat => "list_splat",
            Python::DictionarySplat => "dictionary_splat",
            Python::GlobalStatement => "global_statement",
            Python::NonlocalStatement => "nonlocal_statement",
            Python::ExecStatement => "exec_statement",
            Python::ClassDefinition => "class_definition",
            Python::ParenthesizedListSplat => "parenthesized_list_splat",
            Python::ArgumentList => "argument_list",
            Python::DecoratedDefinition => "decorated_definition",
            Python::Decorator => "decorator",
            Python::Block => "block",
            Python::Variables => "variables",
            Python::ExpressionList => "expression_list",
            Python::DottedName => "dotted_name",
            Python::ExpressionWithinForInClause => "_expression_within_for_in_clause",
            Python::Expression => "_expression",
            Python::PrimaryExpression => "_primary_expression",
            Python::NotOperator => "not_operator",
            Python::BooleanOperator => "boolean_operator",
            Python::BinaryOperator => "binary_operator",
            Python::UnaryOperator => "unary_operator",
            Python::ComparisonOperator => "comparison_operator",
            Python::Lambda => "lambda",
            Python::Lambda2 => "lambda",
            Python::Assignment => "assignment",
            Python::AugmentedAssignment => "augmented_assignment",
            Python::RightHandSide => "_right_hand_side",
            Python::Yield => "yield",
            Python::Attribute => "attribute",
            Python::Subscript => "subscript",
            Python::Slice => "slice",
            Python::Call => "call",
            Python::TypedParameter => "typed_parameter",
            Python::Type => "type",
            Python::KeywordArgument => "keyword_argument",
            Python::List => "list",
            Python::ComprehensionClauses => "_comprehension_clauses",
            Python::ListComprehension => "list_comprehension",
            Python::Dictionary => "dictionary",
            Python::DictionaryComprehension => "dictionary_comprehension",
            Python::Pair => "pair",
            Python::Set => "set",
            Python::SetComprehension => "set_comprehension",
            Python::ParenthesizedExpression => "parenthesized_expression",
            Python::CollectionElements => "_collection_elements",
            Python::Tuple => "tuple",
            Python::GeneratorExpression => "generator_expression",
            Python::ForInClause => "for_in_clause",
            Python::IfClause => "if_clause",
            Python::ConditionalExpression => "conditional_expression",
            Python::ConcatenatedString => "concatenated_string",
            Python::String => "string",
            Python::Interpolation => "interpolation",
            Python::FormatSpecifier => "format_specifier",
            Python::FormatExpression => "format_expression",
            Python::Await => "await",
            Python::ModuleRepeat1 => "module_repeat1",
            Python::SimpleStatementsRepeat1 => "_simple_statements_repeat1",
            Python::ImportPrefixRepeat1 => "import_prefix_repeat1",
            Python::ImportListRepeat1 => "_import_list_repeat1",
            Python::PrintStatementRepeat1 => "print_statement_repeat1",
            Python::AssertStatementRepeat1 => "assert_statement_repeat1",
            Python::IfStatementRepeat1 => "if_statement_repeat1",
            Python::TryStatementRepeat1 => "try_statement_repeat1",
            Python::WithStatementRepeat1 => "with_statement_repeat1",
            Python::ParametersRepeat1 => "_parameters_repeat1",
            Python::GlobalStatementRepeat1 => "global_statement_repeat1",
            Python::ArgumentListRepeat1 => "argument_list_repeat1",
            Python::DecoratedDefinitionRepeat1 => "decorated_definition_repeat1",
            Python::VariablesRepeat1 => "variables_repeat1",
            Python::DottedNameRepeat1 => "dotted_name_repeat1",
            Python::ComparisonOperatorRepeat1 => "comparison_operator_repeat1",
            Python::SubscriptRepeat1 => "subscript_repeat1",
            Python::ComprehensionClausesRepeat1 => "_comprehension_clauses_repeat1",
            Python::DictionaryRepeat1 => "dictionary_repeat1",
            Python::SetRepeat1 => "set_repeat1",
            Python::CollectionElementsRepeat1 => "_collection_elements_repeat1",
            Python::ForInClauseRepeat1 => "for_in_clause_repeat1",
            Python::ConcatenatedStringRepeat1 => "concatenated_string_repeat1",
            Python::StringRepeat1 => "string_repeat1",
            Python::FormatSpecifierRepeat1 => "format_specifier_repeat1",
            Python::Error => "ERROR",
        }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Python> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 8),
        (0, 0),
        (0, 7),
        (0, 77),
        (1, 81),
        (0, 3),
        (3, 1),
        (0, 13),
        (0, 32),
        (0, 67),
        (0, 211),
        (0, 9),
        (0, 11),
        (9, 147),
        (8, 109),
        (0, 206),
        (0, 7),
        (0, 3),
        (1, 112),
        (10, 66),
        (0, 6),
        (0, 104),
        (8, 185),
        (0, 0),
        (15, 122),
        (0, 0),
        (0, 22),
        (0, 85),
        (0, 0),
        (0, 42),
        (1, 2),
        (0, 0),
        (0, 3),
        (1, 103),
        (0, 13),
        (78, 104),
        (4, 212),
        (26, 152),
        (2, 22),
        (0, 48),
        (1, 117),
        (0, 206),
        (1, 11),
    ]),
    entries: ::phf::Slice::Static(&[
        ("module", Python::Module),
        ("|=", Python::PIPEEQ),
        ("if_statement", Python::IfStatement),
        ("if", Python::If),
        ("decorated_definition", Python::DecoratedDefinition),
        ("not", Python::Not),
        ("<=", Python::LTEQ),
        ("chevron", Python::Chevron),
        ("%=", Python::PERCENTEQ),
        ("attribute", Python::Attribute),
        ("async", Python::Async),
        ("named_expression", Python::NamedExpression),
        ("if_clause", Python::IfClause),
        ("format_specifier_repeat1", Python::FormatSpecifierRepeat1),
        ("aliased_import", Python::AliasedImport),
        ("*", Python::STAR),
        ("dictionary_splat", Python::DictionarySplat),
        ("parameters", Python::Parameters),
        ("for_in_clause", Python::ForInClause),
        ("]", Python::RBRACK),
        (
            "comparison_operator_repeat1",
            Python::ComparisonOperatorRepeat1,
        ),
        ("escape_sequence", Python::EscapeSequence),
        ("=", Python::EQ),
        ("else_clause", Python::ElseClause),
        ("pass_statement", Python::PassStatement),
        ("\\\"", Python::DQUOTE),
        ("is", Python::Is),
        ("import_prefix_repeat1", Python::ImportPrefixRepeat1),
        ("else", Python::Else),
        ("_expression", Python::Expression),
        ("try_statement", Python::TryStatement),
        (">=", Python::GTEQ),
        ("tuple", Python::Tuple),
        ("pair", Python::Pair),
        ("__future__", Python::Future),
        ("false", Python::False),
        ("import_statement", Python::ImportStatement),
        ("argument_list_repeat1", Python::ArgumentListRepeat1),
        (
            "decorated_definition_repeat1",
            Python::DecoratedDefinitionRepeat1,
        ),
        ("/=", Python::SLASHEQ),
        ("//=", Python::SLASHSLASHEQ),
        ("//", Python::SLASHSLASH),
        ("string", Python::String),
        ("**=", Python::STARSTAREQ),
        ("elif_clause", Python::ElifClause),
        ("break", Python::Break),
        ("function_definition", Python::FunctionDefinition),
        ("conditional_expression", Python::ConditionalExpression),
        ("<>", Python::LTGT),
        ("with", Python::With),
        ("none", Python::None),
        ("del", Python::Del),
        ("delete_statement", Python::DeleteStatement),
        ("variables", Python::Variables),
        ("_simple_statements", Python::SimpleStatements),
        ("exec", Python::Exec),
        ("==", Python::EQEQ),
        ("variables_repeat1", Python::VariablesRepeat1),
        ("_semicolon", Python::Semicolon),
        ("global_statement_repeat1", Python::GlobalStatementRepeat1),
        ("subscript", Python::Subscript),
        ("[", Python::LBRACK),
        ("print", Python::Print),
        (
            "_simple_statements_repeat1",
            Python::SimpleStatementsRepeat1,
        ),
        ("future_import_statement", Python::FutureImportStatement),
        ("format_specifier_token1", Python::FormatSpecifierToken1),
        ("return", Python::Return),
        ("break_statement", Python::BreakStatement),
        ("finally_clause", Python::FinallyClause),
        ("format_specifier", Python::FormatSpecifier),
        ("expression_list", Python::ExpressionList),
        (">>=", Python::GTGTEQ),
        ("^", Python::CARET),
        ("parenthesized_expression", Python::ParenthesizedExpression),
        ("augmented_assignment", Python::AugmentedAssignment),
        ("{", Python::LBRACE),
        ("parenthesized_list_splat", Python::ParenthesizedListSplat),
        (
            "_collection_elements_repeat1",
            Python::CollectionElementsRepeat1,
        ),
        ("set_repeat1", Python::SetRepeat1),
        ("raise", Python::Raise),
        ("nonlocal", Python::Nonlocal),
        ("comment", Python::Comment),
        ("|", Python::PIPE),
        ("unary_operator", Python::UnaryOperator),
        ("identifier", Python::Identifier),
        ("global_statement", Python::GlobalStatement),
        ("_right_hand_side", Python::RightHandSide),
        (">>", Python::GTGT),
        ("yield", Python::Yield),
        ("for_statement", Python::ForStatement),
        ("lambda_parameters", Python::LambdaParameters),
        (")", Python::RPAREN),
        ("except", Python::Except),
        ("nonlocal_statement", Python::NonlocalStatement),
        ("integer", Python::Integer),
        (
            "_comprehension_clauses_repeat1",
            Python::ComprehensionClausesRepeat1,
        ),
        ("type_conversion", Python::TypeConversion),
        ("string_repeat1", Python::StringRepeat1),
        ("%", Python::PERCENT),
        ("continue_statement", Python::ContinueStatement),
        ("as", Python::As),
        ("call", Python::Call),
        ("slice", Python::Slice),
        ("end", Python::End),
        ("/", Python::SLASH),
        ("<<", Python::LTLT),
        ("}", Python::RBRACE),
        ("concatenated_string", Python::ConcatenatedString),
        ("dotted_name", Python::DottedName),
        ("if_statement_repeat1", Python::IfStatementRepeat1),
        ("return_statement", Python::ReturnStatement),
        ("in", Python::In),
        ("@=", Python::ATEQ),
        ("binary_operator", Python::BinaryOperator),
        ("continue", Python::Continue),
        ("with_statement", Python::WithStatement),
        ("ERROR", Python::Error),
        ("_dedent", Python::Dedent),
        ("<", Python::LT),
        ("with_item", Python::WithItem),
        ("^=", Python::CARETEQ),
        ("set_comprehension", Python::SetComprehension),
        ("import_from_statement", Python::ImportFromStatement),
        ("class", Python::Class),
        ("ellipsis", Python::Ellipsis),
        ("expression_statement", Python::ExpressionStatement),
        ("comparison_operator", Python::ComparisonOperator),
        ("print_statement", Python::PrintStatement),
        ("for_in_clause_repeat1", Python::ForInClauseRepeat1),
        ("lambda", Python::Lambda),
        ("elif", Python::Elif),
        ("_collection_elements", Python::CollectionElements),
        (":", Python::COLON),
        ("for", Python::For),
        ("try", Python::Try),
        (
            "concatenated_string_repeat1",
            Python::ConcatenatedStringRepeat1,
        ),
        ("dictionary", Python::Dictionary),
        ("_primary_expression", Python::PrimaryExpression),
        ("raise_statement", Python::RaiseStatement),
        ("try_statement_repeat1", Python::TryStatementRepeat1),
        ("assert", Python::Assert),
        ("->", Python::DASHGT),
        ("-=", Python::DASHEQ),
        ("finally", Python::Finally),
        ("&", Python::AMP),
        ("exec_statement", Python::ExecStatement),
        ("**", Python::STARSTAR),
        ("import_prefix", Python::ImportPrefix),
        ("_comprehension_clauses", Python::ComprehensionClauses),
        ("decorator", Python::Decorator),
        ("except_clause", Python::ExceptClause),
        ("type", Python::Type),
        ("list", Python::List),
        ("_statement", Python::Statement),
        (
            "_expression_within_for_in_clause",
            Python::ExpressionWithinForInClause,
        ),
        ("-", Python::DASH),
        ("await", Python::Await),
        ("_indent", Python::Indent),
        ("!=", Python::BANGEQ),
        ("_import_list_repeat1", Python::ImportListRepeat1),
        ("module_repeat1", Python::ModuleRepeat1),
        ("_parameters_repeat1", Python::ParametersRepeat1),
        ("_import_list", Python::ImportList),
        ("print_statement_repeat1", Python::PrintStatementRepeat1),
        ("_not_escape_sequence", Python::NotEscapeSequence),
        ("or", Python::Or),
        ("generator_expression", Python::GeneratorExpression),
        ("from", Python::From),
        ("with_statement_repeat1", Python::WithStatementRepeat1),
        ("&=", Python::AMPEQ),
        ("true", Python::True),
        ("_newline", Python::Newline),
        ("boolean_operator", Python::BooleanOperator),
        ("float", Python::Float),
        ("(", Python::LPAREN),
        ("assert_statement", Python::AssertStatement),
        ("@", Python::AT),
        ("assert_statement_repeat1", Python::AssertStatementRepeat1),
        ("block", Python::Block),
        ("assignment", Python::Assignment),
        ("import", Python::Import),
        ("not_operator", Python::NotOperator),
        ("+", Python::PLUS),
        ("argument_list", Python::ArgumentList),
        ("while_statement", Python::WhileStatement),
        ("_string_content", Python::StringContent),
        ("typed_parameter", Python::TypedParameter),
        ("<<=", Python::LTLTEQ),
        ("relative_import", Python::RelativeImport),
        (":=", Python::COLONEQ),
        ("def", Python::Def),
        ("default_parameter", Python::DefaultParameter),
        ("+=", Python::PLUSEQ),
        ("wildcard_import", Python::WildcardImport),
        ("typed_default_parameter", Python::TypedDefaultParameter),
        ("pass", Python::Pass),
        (">", Python::GT),
        ("interpolation", Python::Interpolation),
        ("dictionary_comprehension", Python::DictionaryComprehension),
        ("keyword_argument", Python::KeywordArgument),
        ("format_expression", Python::FormatExpression),
        (".", Python::DOT),
        ("global", Python::Global),
        (",", Python::COMMA),
        ("and", Python::And),
        ("set", Python::Set),
        ("*=", Python::STAREQ),
        ("~", Python::TILDE),
        ("subscript_repeat1", Python::SubscriptRepeat1),
        ("class_definition", Python::ClassDefinition),
        ("while", Python::While),
        ("list_splat", Python::ListSplat),
        ("dotted_name_repeat1", Python::DottedNameRepeat1),
        ("list_comprehension", Python::ListComprehension),
        ("dictionary_repeat1", Python::DictionaryRepeat1),
    ]),
};

impl From<&str> for Python {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Python {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Python == u16
impl PartialEq<u16> for Python {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Python::from(*x)
    }
}

// u16 == Python
impl PartialEq<Python> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Python) -> bool {
        *x == *self
    }
}
