module Checksum

function main()
    count2 = 0
    count3 = 0
    open("../input/input.txt") do file
        for line in eachline(file)
            counts = zeros(256)
            for char in line
                counts[Int(char)] += 1
            end
            count2 += any(counts .== 2)
            count3 += any(counts .== 3)
        end
    end
    println(count2 * count3)
end

main()

end
