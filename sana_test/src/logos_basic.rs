pub fn basic() -> &'static [(&'static str, &'static str)] {
    &[
        ( r"[ \n\t\f]", "Whitespace" ),
        ( "[a-zA-Z_$][a-zA-Z0-9_$]*", "Identifier" ),
        ( r#""([^"\\]|\\t|\\u|\\n|\\")*""#, "String" ),
        ( "private", "Private" ),
        ( "primitive", "Primitive" ),
        ( "protected", "Protected" ),
        ( "in", "In" ),
        ( "instanceof", "Instanceof" ),
        ( r"\.", "Accessor" ),
        ( r"\.\.\.", "Ellipsis" ),
        ( r"\(", "ParenOpen" ),
        ( r"\)", "ParenClose" ),
        ( r"\{", "BraceOpen" ),
        ( r"\}", "BraceClose" ),
        ( r"\+", "OpAddition" ),
        ( r"\+\+", "OpIncrement" ),
        ( "=", "OpAssign" ),
        ( "==", "OpEquality" ),
        ( "===", "OpStrictEquality" ),
        ( "=>", "FatArrow" ),
    ]
}
