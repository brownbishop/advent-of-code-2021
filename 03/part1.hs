module Main where

import Control.Monad
import Data.List.Split
import Data.List (transpose)

gamma :: [[Int]] -> Int
gamma = toDecimal . map mostCommon . transpose

epislon :: [[Int]] -> Int
epislon = toDecimal . map mostUncommon . transpose
    where mostUncommon n = 1 - (mostCommon n)

mostCommon :: [Int] -> Int
mostCommon l = if oneSum > zeroSum then 1 else 0
    where
        oneSum = sum $ filter (==1) l
        zeroSum = (length l) - oneSum

toListOfDigits :: String -> [Int]
toListOfDigits = map (read :: String -> Int) . tail . splitOn ""

toDecimal :: [Int] -> Int
toDecimal = sum . map decInd . zip [0..] . reverse
    where
        decInd (i, b) = b * (2 ^ i)

main = do
    rows <- lines <$> readFile "input"
    -- a number (in base 2) is represented as list of digits
    let numbers =  map toListOfDigits rows
    print $ gamma numbers * epislon numbers
