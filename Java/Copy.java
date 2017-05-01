import java.io.File;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.FileNotFoundException;
import java.nio.file.Files;
import java.nio.file.Paths;
import static java.nio.file.StandardCopyOption.*;

public class Copy { 
  public static void main(String[] args) {
    System.out.println("Beginning Copy");
    File goodOnes = new File("Good Ones");
    boolean created = goodOnes.mkdir();
    if (created) {
      System.out.println("Good Ones directory created");
    }
    File currentDirectory = new File(".");

    int copiedCount = 0;
    int numLines = 0;

    // BufferedReader to read every line that contains the number of photo to be copied
    try {
      BufferedReader goodOnesText = new BufferedReader(new FileReader("good ones.txt"));

      String currentLine;
      int photoNumber;
      File photo;
      File rawPhoto;
      File jpgPhoto1;
      File jpgPhoto2;
      String actualPhotoName;

      // find the prefix of all photos from the folder
      // assumes that the folder name is the same name as the photo prefix
      String[] directories = currentDirectory.getAbsolutePath().split("/");
      String prefix = directories[directories.length - 2];

      // iterate through each line of good ones.txt, copy that file over to the Good Ones folder
      // deal with file extension - test to see which extension exists then use that one
      while ((currentLine = goodOnesText.readLine()) != null) {
        numLines++;
        photoNumber = Integer.parseInt(currentLine);

        rawPhoto = new File(Utils.fileName(prefix, photoNumber, ".NEF"));
        jpgPhoto1 = new File(Utils.fileName(prefix, photoNumber, ".JPG"));
        jpgPhoto2 = new File(Utils.fileName(prefix, photoNumber, ".jpg"));
        if (rawPhoto.exists()) {
          photo = rawPhoto;
        } else if (jpgPhoto1.exists()) {
          photo = jpgPhoto1;
        } else if (jpgPhoto2.exists()) {
          photo = jpgPhoto2;
        } else {
          System.out.println(prefix + "_" + photoNumber + " does not exist");
          continue;
        }
        actualPhotoName = photo.getName();
        Files.copy(Paths.get(actualPhotoName), Paths.get("Good Ones/" + actualPhotoName), REPLACE_EXISTING);
        System.out.println("Copied " + actualPhotoName);
        copiedCount++;
      }
    } catch (FileNotFoundException e) {
      System.out.println("good ones.txt not found");
      e.printStackTrace();
    } catch (IOException e) {
      System.out.println("Error when reading line from good\\ ones.txt");
      e.printStackTrace();
    } 
    System.out.println("Number of photos to be copied: " + numLines);
    System.out.println("Number of photos copied: " + copiedCount);
  }

}
