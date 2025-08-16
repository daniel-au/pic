import java.io.File;
import java.io.IOException;
import java.io.Console;

/** Generates files with a 50% chance of having .NEF extension and 50% .JPG and
 * 20% chance of that file number not being created.  Used for testing my photo
 * renaming program.
 */
public class Generator {
  public static void main(String[] args) {
    Console console = System.console();
    String prefix = console.readLine("What should the photos be named to?");

    double rand;
    File file;
    String path;
    String extension;
    int numCreated = 0;
    for (int i = 1; i <= 500; i++) {
      rand = Math.random();
      if (rand > 0.1 && rand < 0.9) {
        extension = rand < 0.5 ? ".JPG" : ".NEF";
        path = Utils.fileName(prefix, i, extension);
        file = new File(path);
        try {
          file.createNewFile();
          numCreated++;
        } catch (IOException e) {
          System.out.println(path + " could not be created");
          e.printStackTrace();
        }
      } else {
        System.out.println("Did not create " + i);
      }
    }
    System.out.println("Total of " + numCreated + " files created.");
  }

}
