# Sorter
A micro-uitility for sorting files into directories based on their file names.
It is useful in situations where you frequently place files in one place, such as when downloading, but you want to move them to a new location afterwards.

# Usage
Create a `.sort` file in the directory that you would like to sort.
The sort file format is as follows:

```
identifying regex -> path/to/destination/directory
another string -> another/destination/directory
```

Then, run `sorter` in that directory, or with that directory as an argument:

```
$ sorter Downloads
Moved: file with identifying regex.txt -> path/to/destination/directory/file with identifying regex.txt
Moved: another file with identifying regex.md -> path/to/destination/directory/another file with identifying regex.md
```

Each line of the sort file is split at the `->`, then the whitespace at the beginning and end of both parts is trimmed.
If a destination directory does not exist, it will be created.
If a file is not matched, it will not be touched.
If the `.sort` file does not exist, the program will crash.
