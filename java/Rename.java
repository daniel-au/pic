import java.util.Scanner;
import java.io.File;
import java.io.IOException;

public class Rename {
    public static void main(String[] args) {
        System.out.println("This assumes there are fewer than 10,000 photos to rename");

        File f;
        Scanner input = new Scanner(System.in).useDelimiter("\\n");

        System.out.print("What is the first photo number? ");
        int first = input.nextInt();
        System.out.print("What is the last photo number? ");
        int last = input.nextInt();
	      System.out.println("What number do you want the renamed photos to start at?");
	      int newNumber = input.nextInt();
        String original_prefix = System.console().readLine("What is the current prefix? (DSC)\n");
        String new_prefix = System.console().readLine("What should the prefix be changed to?\n");

        for (int i = 1; i <= last - first + 1; i++) {
            f = new File(fileName(original_prefix, i + first - 1));
            f.renameTo(new File(fileName(new_prefix, i + newNumber - 1)));
        }
    }

    private static String fileName(String prefix, int i) {
        StringBuilder name = new StringBuilder();
        name.append(prefix);
        name.append('_');
        if (i < 10) {
            name.append("000");
        } else if (i < 100) {
            name.append("00");
        } else if (i < 1000) {
            name.append("0");
        }
        name.append(i);
        name.append(".JPG");
        return name.toString();
    }
}
