main = do
  interact $ unlines . fmap (\a -> if (a + 1) `mod` 3 == 0 || (a - 1) `mod` 3 == 0 then "First" else "Second") . Prelude.map read . tail . lines