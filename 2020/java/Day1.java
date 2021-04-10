import java.io.File;
import java.nio.file.Files;
import java.util.Arrays;
import java.util.stream.IntStream;

class Day1 {
    public static void main(String[] args) throws Exception {
        System.out.printf("Hi %s\n", args[0]);
        int[] numbers;
        try (var lineStream = Files.lines(new File(args[0]).toPath())) {
            numbers = lineStream.mapToInt(Integer::parseInt).toArray();
        }
        IntStream
            .range(0, numbers.length)
            .filter(i -> {
                var a = numbers[i];
                var optionalB = IntStream
                    .range(i + 1, numbers.length)
                    .filter(j -> a + numbers[j] == 2020)
                    .findFirst();
                optionalB.ifPresent(j -> System.out.println(a * numbers[j]));
                return optionalB.isPresent();
            })
            .findFirst();
    }
}
