import Debug.Trace

main = print (findFuelDeep 1969)

mainMore = do
  content <- readFile "sumfuel-input.txt"
  let contentLines = lines content
  let masses = map (read :: String -> Int) contentLines
  let fuels = map (findFuelDeep) masses
  print (sum fuels)

findFuel mass = (div mass 3) - 2

findFuelDeep mass
  | fuelMass <= 0 = trace ("a " ++ show mass ++ ", " ++ show fuelMass) $ 0
  | otherwise = trace ("b " ++ show mass ++ ", " ++ show fuelMass) $ fuelMass + findFuelDeep fuelMass
  where fuelMass = findFuel mass
