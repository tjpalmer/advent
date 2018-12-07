module Sum
using DelimitedFiles

values = readdlm("../input/input.txt")
println(sum(values))

end
