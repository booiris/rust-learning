import Data.List (foldl')

calculateMaxDifference :: [Int] -> Int
calculateMaxDifference a = foldl' (\ans d -> max ans (calculateForD d)) minBound [1 .. length a]
  where
    calculateForD :: Int -> Int
    calculateForD d
      | length a `mod` d == 0 =
          let (minn, maxn) = calculateMinMax d a
           in maxn - minn
      | otherwise = minBound

calculateMinMax :: Int -> [Int] -> (Int, Int)
calculateMinMax d a = foldl' process (maxBound, minBound) [0, d .. length a - 1]
  where
    process (mn, mx) i =
      let subArray = take d (drop i a)
          sm = sum subArray
       in (min mn sm, max mx sm)

chunksOf :: Int -> [a] -> [[a]]
chunksOf _ [] = []
chunksOf n lst = take n lst : chunksOf n (drop n lst)

main = do
  interact $ unlines . fmap (show . calculateMaxDifference . map read . words . head . tail) . chunksOf 2 . tail . lines