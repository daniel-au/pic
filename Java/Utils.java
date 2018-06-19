/** Utility class with static methods. */

import java.util.regex.*;

public class Utils {
  /**
   * getExtension() returns the extension of filename that is passed in
   */
  public static String getExtension(String fileName) {
		String extension = "";
    Pattern extensionPattern = Pattern.compile("\\.\\w+\\z");
    Matcher extensionMatcher = extensionPattern.matcher(fileName);
    if (extensionMatcher.find()) {
			extension = extensionMatcher.group();
    }
    return extension;
  }

  public static String fileName(String prefix, int index, String extension) {
    StringBuilder fileName = new StringBuilder(prefix);

		fileName.append('_');
		if (index < 10) {
			fileName.append("000");
		} else if (index < 100) {
			fileName.append("00");
		} else if (index < 1000) {
			fileName.append('0');
		} else {
            // no need to append anything - assumes 9,999 photos max
        }
		fileName.append(index);
		fileName.append(extension);
		return fileName.toString();
  }

  /**
   * newFileName() takes in the original fileName of the file, the prefix which
   * the photo should be renamed to, and the photo number.  From there, it
   * constructs the new filename and keeps the file extension.
   * @param originalFileName - the original filename of the photo, used to find
   * the file extension
   * @param prefix - the prefix to be renamed to
   * @param index - the number of the photo
   * @return the new filename Instead of using patterns and matchers, it
   * would have been easy to look at the last 4 or 5 characters of the filename
   * and see where the '.' was to get either the 3 or 4 character extension.
   */
  public static String newFileName(String originalFileName, String prefix, int index) {
    String extension = getExtension(originalFileName);
    return fileName(prefix, index, extension);
	}
}
