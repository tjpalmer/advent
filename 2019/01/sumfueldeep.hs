main = do
  content <- readFile "sumfuel-input.txt"
  let contentLines = lines content
  let masses = map (read :: String -> Int) contentLines
  let fuels = map (findFuelDeep) masses
  print (sum fuels)

findFuel mass = (div mass 3) - 2

findFuelDeep mass
  | fuelMass <= 0 = 0
  | otherwise = fuelMass + findFuelDeep fuelMass
  where fuelMass = findFuel mass
