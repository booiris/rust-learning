import Control.Monad (replicateM_)

calSum n = ((n + 1) `div` 2) ^ 2 * 2 - 1

maxN :: Int -> Int
maxN x = 2 * (floor . sqrt $ fromIntegral (x + 1) / 2) - 1

customPrint step 1 label maxn = do
  replicateM_ ((maxn - 1) `div` 2) (putStr " ")
  putStrLn label
  customPrint (-step) (1 + step) label maxn
customPrint step n label maxn
  | abs n > maxn = return ()
  | otherwise = do
      replicateM_ ((maxn - n) `div` 2) (putStr " ")
      replicateM_ n (putStr label)
      putStrLn ""
      customPrint step (n - step) label maxn

main = do
  input <- getLine
  let (numStr : label : _) = words input
  let num = read numStr
  let key = maxN num
  customPrint 2 key label key
  print $ num - calSum key