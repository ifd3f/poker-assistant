module Main
  ( main
  ) where

import Poker
import GHC.Arr
import qualified Data.List as List
import qualified Data.Aeson as Aeson
import qualified Data.ByteString.Lazy.Char8 as BSL

main :: IO ()
main = do
  let cards = List.sort allHands
  let json = Aeson.encode cards
  BSL.putStrLn json
  pure ()



