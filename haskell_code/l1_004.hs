import Text.Printf (printf)

main = do
  num <- fmap read getLine :: IO Int
  printf "Celsius = %d\n" (5 * (num - 32) `quot` 9)