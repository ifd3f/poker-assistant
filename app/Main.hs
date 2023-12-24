module Main
  ( main
  ) where

import Poker

main :: IO ()
main = do
  print (R10 `Of` Spade)
