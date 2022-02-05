use nom::{IResult};
use rstest::rstest;
use crate::ast::{Const, Expr};
use super::{const_val_parser, expr_parser};


#[rstest]
#[case("1", Const::new(1))]
#[case(" 0   ", Const::new(0))]
#[case("1000 ", Const::new(1000))]
fn const_val_parser_文字列から1つ以上の数値がパースされること(#[case] input: &str, #[case] expected: Const) {
    // Act
    let (_, actual) = const_val_parser(input).unwrap();
    
    // Assert
    assert_eq!(actual, expected);
}

#[rstest]
#[case("2*(3+4)", 14)]
#[case("(4-1)*3", 9)]
#[case("(1 + 2) * (10 - 2)", 24)]
fn expr_parser_かっこが最優先で計算されること(#[case] input: &str, #[case] expected: i32) {
    // Act
    let (_, actual) = expr_parser(input).unwrap();

    // Assert
    assert_eq!(actual.eval(), expected);
}

#[rstest]
#[case("2+3*4", 14)]
#[case("2+8/4", 4)]
#[case("10/2+4", 9)]
fn expr_parser_乗算_除算が加算_減算より優先度が高いこと(#[case] input: &str, #[case] expected: i32) {
    // Act
    let (_, actual) = expr_parser(input).unwrap();
    
    // Assert
    assert_eq!(actual.eval(), expected);
}

#[rstest]
#[case("1-2-3", -4)]
#[case("1+2-3", 0)]
#[case("4*5/2", 10)]
#[case("50/5*2", 20)]
fn expr_parser_同じ優先順位の演算子は左から計算されること(#[case] input: &str, #[case] expected: i32) {
    // Act
    let (_, expr) = expr_parser(input).unwrap();
    
    // Assert
    assert_eq!(expr.eval(), expected);
}

#[rstest]
#[case("1--2-3", Err(nom::Err::Error(nom::error::Error::new("--2-3", nom::error::ErrorKind::Not))))]
#[case("1-2+*3", Err(nom::Err::Error(nom::error::Error::new("-2+*3", nom::error::ErrorKind::Not))))]
fn expr_parser_式が不正なときエラーが返ること(#[case] input: &str, #[case] expected: IResult<&str, Expr>) {
    // Act
    let result = expr_parser(input).unwrap_err();
    
    // Assert
    assert_eq!(result, expected.unwrap_err());
}