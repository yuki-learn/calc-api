use super::{ Expr, Const };

#[test]
fn const_値がそのまま返ること() {
    // Arrange
    let expect = 10;
    let const_val = Const::new(expect);

    // Act
    let actual = const_val.eval();
    
    // Assert
    assert_eq!(actual, expect);
}

#[test]
fn add_足し算ができること() {
    // Arrange
    let add = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Const(Const::new(7))),
            Box::new(Expr::Const(Const::new(2)))
        )),

        Box::new(Expr::Const(Const::new(11)))
    );

    let except = (7 + 2) + 11;

    // Act
    let actual = add.eval();
    
    // Assert
    assert_eq!(actual, except);
}

#[test]
fn sub_引き算ができること() {
    // Arrange
    let sub = Expr::Sub(
        Box::new(Expr::Sub(
            Box::new(Expr::Const(Const::new(10))),
            Box::new(Expr::Const(Const::new(3)))
        )),

        Box::new(Expr::Const(Const::new(2)))
    );

    let except = (10 - 3) - 2;

    // Act
    let actual = sub.eval();
    
    // Assert
    assert_eq!(actual, except);
}

#[test]
fn mul_掛け算ができること() {
    // Arrange
    let mul = Expr::Mul(
        Box::new(Expr::Mul(
            Box::new(Expr::Const(Const::new(2))),
            Box::new(Expr::Const(Const::new(4)))
        )),

        Box::new(Expr::Const(Const::new(9)))
    );

    let except = (2 * 4) * 9;

    // Act
    let actual = mul.eval();
    
    // Assert
    assert_eq!(actual, except);
}


#[test]
fn div_割り算ができること() {
    // Arrange
    let div = Expr::Div(
        Box::new(Expr::Div(
            Box::new(Expr::Const(Const::new(24))),
            Box::new(Expr::Const(Const::new(6)))
        )),

        Box::new(Expr::Const(Const::new(2)))
    );

    let except = (24 / 6) / 2;

    // Act
    let actual = div.eval();
    
    // Assert
    assert_eq!(actual, except);
}

#[test]
fn eval_四則演算できること() {
    // Arrange
    let expr = Expr::Div(
        Box::new(Expr::Mul(
            Box::new(Expr::Sub(
                Box::new(Expr::Sub(
                    Box::new(Expr::Const(Const::new(10))),
                    Box::new(Expr::Const(Const::new(4)))
                )),
    
                Box::new(Expr::Add(
                    Box::new(Expr::Const(Const::new(2))),
                    Box::new(Expr::Const(Const::new(1)))
                )),
            )),
            Box::new(Expr::Const(Const::new(2))),
    
        )),
        Box::new(Expr::Const(Const::new(3))),
    );

    // (6) - (3) * 2 / 3 = 6 - 6 / 3 = 4
    let except = (10 - 4) - (2 + 1) * 2 / 3;

    // Act
    let actual = expr.eval();
    
    // Assert
    assert_eq!(actual, except);
}