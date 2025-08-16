# Used to rename a batch of photos, or copy the ones specified in a file

# Copy assumes files are in the format 'Prefix_XXXX.YYY' where XXXX is the
# number and YYY is the file extension.

import os
import random
import re
import shutil
import string
import sys

# list of extensions that match photos and videos
EXTENSION_PATTERN = "\\.\\w+$"
NUM_AND_EXTENSION_PATTERN = "_\\d+\\.\\w+$"
PHOTO_EXTENSIONS = {".JPG", ".NEF", ".jpg", ".jpeg", ".MOV", ".CR2", ".MP4"}
PREFIX_LENGTH = 6


def get_photo_number(filename):
    """Returns the photo number from the filename."""
    # could probably just use os.path.splitext and then a simpler regex
    start_index = re.search(NUM_AND_EXTENSION_PATTERN, filename).start() + 1
    end_index = re.search(EXTENSION_PATTERN, filename).start()
    return int(filename[start_index:end_index])


def get_extension(filename):
    """Returns the filetype extension from the filename."""
    _, extension = os.path.splitext(filename)
    return extension


def get_filename_without_extension(filename):
    """Returns the filename without the extension."""
    filename_prefix, _ = os.path.splitext(filename)
    return filename_prefix


def is_photo_or_video(filename):
    """Returns true if the file is a photo."""
    return os.path.isfile(filename) and get_extension(filename) in PHOTO_EXTENSIONS


def get_photo_files():
    """Returns a sorted list of photos in the current directory."""
    files = [
        filename
        for filename in os.listdir()
        if is_photo_or_video(filename)
    ]
    files.sort()
    return files


def create_random_prefix(photos):
    """
    Creates an unused random prefix of PREFIX_LENGTH characters long

    It creates a random prefix and checks to make sure no photos have the prefix. If a photo has
    that prefix, it will generate another random prefix.
    """
    while True:
        random_prefix = "".join(
            [random.choice(string.ascii_letters) for i in range(0, PREFIX_LENGTH)]
        )
        for photo in photos:
            if photo.startswith(random_prefix):
                continue
        break
    return random_prefix


def new_filename(original_filename, new_prefix, index):
    """Returns the replacement filename for the original file."""
    if new_prefix != "":
        new_file_name = "{0}_{1:04d}{2}".format(
            new_prefix, index, get_extension(original_filename)
        )
    else:
        new_file_name = "{0:04d}{1}".format(index, get_extension(original_filename))
    return new_file_name


def rename_all_photos(prefix, index):
    photos = get_photo_files()
    i = 0
    while i < len(photos):
        os.rename(photos[i], new_filename(photos[i], prefix, index + i))
        i += 1


def copy():
    """
    Copies photos where the numbers match the photo numbers specified in a file.

    Prompts the user for the file that contains a list of photo numbers to be
    copied. It assumes that there is a single number on each line in the file.

    Creates a new directory with the name of the file minus the extension. The
    photo numbers specified in the file are stored in a set. Each photo in the
    directory is iterated over to see if it is to be copied. If it is, it is
    copied to the newly created directory.
    """
    # ask the user which file contains the photos to be copied
    input_file = input(
        "\nWhat file contains the photo numbers to be copied? Please use the full filename "
        "including the extension. Input \'.\' if default file \'Good Ones.txt\'\nFile Name:"
    )
    if input_file == ".":
        input_file = "Good Ones.txt"

    # create folder for copied files
    folder_name = get_filename_without_extension(input_file)
    if os.path.exists(folder_name):
        print("{0} directory already exists.".format(folder_name))
    else:
        os.makedirs(folder_name)
        print("{0} directory created.".format(folder_name))

    copied_count = 0
    # read in numbers and create a set
    f = open(input_file, "r")
    to_copy = set(int(line.strip()) for line in f.readlines())

    # iterate through all pictures in the directory and copy the ones that are
    # in the set to be copied
    photos = get_photo_files()
    for photo in photos:
        # if number in file is in the set to be copied
        if get_photo_number(photo) in to_copy:
            shutil.copy2(photo, folder_name)
            print("Copied {0}.".format(photo))
            copied_count += 1
    print("Number of photos to be copied: {0}".format(len(to_copy)))
    print("Number of photos successfully copied {0}".format(copied_count))


def rename():
    """
    Renames all photo and video files in the current directory.

    Prompts the user for the new photo title and starting index. Uses that as a
    prefix and incrementally renames each photo and video to have the new
    prefix, an underscore, the index, and original extension.
    """
    print("\nThis assumes there are fewer than 10,000 photos to rename")
    new_prefix = input(
        "What should the photos be renamed to? (Input '.' if it matches the current directory): "
    )
    if new_prefix == ".":
        new_prefix = os.path.basename(os.path.abspath("."))
        print("New prefix is {0}".format(new_prefix))

    index = int(input("What number should the photos start at? "))

    photos = get_photo_files()

    # rename to random prefix - prevents overwriting any file
    random_prefix = create_random_prefix(photos)
    rename_all_photos(random_prefix, index)

    # rename to new prefix
    rename_all_photos(new_prefix, index)


def usage():
    """Prints usage message."""
    usage_message = (
        "This program is used for your batch of photos. It takes in one command line argument - "
        "either \'rename\' or \'copy\' to rename a batch of photos or copy the ones enumerated in "
        "a text file. Example: \'$ python3 PicUtils.py copy\'"
    )
    print(usage_message)


def main():
    num_args = len(sys.argv)
    if num_args == 1:
        print("Too few command line arguments.\n")
        usage()
    elif num_args == 2:
        if sys.argv[1] == "rename":
            rename()
        elif sys.argv[1] == "copy":
            copy()
        else:
            print("Incorrect command line arguments")
            usage()
    else:
        print("Too many command line arguments.")
        usage()


if __name__ == "__main__":
    main()
