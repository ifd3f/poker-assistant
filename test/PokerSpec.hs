module PokerSpec
  ( spec
  ) where

import Poker
import Test.Hspec

spec :: Spec
spec = do
  describe "compare hands" $ do
    it "works for case" $ do
      makeHand
           ( R10 `Of` Heart
           , RJ `Of` Heart
           , RQ `Of` Heart
           , RK `Of` Heart
           , RA `Of` Heart) `compare` makeHand
           ( R2 `Of` Heart
           , R2 `Of` Spade
           , R2 `Of` Club
           , RQ `Of` Heart
           , RQ `Of` Diamond) `shouldBe` GT

  describe "patterns" $ do
    it "works for royal flush of hearts" $ do
      patterns
        (makeHand
           ( R10 `Of` Heart
           , RJ `Of` Heart
           , RQ `Of` Heart
           , RK `Of` Heart
           , RA `Of` Heart)) `shouldBe`
        [ StraightFlush (RA `Of` Heart)
        , Flush Heart
        , Straight RA
        , High RA
        , High RK
        , High RQ
        , High RJ
        , High R10
        , HighSuit (RA `Of` Heart)
        , HighSuit (RK `Of` Heart)
        , HighSuit (RQ `Of` Heart)
        , HighSuit (RJ `Of` Heart)
        , HighSuit (R10 `Of` Heart)
        ]
    it "works for full house" $ do
      patterns
        (makeHand
           ( R2 `Of` Heart
           , R2 `Of` Spade
           , R2 `Of` Club
           , RQ `Of` Heart
           , RQ `Of` Diamond)) `shouldBe`
        [ FullHouse R2 RQ
        , Kind 3 R2
        , Kind 2 RQ
        , High RQ
        , High RQ
        , High R2
        , High R2
        , High R2
        , HighSuit (RQ `Of` Heart)
        , HighSuit (RQ `Of` Diamond)
        , HighSuit (R2 `Of` Spade)
        , HighSuit (R2 `Of` Heart)
        , HighSuit (R2 `Of` Club)
        ]
    it "works for high" $ do
      patterns
        (makeHand
           ( R2 `Of` Spade
           , R4 `Of` Spade
           , R6 `Of` Spade
           , R9 `Of` Spade
           , RK `Of` Diamond)) `shouldBe`
        [ High RK
        , High R9
        , High R6
        , High R4
        , High R2
        , HighSuit (RK `Of` Diamond)
        , HighSuit (R9 `Of` Spade)
        , HighSuit (R6 `Of` Spade)
        , HighSuit (R4 `Of` Spade)
        , HighSuit (R2 `Of` Spade)
        ]
