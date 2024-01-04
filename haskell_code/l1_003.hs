import Data.Char (ord)
import Data.List (foldl')

getNum s num = foldl' (\ans now -> if ord now - ord '0' == num then ans + 1 else ans) 0 s

main :: IO ()
main = do
  ini <- getLine
  let results = [(num, result) | num <- [0 .. 9], let result = getNum ini num, result /= 0]
  mapM_ (\(num, result) -> putStrLn $ show num ++ ":" ++ show result) results