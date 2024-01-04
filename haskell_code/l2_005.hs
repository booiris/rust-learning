import Control.Monad (replicateM)
import Data.Map.Strict as Map

main = do
  n <- fmap read getLine
  pairs <- replicateM n getLine
  let map =
        Prelude.foldl
          ( \acc now ->
              let [id1, key, id2] = words now
               in Map.insert key (id1, id2) acc
          )
          Map.empty
          pairs
  _ <- getLine
  q <- fmap words getLine
  mapM_
    ( \key ->
        case Map.lookup key map of
          Just (id1, id2) -> putStrLn $ id1 ++ " " ++ id2
          Nothing -> putStrLn "Not found"
    )
    q