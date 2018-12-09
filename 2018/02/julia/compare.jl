module Compare

function main()
    ids = []
    open("../input/input.txt") do file
        for line in eachline(file)
            id = map(x -> x[1], split(line, ""))
            for other in ids
                diff = other .!= id
                if sum(diff) == 1
                    # Print what's in common.
                    println(join(id[.!diff]))
                    # No labeled break in Julia yet.
                    # See also: https://github.com/JuliaLang/julia/issues/5334
                    @goto done
                end
            end
            push!(ids, id)
        end
        @label done
    end
end

main()

end
