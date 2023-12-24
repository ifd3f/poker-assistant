{-# LANGUAGE InstanceSigs #-}
module Poker
  ( Suit(..)
  , Rank(..)
  , Card(..)
  , Hand(..)
  , HandPattern(..)
  , patterns
  , makeHand
  , suit
  , rank
  , displaySuit
  , (♣)
  , (♠)
  , (♥)
  , (♦)
  , (♣.)
  , (♠.)
  , (♥.)
  , (♦.)
  ) where

import qualified Data.List as List
import Data.List.NonEmpty (NonEmpty(..))
import qualified Data.List.NonEmpty as NonEmpty
import qualified Data.Map as Map
import Data.Map (Map)
import Data.Maybe (maybeToList)

data Suit
  = Club
  | Diamond
  | Heart
  | Spade
  deriving (Eq, Ord)

instance Show Suit where
  show :: Suit -> String
  show s = "(" ++ [displaySuit s] ++ ")"

displaySuit :: Suit -> Char
displaySuit Club = '♣'
displaySuit Spade = '♠'
displaySuit Heart = '♥'
displaySuit Diamond = '♦'

data Rank
  = R2
  | R3
  | R4
  | R5
  | R6
  | R7
  | R8
  | R9
  | R10
  | RJ
  | RQ
  | RK
  | RA
  deriving (Eq, Ord, Show)

data Card =
  Rank `Of` Suit
  deriving (Eq, Ord)

instance Show Card where
  show :: Card -> String
  show (r `Of` s) = "(" ++ [displaySuit s] ++ ".)" ++ show r

(♣) :: Suit
(♣) = Club

(♠) :: Suit
(♠) = Spade

(♥) :: Suit
(♥) = Heart

(♦) :: Suit
(♦) = Diamond

(♣.) :: Rank -> Card
(♣.) r = r `Of` Club

(♠.) :: Rank -> Card
(♠.) r = r `Of` Spade

(♥.) :: Rank -> Card
(♥.) r = r `Of` Heart

(♦.) :: Rank -> Card
(♦.) r = r `Of` Diamond

rank :: Card -> Rank
rank (r `Of` _) = r

suit :: Card -> Suit
suit (_ `Of` s) = s

-- | A hand of five cards. Invariant: cards are in descending natural Card order.
newtype Hand =
  Hand (NonEmpty Card)
  deriving (Eq, Show)

-- | Pattern of hands. Multiple of these may apply to a single hand.
data HandPattern
  = StraightFlush Card
  | Kind Int Rank
  | FullHouse Rank Rank
  | Flush Suit
  | Straight Rank
  | High Rank
  | HighSuit Card
  deriving (Eq, Show)

instance Ord HandPattern where
  compare pl pr =
    case (pl, pr) of
      (Kind nl rl, Kind nr rr) -> (nl, rl) `compare` (nr, rr)
      -- Straight/Royal Flush
      (StraightFlush l, StraightFlush r) -> l `compare` r
      (StraightFlush _, _) -> GT
      (_, StraightFlush _) -> LT
      -- 4 of a kind
      (Kind 4 _, _) -> GT
      (_, Kind 4 _) -> LT
      -- Full house
      (FullHouse l3 l2, FullHouse r3 r2) -> (l3, l2) `compare` (r3, r2)
      (FullHouse _ _, _) -> GT
      (_, FullHouse _ _) -> LT
      -- Flush
      (Flush _, Flush _) -> EQ
      (Flush _, _) -> GT
      (_, Flush _) -> LT
      -- Straight
      (Straight l, Straight r) -> l `compare` r
      (Straight _, _) -> GT
      (_, Straight _) -> LT
      -- 3-of-a-kind
      (Kind 3 _, _) -> GT
      (_, Kind 3 _) -> LT
      -- 2-of-a-kind
      (Kind 2 _, _) -> GT
      (_, Kind 2 _) -> LT
      -- High
      (High l, High r) -> l `compare` r
      (High _, _) -> GT
      (_, High _) -> LT
      -- Tiebreaker
      (HighSuit l, HighSuit r) -> l `compare` r
      (Kind _ _, _) -> GT
      (_, Kind _ _) -> LT

-- | Return a list of every pattern that a hand matches.
patterns :: Hand -> [HandPattern]
patterns h@(Hand (first :| rest)) =
  List.sortBy (flip compare) $
  (StraightFlush <$> maybeToList straightFlush) ++
  (Straight <$> maybeToList isStraight') ++
  maybeToList fullHouse ++
  kinds ++ (Flush <$> maybeToList isFlush') ++ highs ++ bottomTiebreaker
  where
    cs = first : rest
    isFlush' = isFlush h
    isStraight' = isStraight h
    straightFlush = do
      _ <- isFlush'
      _ <- isStraight'
      pure first
    counts = Map.toList (rankCounts cs)
    kinds = map (\(r, n) -> Kind n r) $ filter ((>= 2) . snd) counts
    fullHouse = do
      (three, _) <- List.find ((== 3) . snd) counts
      (two, _) <- List.find ((== 2) . snd) counts
      pure $ FullHouse three two
    highs = fmap (High . rank) cs
    bottomTiebreaker = fmap HighSuit cs

-- | Ranks in descending order.
rankOrder :: [Rank]
rankOrder = [R2, R3, R4, R5, R6, R7, R8, R9, R10, RJ, RQ, RK, RA]

-- | Helper to make a Hand.
makeHand :: (Card, Card, Card, Card, Card) -> Hand
makeHand (a, b, c, d, e) =
  Hand (NonEmpty.sortBy (flip compare) $ NonEmpty.fromList [a, b, c, d, e])

-- | A flush is a hand where every card is the same suit.
-- | This function returns Nothing if it is not a flush, and the hand itself if it is.
isFlush :: Hand -> Maybe Suit
isFlush (Hand (c :| cs)) =
  if all ((== suit') . suit) cs
    then Just suit'
    else Nothing
  where
    suit' = suit c

-- | A straight is a hand where every card is of ascending rank, suits ignored.
-- | This function returns Nothing if it is not a straight, and the highest Rank if it is.
isStraight :: Hand -> Maybe Rank
isStraight (Hand (c :| cs)) =
  let ranks = fmap rank (c : cs)
   in if List.reverse ranks `List.isInfixOf` rankOrder
        then Just $ rank c
        else Nothing

-- | Count rank occurrences.
rankCounts :: [Card] -> Map Rank Int
rankCounts h = Map.fromListWith (+) $ fmap (\c -> (rank c, 1)) h

instance Ord Hand where
  l `compare` r = patterns l `compare` patterns r
