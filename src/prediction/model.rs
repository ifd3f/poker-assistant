use poker::Card;
use smallvec::{smallvec, SmallVec};

pub const CARD_SV_SIZE: usize = 8;
pub type HandVec = SmallVec<[Card; CARD_SV_SIZE]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<Hole, Exchanged> {
    pub hole: Hole,
    pub exchanged: Exchanged,
    pub stud: HandVec,
}

pub type ThisPlayer = Player<HandVec, HandVec>;
pub type OtherPlayer = Player<usize, usize>;

pub struct Game {
    pub player: ThisPlayer,
    pub opponents: SmallVec<[OtherPlayer; 8]>,
    pub community: SmallVec<[Card; 8]>,
}

impl Game {
    #[inline]
    pub fn known_existing_cards(&self) -> impl Iterator<Item = Card> + '_ {
        let player_hole = self.player.hole.iter().copied();
        let player_stud = self.player.stud.iter().copied();
        let opponent_stud = self.opponents.iter().flat_map(|p| p.stud.iter().copied());

        player_hole.chain(player_stud).chain(opponent_stud)
    }

    #[inline]
    pub fn unknown_card_count(&self) -> usize {
        self.opponents.iter().map(|p| p.hole).sum()
    }
}

pub struct Holdem(Game);

impl Holdem {
    pub fn new(n_players: usize, player_hole: [Card; 2]) -> Self {
        Holdem(Game {
            player: Player {
                hole: smallvec![player_hole[0], player_hole[1]],
                stud: smallvec![],
                exchanged: smallvec![],
            },
            opponents: smallvec![Player{ hole: 2, stud: smallvec![], exchanged: 0 }; n_players],
            community: smallvec![],
        })
    }

    pub fn add_community(&mut self, cards: impl IntoIterator<Item = Card>) {
        self.0.community.extend(cards)
    }

    pub fn game(&self) -> &Game {
        &self.0
    }
}

/*
pub fn init_stud(n_players: usize, player_hole: Card) -> Game {
    Game {
        player: Player {
            hole: smallvec![player_hole],
            stud: smallvec![],
        },
        opponents: smallvec![Player{ hole: 2, stud: smallvec![] }; n_players],
        community: smallvec![],
    }
}

pub fn init_draw(n_players: usize, player_hole: [Card; 5]) -> Game {
    Game {
        player: Player {
            hole: player_hole.into(),
            stud: smallvec![],
        },
        opponents: smallvec![Player{ hole: 2, stud: smallvec![] }; n_players],
        community: smallvec![],
    }
}
*/
