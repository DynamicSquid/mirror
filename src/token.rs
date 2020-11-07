#[derive(PartialEq, Debug, Clone)]
enum TokType {
    Set,
    If, Else,
    For, In,
    Def,
    Op,
    IntVal, FloatVal, CharVal, StrVal,
    IntArr, FloatArr, CharArr, StrArr,
    Var,
    Assign,
    OpenBracket, CloseBracket,
    OpenSquare, CloseSquare,
    OpenCurly, CloseCurly,
    Comma,
    Eol
}

#[derive(Clone)]
struct Tok {
    typ: TokType,
    val: String
}