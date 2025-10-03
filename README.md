# Sorter
A micro-uitility for sorting files into directories based on their file names.
It is useful in situations where you frequently place files in one place, such as when downloading, but you want to move them to a new location afterwards.

# Usage
Create a `.sort` file in the directory that you would like to sort.
The sort file format is as follows:

```
identifying string -> path/to/destination/directory
another string -> another/destination/directory
```

The sort file is split `->`, then trims whitespace at the beginning and end of both parts.
