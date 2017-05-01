/** This class is just a class that implements the FileFilter interface so that
 * it only finds files that are of specific file extensions.
 */

import java.io.File;
import java.io.FileFilter;

public class Filter implements FileFilter {
  public boolean accept(File file){
    String extension = Utils.getExtension(file.getName());
    return (
      extension.equals(".JPG") ||
      extension.equals(".NEF") || // Nikon Raw
      extension.equals(".jpg") ||
      extension.equals(".MOV") || // Nikon Video
      extension.equals(".CR2")    // Canon Raw
    );
  }
}
