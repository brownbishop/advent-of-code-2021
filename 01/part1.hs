module Main where

solve :: [Int] -> [Int]
solve list = length $ filter (<0) $ zipWith (-) list (tail list)

main :: IO ()
main = do
    rows <- lines <$> readFile "input"
    let measurements = map (read :: Int) rows
    print $ solve measurements
