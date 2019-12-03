import qualified Data.Either as E
import qualified Data.Text as T
import qualified Data.Text.Read as R

eval ints index =
  let
    op = ints !! index
    a = ints !! (ints !! (index + 1))
    b = ints !! (ints !! (index + 2))
    result = case op of
      1 -> a + b
      2 -> a * b
      _ -> 0
  in replace ints (ints !! (index + 3)) result

evalMulti ints index =
  case (ints !! index) of
    99 -> ints !! 0
    _ -> evalMulti (eval ints index) (index + 4)

main = mainMore

mainLess = do
  -- let ints = [1,9,10,3,2,3,11,0,99,30,40,50]
  -- let ints = [1,0,0,0,99]
  -- let ints = [2,3,0,3,99]
  -- let ints = [2,4,4,5,99,0]
  let ints = [1,1,1,4,99,5,6,0,99]
  print ints
  print (evalMulti ints 0)

mainMore = do
  content <- readFile "input.txt"
  let words = T.splitOn (T.pack ",") (T.pack content)
  let ints = map (fst) (E.rights (map (R.decimal) words))
  let intsFixed = replace (replace ints 1 12) 2 2
  print (evalMulti intsFixed 0)

replace ints index value =
  let (a, _ : b) = splitAt index ints in
    a ++ value : b
