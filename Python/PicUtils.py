# Used to rename a batch of photos, or copy the ones specified in a file

# Copy assumes files are in the format 'Prefix_XXXX.YYY' where XXXX is the
# number and YYY is the file extension.

import sys
import os
import re
import shutil
from textwrap import dedent

# list of extensions that match photos and videos
extensionPattern = '\\.\\w+$'

# Utility methods
def createOneLineString(multiline):
    return dedent(multiline).replace('\n', ' ').strip()

def getPhotoNumber(fileName):
    """Returns the photo number from the filename."""
    numAndExtensionPattern = '_\\d+' + extensionPattern
    startIndex = re.search(numAndExtensionPattern, fileName).start() + 1
    endIndex = re.search(extensionPattern, fileName).start()
    return int(fileName[startIndex:endIndex])

def getExtension(fileName):
    """Returns the filetype extension form the filename."""
    extension = ''
    matcher = re.search(extensionPattern, fileName)
    if matcher:
        extension = matcher.group(0)
    return extension

def getFileNameWithoutExtension(fileName):
    """Returns the filename without the extension."""
    matcher = re.search(extensionPattern, fileName)
    if matcher:
        return fileName[:matcher.start()]
    return fileName

def isPhotoOrVideo(fileName):
    """Returns true if the file is a photo."""
    photoExtensions = ['.JPG', '.NEF', '.jpg', '.jpeg', '.MOV', '.CR2', '.MP4']
    return os.path.isfile(fileName) and getExtension(fileName) in photoExtensions

def getPhotoFiles():
    """Returns a sorted list of photos in the current directory."""
    files = list(filter(isPhotoOrVideo, os.listdir()))
    files.sort()
    return files

def newFileName(originalFileName, newPrefix, index):
    """Returns the replacement fileName for the original file."""
    return '{0}_{1:04d}{2}'.format(
        newPrefix,
        index,
        getExtension(originalFileName)
    )

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
    inputFile = input("""\nWhat file contains the photo numbers to be copied? Please use the full filename including the extension. Input \'.\' if default file \'Good Ones.txt\'\nFile Name:""")
    if inputFile == '.':
        inputFile = 'Good Ones.txt'

    # create folder for copied files
    folderName = getFileNameWithoutExtension(inputFile)
    if os.path.exists(folderName):
        print('{0} directory already exists.'.format(folderName))
    else: 
        os.makedirs(folderName)
        print('{0} directory created.'.format(folderName))

    copiedCount = 0
    # read in numbers and create a set
    f = open(inputFile, 'r')
    toCopy = set(int(line.strip()) for line in f.readlines())

    # iterate through all pictures in the directory and copy the ones that are
    # in the set to be copied
    photos = getPhotoFiles()
    for photo in photos:
        # if number in file is in the set to be copied
        if getPhotoNumber(photo) in toCopy:
            shutil.copy2(photo, folderName)
            print('Copied {0}.'.format(photo))
            copiedCount += 1
    print('Number of photos to be copied: {0}'.format(len(toCopy)))
    print('Number of photos successfully copied {0}'.format(copiedCount))

def rename():
    """
    Renames all photo and video files in the current directory.

    Prompts the user for the new photo title and starting index. Uses that as a
    prefix and incrementally renames each photo and video to have the new
    prefix, an underscore, the index, and original extension.
    """
    print('\nThis assumes there are fewer than 10,000 photos to rename')
    newPrefix = input('What should the photos be renamed to? (Input \'.\' if it matches the current directory): ')
    if newPrefix == '.':
        newPrefix = os.path.basename(os.path.abspath('.'))
        print('New prefix is {0}'.format(newPrefix))
    index = int(input('What number should the photos start at? '))
    photos = getPhotoFiles()
    for i in range(len(photos)):
        # TODO - check if new name for file is already a file, don't overwrite
        os.rename(photos[i], newFileName(photos[i], newPrefix, index + i))

def usage():
    """Prints usage message."""
    usageMessage = """This program is used for your batch of photos. It takes in one command line argument - either \'rename\' or \'copy\' to rename a batch of photos or copy the ones enumerated in a text file. Example: \'$ python3 PicUtils.py copy\'"""
    print(usageMessage)

def main():
    numArgs = len(sys.argv)
    if numArgs == 1:
        print('Too few command line arguments.\n')
        usage()
    elif numArgs == 2:
        if sys.argv[1] == 'rename':
            rename()
        elif sys.argv[1] == 'copy':
            copy()
        else:
            print('Incorrect command line arguments')
            usage()
    else:
        print('Too many command line arguments.')
        usage()

if __name__ == '__main__':
    main()
