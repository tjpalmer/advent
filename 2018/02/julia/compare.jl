module Compare

function main()
    ids = []
    open("../input/input.txt") do file
        for line in eachline(file)
            id = map(x -> x[1], split(line, ""))
            match_count = length(id) - 1
            for other in ids
                common = other .== id
                if sum(common) == match_count
                    # Print what's in common.
                    println(join(id[common]))
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
