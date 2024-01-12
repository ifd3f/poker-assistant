use std::collections::{HashMap, HashSet};

use compact_poker::SCard;
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

    #[error("Error parsing S-expression: {0}")]
    LexprError(#[from] lexpr::parse::Error),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum EvaluationError {
    #[error("Hand already exists: {0}")]
    HandAlreadyExists(String),

    #[error("Could not find hand with name {0}")]
    UnknownHand(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Evaluation {
    /// Every card to remove from the sampling deck.
    ///
    /// This includes exchanged cards and known cards in players' hands.
    pub discarded: HashSet<SCard>,

    /// Hands in the evaluation.
    pub hands: HashMap<String, ConcreteHand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteHand {
    /// Whether or not this hand should get a histogram.
    pub should_plot: bool,

    /// Name of this hand.
    pub name: String,
    
    /// Cards known in this hand.
    pub known_cards: HashSet<SCard>,

    /// Number of holes in the hand for the simulator to fill.
    pub n_holes: usize,
}

pub fn evaluate_directives<'a>(
    directives: impl IntoIterator<Item = &'a Directive>,
) -> Result<Evaluation, EvaluationError> {
    let mut ctx = Evaluation::default();
    for d in directives {
        evaluate_directive(&mut ctx, d)?;
    }
    Ok(ctx)
}

fn evaluate_directive(ctx: &mut Evaluation, directive: &Directive) -> Result<(), EvaluationError> {
    match directive {
        Directive::DefineCards(dh) => {
            let ch = evaluate_define_hand(ctx, dh, false)?;
            ctx.discarded.extend(&ch.known_cards);
            ctx.hands.insert(ch.name.clone(), ch);
        }
        Directive::PlotCards(dh) => {
            let ch = evaluate_define_hand(ctx, dh, true)?;
            ctx.discarded.extend(&ch.known_cards);
            ctx.hands.insert(ch.name.clone(), ch);
        }
        Directive::Discard(cards) => {
            let (known_cards, _n_holes) = evaluate_card_exprs(ctx, cards)?;
            ctx.discarded.extend(known_cards);
        }
    }

    Ok(())
}

fn evaluate_define_hand(
    ctx: &Evaluation,
    dh: &DefineHand,
    should_plot: bool,
) -> Result<ConcreteHand, EvaluationError> {
    if let Some(_) = ctx.hands.get(&dh.name) {
        return Err(EvaluationError::HandAlreadyExists(dh.name.clone()));
    }

    let (known_cards, n_holes) = evaluate_card_exprs(ctx, dh.cards.iter())?;

    Ok(ConcreteHand {
        should_plot,
        name: dh.name.clone(),
        known_cards,
        n_holes,
    })
}

fn evaluate_card_exprs<'a>(
    ctx: &Evaluation,
    card_exprs: impl IntoIterator<Item = &'a CardsExp>,
) -> Result<(HashSet<SCard>, usize), EvaluationError> {
    let mut known_cards = HashSet::new();
    let mut n_holes = 0;
    for cexpr in card_exprs {
        match cexpr {
            CardsExp::Lit(c) => {
                known_cards.insert((*c).into());
            }
            CardsExp::Hole => {
                n_holes += 1;
            }
            CardsExp::Subs(ref_name) => match ctx.hands.get(ref_name) {
                Some(hand) => {
                    known_cards.extend(&hand.known_cards);
                    n_holes += hand.n_holes;
                }
                None => Err(EvaluationError::UnknownHand(ref_name.clone()))?,
            },
        }
    }

    Ok((known_cards, n_holes))
}

pub fn parse_program_from_str(s: &str) -> Result<Vec<Directive>, ParseError> {
    match lexpr::from_str(&format!("({s})"))? {
        Value::Cons(v) => {
            let (ds, _) = v.into_vec();
            ds.iter().map(parse_directive).collect()
        }
        _ => panic!("impossible state - must be a cons because we surrounded it with parens"),
    }
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
        let val: Value =
            r#"(define-cards player "3c Td $community ? ?")"#.parse::<Value>().unwrap();

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
                    SCard::new(Rank::Three, Suit::Clubs).into(),
                    SCard::new(Rank::Ten, Suit::Diamonds).into(),
                    SCard::new(Rank::Two, Suit::Spades).into(),
                ]
            })
        )
    }

    #[allow(non_snake_case)]
    #[test]
    fn eval_holdem_example() {
        let program = r#"
            (define-cards community "3c Td 2s ? ?")
            (plot-cards self "As Kh $community")
            (plot-cards opponents "? ? $community")
        "#;

        let result = parse_program_from_str(program).unwrap();
        let eval = evaluate_directives(result.iter()).unwrap();

        let c_3c = SCard::new(Three, Clubs);
        let c_Td = SCard::new(Ten, Diamonds);
        let c_As = SCard::new(Ace, Spades);
        let c_Kh = SCard::new(King, Hearts);
        let c_2s = SCard::new(Two, Spades);

        use Rank::*;
        use Suit::*;

        let expected = Evaluation {
            discarded: [c_3c, c_Td, c_As, c_2s, c_Kh].into(),
            hands: [
                (
                    "community".into(),
                    ConcreteHand {
                        should_plot: false,
                        name: "community".into(),
                        known_cards: [c_3c, c_Td, c_2s].into(),
                        n_holes: 2,
                    },
                ),
                (
                    "self".into(),
                    ConcreteHand {
                        should_plot: true,
                        name: "self".into(),
                        known_cards: [c_3c, c_Td, c_2s, c_As, c_Kh].into(),
                        n_holes: 2,
                    },
                ),
                (
                    "opponents".into(),
                    ConcreteHand {
                        should_plot: true,
                        name: "opponents".into(),
                        known_cards: [c_3c, c_Td, c_2s].into(),
                        n_holes: 4,
                    },
                ),
            ]
            .into(),
        };

        assert_eq!(eval, expected)
    }
}
