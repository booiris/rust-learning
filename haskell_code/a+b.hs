-- main = do
--   interact $ unlines . map (show . (\[a, b] -> a + b) . map read . words) . lines

main = do
  [a, b] <- fmap (map read . words) getLine
  print $ a + b