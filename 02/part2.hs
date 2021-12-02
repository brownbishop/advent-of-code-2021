module Main where

import Control.Monad
import Data.List.Split

data Submarine = Submarine
    { hpos :: Int
    , depth :: Int
    , aim :: Int
    } deriving (Show, Eq)

emptySub = Submarine { hpos = 0, depth = 0, aim = 0 }

lineToInput :: String -> (String, Int)
lineToInput [] = ("", 0)
lineToInput line = (l !! 0, read $ l !! 1)
    where l = words line

simulate :: Submarine -> (String, Int) -> Submarine
simulate sub input
    | direction == "up" = Submarine
        { hpos = hpos sub
        , depth = depth sub
        , aim = aim sub - n
        }
    | direction == "down" = Submarine
        { hpos = hpos sub
        , depth = depth sub
        , aim = aim sub + n
        }
    | direction == "forward" = Submarine
        { hpos = hpos sub + n
        , depth = depth sub + n * aim sub
        , aim = aim sub
        }
    | otherwise = emptySub
  where
    direction = fst input
    n = snd input

main = do
    rows <- lines <$> readFile "input"
    let moves = map lineToInput rows
    let subFinal = foldl simulate emptySub moves
    print $ hpos subFinal * depth subFinal
