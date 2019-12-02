main = do
  content <- readFile "sumfuel-input.txt"
  let contentLines = lines content
  let masses = map (read :: String -> Int) contentLines
  let fuels = map (findFuel) masses
  print (sum fuels)

findFuel mass = (div mass 3) - 2
