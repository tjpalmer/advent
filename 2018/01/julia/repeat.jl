module Repeat
using DelimitedFiles

function main()
    values = round.(Int, readdlm("../input/input.txt"))
    prevs = Set()
    total = 0
    done = false
    while !done
        for value in values
            if total in prevs
                println(total)
                done = true
                break
            end
            union!(prevs, [total])
            total += value
        end
        # println("-")
    end
end

main()

end
