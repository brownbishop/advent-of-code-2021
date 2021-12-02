module Main where

solve :: [Int] -> Int
solve list = length $ filter (<0) $ zipWith (-) list (tail list)

window :: [Int] -> [Int]
window l = zipWith3 (\x y z -> x + y + z) l (drop 1 l) (drop 2 l)

main :: IO ()
main = do
    rows <- lines <$> readFile "input"
    let measurements = map (read :: String -> Int) rows
    print $ (solve . window) measurements
