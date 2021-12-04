module Main where

import Control.Monad
import Data.List.Split
import Data.List (transpose)

mostCommon :: [Int] -> Int
mostCommon l = if oneSum >= zeroSum then 1 else 0
    where
        oneSum = sum $ filter (==1) l
        zeroSum = length l - oneSum

toListOfDigits :: String -> [Int]
toListOfDigits = map (read :: String -> Int) . tail . splitOn ""

toDecimal :: [Int] -> Int
toDecimal = sum . map decInd . zip [0..] . reverse
    where
        decInd (i, b) = b * (2 ^ i)

-- sieve the numbers according to given criteria
sieve :: [[Int]] -> Int -> (Int -> Int -> Bool) -> [Int]
sieve [n] _ _ = n
sieve l pos comp = sieve filteredList nextPos comp
    where
        filteredList = filter isCommon l
        common = mostCommon $ map (!! pos) l
        isCommon = (comp common) . (!! pos)
        nextPos = pos + 1

oxigen :: [[Int]] -> Int
oxigen l = toDecimal $ sieve l 0 (==)

carbonDioxide :: [[Int]] -> Int
carbonDioxide l = toDecimal $ sieve l 0 (/=)

main = do
    rows <- lines <$> readFile "input"
    -- a number (in base 2) is represented as list of digits
    let numbers =  map toListOfDigits rows
    print $ oxigen numbers * carbonDioxide numbers
