/** Meant to rename all photos in the current directory starting from whatever
 * number is passed in.  The photo is renamed in the format of
 * "Prefix_####.EXT".  The Prefix is prompted from the user. It is followed by
 * an underscore, and then 4 digits denoting the number of the photo, followed
 * by the original extension of the photo (only if it is a type denoted in the
 * Filter class).
 */

import java.io.Console;
import java.io.File;

public class RenameAll {
  public static void main(String[] args) {
    // ask what the prefix should be changed to
    Console console = System.console();
    String newPrefix = console.readLine("What should the photos be renamed to? ");
 
    // ask what number the renamed files should start at?
    int index = Integer.parseInt(console.readLine("What number should the renamed photos start at? "));
 
    // get all files in the current directory
	  File currentDirectory = new File(".");
    File[] photos = currentDirectory.listFiles(new Filter());

    // for every file in the directory
    String newFileName;
    for (File file: photos) {
      newFileName = Utils.newFileName(file.getName(), newPrefix, index);
      file.renameTo(new File(newFileName));
      index++;
    }
  }

}
