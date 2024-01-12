use lexpr::Value;
use poker::{Card, ParseCardError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    /// Define a list of cards, but do not plot it.
    ///
    /// Cards are implicity discarded from the deck as well.
    DefineCards(DefineHand),

    /// Define a list of cards, and mark it as a group to be plotted.
    ///
    /// Cards are implicity discarded from the deck as well.
    PlotCards(DefineHand),

    /// Cards to be discarded from the deck.
    Discard(Vec<CardsExp>),
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::From)]
pub enum CardsExp {
    /// A singular card.
    #[from(forward)]
    Lit(Card),

    /// An unknown card.
    Hole,

    /// Substitute another hand in for this card.
    #[from(ignore)]
    Subs(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefineHand {
    /// The name of the hand.
    pub name: String,

    /// List of cards.
    pub cards: Vec<CardsExp>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Expected directive, got {got}")]
    ExpectedDirective { got: Value },

    #[error("Unknown directive {name}")]
    UnknownDirective { name: String },

    #[error("Could not parse card expression {0}")]
    CouldNotParseCard(ParseCardError),

    #[error("Bad DefineHand expression {0}")]
    BadDefineHandExpression(Value),

    #[error("Bad Discard expression {0}")]
    BadDiscardExpression(Value),
}

pub fn parse_directive(exp: &Value) -> Result<Directive, ParseError> {
    use ParseError::*;

    match exp {
        Value::Cons(c) => match c.car() {
            Value::Symbol(dname) => match dname.as_ref() {
                "define-cards" => Ok(Directive::DefineCards(parse_define_hand(c.cdr())?)),
                "plot-cards" => Ok(Directive::PlotCards(parse_define_hand(c.cdr())?)),
                "discard" => Ok(Directive::Discard(match c.cdr() {
                    Value::String(s) => parse_cards_list(s.as_ref()),
                    other => Err(BadDiscardExpression(other.clone())),
                }?)),
                name => Err(UnknownDirective {
                    name: name.to_owned(),
                }),
            },
            name => Err(UnknownDirective {
                name: name.to_string(),
            }),
        },
        got => Err(ExpectedDirective { got: got.clone() }),
    }
}

fn parse_define_hand(exp: &Value) -> Result<DefineHand, ParseError> {
    match exp {
        Value::Cons(c) => {
            let (c, _) = c.to_vec();
            match &c.as_slice() {
                &[Value::Symbol(name), Value::String(s)] => Ok(DefineHand {
                    name: name.to_string(),
                    cards: parse_cards_list(s.as_ref())?,
                }),
                _ => Err(ParseError::BadDefineHandExpression(exp.clone())),
            }
        }
        _ => Err(ParseError::BadDefineHandExpression(exp.clone())),
    }
}

fn parse_cards_list(cards_list: impl AsRef<str>) -> Result<Vec<CardsExp>, ParseError> {
    cards_list
        .as_ref()
        .split_whitespace()
        .map(parse_card_exp)
        .collect()
}

fn parse_card_exp(exp: impl AsRef<str>) -> Result<CardsExp, ParseError> {
    let exp = exp.as_ref();

    if exp.len() == 0 {
        panic!("exp.len() == 0, this should never happen")
    }

    match exp.split_at(1) {
        ("$", rest) => Ok(CardsExp::Subs(rest.to_owned())),
        ("?", "") => Ok(CardsExp::Hole),
        _ => Ok(CardsExp::Lit(
            exp.parse::<Card>().map_err(ParseError::CouldNotParseCard)?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use lexpr::sexp;
    use poker::*;

    use super::*;

    #[test]
    fn parse_plot_cards() {
        let val: Value = r#"(plot-cards player "3c Td $community ? ?")"#.parse::<Value>().unwrap();

        let result = parse_directive(&val).unwrap();

        assert_eq!(
            result,
            Directive::PlotCards(DefineHand {
                name: "player".into(),
                cards: vec![
                    Card::new(Rank::Three, Suit::Clubs).into(),
                    Card::new(Rank::Ten, Suit::Diamonds).into(),
                    CardsExp::Subs("community".into()),
                    CardsExp::Hole,
                    CardsExp::Hole,
                ]
            })
        )
    }

    #[test]
    fn parse_define_cards() {
        let val: Value = r#"(define-cards player "3c Td $community ? ?")"#.parse::<Value>().unwrap();

        let result = parse_directive(&val).unwrap();

        assert_eq!(
            result,
            Directive::DefineCards(DefineHand {
                name: "player".into(),
                cards: vec![
                    Card::new(Rank::Three, Suit::Clubs).into(),
                    Card::new(Rank::Ten, Suit::Diamonds).into(),
                    CardsExp::Subs("community".into()),
                    CardsExp::Hole,
                    CardsExp::Hole,
                ]
            })
        )
    }

    #[test]
    fn parse_discard_cards() {
        let val: Value = r#"(define-cards player "3c Td 2s")"#.parse::<Value>().unwrap();

        let result = parse_directive(&val).unwrap();

        assert_eq!(
            result,
            Directive::DefineCards(DefineHand {
                name: "player".into(),
                cards: vec![
                    Card::new(Rank::Three, Suit::Clubs).into(),
                    Card::new(Rank::Ten, Suit::Diamonds).into(),
                    Card::new(Rank::Two, Suit::Spades).into(),
                ]
            })
        )
    }
}
