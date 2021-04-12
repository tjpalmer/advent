import java.io.File;
import java.nio.file.Files;
import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class Day1b {
    public static void main(String[] args) throws Exception {
        int[] numbers;
        try (var lineStream = Files.lines(new File(args[0]).toPath())) {
            numbers = lineStream.mapToInt(Integer::parseInt).toArray();
        }
        // TODO Sort first to know when it's too high.
        // TODO Binary search on last group (or even earlier?).
        // TODO Generic Cartesian of n streams.
        var founds = IntStream
            .range(0, numbers.length)
            .mapToObj(i -> {
                var a = numbers[i];
                return IntStream
                    .range(i + 1, numbers.length)
                    .mapToObj(j -> {
                        var b = numbers[j];
                        return IntStream
                            .range(j + 1, numbers.length)
                            .map(k -> numbers[k])
                            .filter(c -> a + b + c == 2020)
                            .findAny()
                            .stream()
                            .mapToObj(c -> new int[]{a, b, c})
                            .findAny()
                            .orElse(null);
                    })
                    .filter(maybeFound -> maybeFound != null)
                    .findAny()
                    .orElse(null);
            })
            .filter(maybeFound -> maybeFound != null)
            .findAny()
            .orElseThrow();
        var product = Arrays.stream(founds).reduce(1, (a, b) -> a * b);
        var joined = Arrays
            .stream(founds)
            .mapToObj(String::valueOf)
            .collect(Collectors.joining(" * "));
        System.out.printf("%s == %s\n", joined, product);
    }
}
