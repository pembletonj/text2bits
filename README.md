# text2bits
Takes a file filled with 0's and 1's in plain text and outputs the binary equivalent to another file.

Usage: $ text2bits (input file) (output file)

It treats "#" as the beginning of a comment and will not read anything to the right of the # on that line.
All other characters are ignored.
