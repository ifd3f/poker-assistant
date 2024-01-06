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

pub type ThisPlayer = Player<PartialHand, PartialHand>;
pub type OtherPlayer = Player<PartialHand<u8>, usize>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialHand<Drawn = HandVec> {
    pub drawn: Drawn,
    pub undrawn: u8,
}

impl PartialHand {
    pub fn null() -> Self {
        Self {
            drawn: smallvec![],
            undrawn: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Game {
    pub player: ThisPlayer,
    pub opponents: SmallVec<[OtherPlayer; 8]>,
    pub community: PartialHand,
}

impl Game {
    pub fn from_deals(deals: impl IntoIterator<Item = Deal>) -> Self {
        deals.into_iter().fold(Game::default(), |mut g, d| {
            g.player.hole.undrawn += d.hole;
            g.player.stud.undrawn += d.stud;
            g.community.undrawn += d.community;
            for p in &mut g.opponents {
                p.hole.undrawn += d.hole;
                p.stud.undrawn += d.stud;
            }
            g
        })
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
