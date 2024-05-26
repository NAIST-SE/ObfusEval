#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    IdentifierDigit(i32),
    // Constant(String),
    // string-literal
    Punctuator(String),
    PreProcessor(String),
    // ライブラリインクルード構文規則
    IncludeLibrarySyntax(String),
    IncludeLibraryLocalSyntax(String),
    // コメント構文規則
    SingleLineCommentSyntax(String),
    MultiLineCommentSyntax(String),

    // 文字列リテラル構文規則
    StringLiteralSyntax(String),
    WideStringLiteralSyntax(String),
}

impl Token {
    pub fn get_keywords() -> Vec<&'static str> {
        vec![
            "auto",
            "break",
            "case",
            "char",
            "const",
            "continue",
            "default",
            "do",
            "double",
            "else",
            "enum",
            "extern",
            "float",
            "for",
            "goto",
            "if",
            "inline",
            "int",
            "long",
            "register",
            "restrict",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "struct",
            "switch",
            "typedef",
            "typeof",
            "typeof_unqual",
            "union",
            "unsigned",
            "void",
            "volatile",
            "while",
            "_Alignas",
            "_Alignof",
            "_Atomic",
            "_Bool",
            "_Complex",
            "_Generic",
            "_Imaginary",
            "_Noreturn",
            "_Static_assert",
            "_Thread_local",
            "__asm",
            "__based",
            "__cdecl",
            "__declspec",
            "__except",
            "__finally",
            "__fastcall",
            "__int16",
            "__inline",
            "__int64",
            "__int32",
            "__leave",
            "__int8",
            "__stdcall",
            "__restrict",
            "__typeof__",
            "__try",
            "dllexport",
            "__typeof_unqual__",
            "naked",
            "dllimport",
            "thread",
            "static_assert",
        ]
    }

    pub fn get_punctuators_as_str_only_char() -> &'static str {
        "()[]{}*,:=;"
    }

    pub fn get_puctuator_for_varadic_arguments() -> &'static str {
        "..."
    }

    pub fn get_pre_processors() -> Vec<&'static str> {
        vec![
            "#define", "#elif", "#else", "#endif", "#error", "#if", "#ifdef", "#ifndef", "#import",
            "#line", "#pragma", "#undef", "#using",
        ]
    }

    // pub fn get_puctuator_for_varadic_arguments() -> &'static str {
    //     "..."
    // }

    // pub fn get_puctuators_only_char() -> Vec<char> {
    //     vec!['(', ')', '[', ']', '{', '}', '*', ',', ':', '=', ';', '#']
    // }

    // pub fn get_puctuators_only_char() -> Vec<char> {
    //     vec!['(', ')', '[', ']', '{', '}', '*', ',', ':', '=', ';', '#']
    // }
    // '...'
}

/*
Operator Reference:
- https://learn.microsoft.com/en-us/cpp/c-language/precedence-and-order-of-evaluation?view=msvc-170
- (ja) https://www602.math.ryukoku.ac.jp/Prog1/cops.pdf
*/
#[derive(Debug, PartialEq)]
pub enum Toksen {
    StorageClass(StorageClassMember),
    Struct(String),
    Type(TypeMember),
    Variable(String),
    // 文字列
    DoubleQuotation(String),
    // 可変長引数
    VaradicArguments,
    // ()
    ParenthesesStart,
    ParenthesesEnd,
    // {}
    CurlyBracketStart,
    CurlyBracketEnd,
    // 構文終了
    StatementEndStyntax,
    // コメント構文規則
    SingleLineCommentOperator(String),
    MultiLineCommentOperator(String),
    // -------
    // 関数呼び出し演算子
    CallFunctionOperator(String),
    // 配列添字演算子
    SubscriptOperator(String),
    // 直接メンバ・間接メンバ演算子
    DirectMemberReferenceOperator,
    IndirectMemberReferenceOperator,
    // 後置増分・後置減分演算子
    PostfixIncrementOperator,
    PostfixDecrementOperator,
    // 前置増分・前置減分演算子
    PrefixIncrementOperator,
    PrefixDecrementOperator,
    // 記憶量演算子
    SizeofOperator,
    // アドレス演算子
    AddressOperator,
    // 間接参照演算子
    IndirectReferenceOperator,
    // 単項加算・単項減算演算子
    UnaryAdditionEvaluatingOperator(String),
    UnarySubtractionEvaluatingOperator(String),
    // ビット反転・否定演算子
    BitReversalEvaluatingOperator,
    NegationEvaluatingOperator,
    // キャスト演算子
    CastOperator(String),
    // 乗算・除算・乗除演算子
    MultiplicationEvaluatingOperator(String, String),
    DivisionEvaluatingOperator(String, String),
    RemainderEvaluatingOperator(String, String),
    // 加算・減算演算子
    AdditionEvaluatingOperator(String, String),
    SubtractionEvaluatingOperator(String, String),
    // シフト演算子
    LeftShiftEvaluatingOperator(String, String),
    RightShiftEvaluatingOperator(String, String),
    // 比較演算子
    SmallerThanEvaluatingOperator(String, String),
    SmalletThanOrEqualEvaluatingOperator(String, String),
    GreaterThanOrEqualEvaluatingOperator(String, String),
    GreaterThanEvaluatingOperator(String, String),
    // 等価評価演算子
    EqualEvaluatingOperator(String, String),
    NotEqualEvaluatingOperator(String, String),
    // ビット演算子
    BitwiseAndOperator(String, String),
    BitwiseXorOperator(String, String),
    BitwiseOrOperator(String, String),
    // 論理演算子
    LogicalAndOperator(String, String),
    LogicalOrOperator(String, String),
    // 三項演算子
    ConditionalExpressionOperator,
    ConditionalExpressionEndingOperator,
    // 代入演算子
    SimpleAssignmentOperator(String, String),
    AdditionAssignmentOperator(String, String),
    SubtractionAssignmentOperator(String, String),
    MultiplicationAssignmentOperator(String, String),
    DivisionAssignmentOperator(String, String),
    RemainderAssignmentOperator(String, String),
    // シフト代入演算子
    LeftShiftAssignmentOperator(String, String),
    RightShiftAssignmentOperator(String, String),
    // ビット代入演算子
    BitwiseAndAssignmentOperator(String, String),
    BitwiseXorAssignmentOperator(String, String),
    BitwiseOrAssignmentOperator(String, String),
    // 順次評価演算子
    SequentialEvaluatingOperator,
}

#[derive(Debug, PartialEq)]
pub enum StorageClassMember {
    Auto,
    Register,
    Static,
    Extern,
    TypeDef,
}

#[derive(Debug, PartialEq)]
pub enum TypeMember {
    Void,
    Bool,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    SignedShort,
    UnsignedShort,
    Int,
    SignedInt,
    UnsignedInt,
    Long,
    SignedLong,
    UnsignedLong,
    LongLong,
    SingedLongLong,
    UnsingedLongLong,
    Float,
    Double,
}
