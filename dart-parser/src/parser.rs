mod class;
mod comment;
mod common;
mod directive;
mod expr;
mod func;
mod scope;
mod string;
mod var;

use std::str;

use nom::{branch::alt, combinator::eof, multi::many0, sequence::terminated, Parser};

use crate::{dart::*, parser::class::class};

use self::{comment::comment, common::spbr, directive::directive, func::func, var::var};

type PResult<'s, T> = nom::IResult<&'s str, T>;

pub fn parse(s: &str) -> PResult<Vec<Dart>> {
    terminated(
        many0(alt((
            spbr.map(Dart::Verbatim),
            directive.map(Dart::Directive),
            var.map(Dart::Var),
            func.map(Dart::Func),
            class.map(Dart::Class),
            comment.map(Dart::Comment),
        ))),
        eof,
    )(s)
}

#[cfg(test)]
mod tests {

    use crate::dart::{
        comment::Comment,
        directive::{Directive, Import},
    };

    use super::*;

    #[test]
    fn mixed_test() {
        assert_eq!(
            parse(DART_MIXED.trim_start()),
            Ok((
                "",
                vec![
                    Dart::Directive(Directive::Import(Import::target("dart:math"))),
                    Dart::Verbatim("\n"),
                    Dart::Directive(Directive::Import(Import::target_as(
                        "package:path/path.dart",
                        "p"
                    ))),
                    Dart::Verbatim("\n\n"),
                    Dart::Directive(Directive::Part("types.g.dart")),
                    Dart::Verbatim("\n\n"),
                    Dart::Comment(Comment::SingleLine("// A comment\n")),
                    Dart::Comment(Comment::MultiLine("/*\nAnother comment\n*/")),
                    Dart::Verbatim("\n"),
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([VarModifier::Const]),
                        var_type: None,
                        name: "category",
                        initializer: Some("\"mixed bag\""),
                    }),
                    Dart::Verbatim("\n"),
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([
                            VarModifier::Late,
                            VarModifier::Final
                        ]),
                        var_type: Some(IdentifierExt::name("int")),
                        name: "crash_count",
                        initializer: None,
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Base",
                        extends: None,
                        implements: Vec::default(),
                        body: "{\n  String id;\n}",
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Record",
                        extends: Some(IdentifierExt::name("Base")),
                        implements: vec![
                            IdentifierExt {
                                name: "A",
                                type_args: vec![
                                    IdentifierExt {
                                        name: "Future",
                                        type_args: vec![IdentifierExt::name("void")],
                                        is_nullable: false,
                                    },
                                    IdentifierExt {
                                        name: "B",
                                        type_args: Vec::default(),
                                        is_nullable: true,
                                    },
                                ],
                                is_nullable: false,
                            },
                            IdentifierExt::name("C")
                        ],
                        body: "{\n  String name;\n}",
                    }),
                    Dart::Verbatim("\n")
                ]
            ))
        );
    }

    const DART_MIXED: &str = r#"
import 'dart:math';
import 'package:path/path.dart' as p;

part 'types.g.dart';

// A comment
/*
Another comment
*/
const category = "mixed bag";
late final int crash_count;

class Base {
  String id;
}

class Record extends Base implements A<Future<void>, B?>, C {
  String name;
}
"#;
}
