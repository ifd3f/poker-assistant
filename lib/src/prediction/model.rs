use compact_poker::SCard;
use smallvec::{smallvec, SmallVec};

use crate::game_repr::Deal;

/// We use 7 to be able to hold a 7-card stud hand.
pub type HandVec<C = SCard> = SmallVec<[C; 7]>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Player<Hole, Exchanged> {
    /// Cards unknown to other players.
    pub hole: Hole,

    /// Cards exchanged with the sample deck.
    pub exchanged: Exchanged,

    /// Cards unknown to other players.
    pub stud: PartialHand,
}

pub type ThisPlayer = Player<PartialHand<HandVec>, HandVec>;
pub type OtherPlayer = Player<PartialHand<u8>, usize>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialHand<Drawn = HandVec> {
    /// Cards drawn so far.
    pub drawn: Drawn,

    /// Cards yet to be drawn.
    pub undrawn: u8,
}

impl<D: Default> PartialHand<D> {
    pub fn null() -> Self {
        Self {
            drawn: Default::default(),
            undrawn: 0,
        }
    }

    pub fn undrawn(undrawn: u8) -> Self {
        Self {
            drawn: Default::default(),
            undrawn,
        }
    }
}

impl PartialHand<u8> {
    pub fn total_cards(&self) -> u8 {
        self.drawn + self.undrawn
    }

    pub fn add_cards(&mut self, cards: u8) {
        self.drawn += cards;
        self.undrawn -= cards;
    }
}

impl PartialHand<HandVec> {
    pub fn add_cards(&mut self, cards: impl IntoIterator<Item = SCard>) {
        let initial_size = self.drawn.len();
        self.drawn.extend(cards);
        self.undrawn -= (self.drawn.len() - initial_size) as u8;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Game {
    pub player: ThisPlayer,
    pub opponents: SmallVec<[OtherPlayer; 8]>,
    pub community: PartialHand,
}

impl Game {
    pub fn from_deals(n_opponents: usize, deals: impl IntoIterator<Item = Deal>) -> Self {
        let net_deal: Deal = deals.into_iter().sum();

        Game {
            player: ThisPlayer {
                hole: PartialHand::undrawn(net_deal.hole),
                exchanged: smallvec![],
                stud: PartialHand::undrawn(net_deal.stud),
            },
            opponents: smallvec![
                OtherPlayer {
                    hole: PartialHand::undrawn(net_deal.hole),
                    exchanged: 0,
                    stud: PartialHand::undrawn(net_deal.stud)
                }; n_opponents
            ],
            community: PartialHand::undrawn(net_deal.community),
        }
    }

    #[inline]
    pub fn known_existing_cards(&self) -> impl Iterator<Item = SCard> + '_ {
        let player_hole = self.player.hole.drawn.iter().copied();
        let player_stud = self.player.stud.drawn.iter().copied();
        let opponent_stud = self
            .opponents
            .iter()
            .flat_map(|p| p.stud.drawn.iter().copied());

        player_hole.chain(player_stud).chain(opponent_stud)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_repr::*;

    #[test]
    fn from_deals_works_for_holdem() {
        let game = Game::from_deals(
            2,
            holdem().into_iter().filter_map(|r| match r {
                Round::Deal(d) => Some(d),
                _ => None,
            }),
        );

        assert_eq!(
            game,
            Game {
                player: Player {
                    hole: PartialHand::undrawn(2),
                    exchanged: smallvec![],
                    stud: PartialHand::undrawn(0)
                },
                opponents: smallvec![Player {
                    hole: PartialHand::undrawn(2),
                    exchanged: 0,
                    stud: PartialHand::undrawn(0)
                }; 2],
                community: PartialHand::undrawn(5)
            }
        )
    }
}
