# BufReader and BufWriter example

This does the following:

* Read from `stdin` using BufReader, one byte at a time
* Capitalize the character from lower case to upper case
* Write this result to `stdout` using BufWrite, one byte at a time

Included is a sample text file that looks like this:

```
this
is
a
sample
file
```

After cloning the repo, this can all be run like so:

```bash
cat sample.txt | cargo run
```

The expected output looks like this, making the letters uppercase and also printing the newlines:

```
THIS
IS
A
SAMPLE
FILE
```
