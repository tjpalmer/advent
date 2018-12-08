module Compare

function main()
    count2 = 0
    count3 = 0
    ids = []
    open("../input/input.txt") do file
        for line in eachline(file)
            id = reshape(map(x -> x[1], split(line, "")), (1, :))
            # for other in ids
            # end
            if length(ids) > 5
                println(size(id))
                ids_all = vcat(ids...)
                println(size(ids_all))
                println(ids_all)
                println(ids_all .== id)
                println(sum(ids_all .== id, dims=2))
                break
            end
            push!(ids, id)
        end
    end
end

main()

end
