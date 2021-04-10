import java.io.File;
import java.nio.file.Files;
import java.util.stream.IntStream;

class Day1 {
    public static void main(String[] args) throws Exception {
        int[] numbers;
        try (var lineStream = Files.lines(new File(args[0]).toPath())) {
            numbers = lineStream.mapToInt(Integer::parseInt).toArray();
        }
        var ab = IntStream
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
        System.out.printf("%s * %s == %s\n", ab[0], ab[1], ab[0] * ab[1]);
    }
}
