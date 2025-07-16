"""Script meant for renaming files with a .JPG extension to .jpg."""

from pathlib import Path
from os import rename

PHOTO_EXTENSIONS = {".JPG"}

def rename_jpg_extensions():
    """Given that files are in JPG format, change the extension to .jpg."""
    curr_path_dir = Path(".")
    curr_dir_files = curr_path_dir.iterdir()
    for file_path in curr_dir_files:
        if file_path.suffix in PHOTO_EXTENSIONS:
            rename(file_path, file_path.stem + ".jpg")
            print(file_path)


if __name__ == "__main__":
    rename_jpg_extensions()
