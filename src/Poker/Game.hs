{-# LANGUAGE TemplateHaskell #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

module Poker.Game
  ( GamePosition(..)
  , PlayerPosition(..)
  ) where

import Poker
import Control.Lens

data PlayerPosition hole = PlayerPosition
  { _pHole :: hole
  , _pStud :: [Card]
  } deriving (Eq, Show)

makeLenses ''PlayerPosition

data GamePosition = GamePosition
  { _gOtherPlayers :: [PlayerPosition Int]
  , _gThisPlayer :: PlayerPosition [Card]
  , _gCommunity :: [Card]
  } deriving (Eq, Show)

makeLenses ''GamePosition

-- knownExistingCards gp = [c | p <- gp^.gOtherPlayers, c <- p^.pStud] ++ gp^.gThisPlayer.pHole ++ gp^.gThisPlayer.pStud ++ gp^.gCommunity


