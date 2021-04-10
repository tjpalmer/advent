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
        var founds = IntStream
            .range(0, numbers.length)
            .mapToObj(i -> {
                var a = numbers[i];
                return IntStream
                    .range(i + 1, numbers.length)
                    .map(j -> numbers[j])
                    .filter(b -> a + b == 2020)
                    .mapToObj(b -> new int[]{a, b})
                    .findAny()
                    .orElse(null);
            })
            .filter(ab2 -> ab2 != null)
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
