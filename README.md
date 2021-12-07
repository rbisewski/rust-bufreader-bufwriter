# Tangram BufReader and BufWriter example

As discussed with the tangram team, I would complete the assigned problem where I would attempt to do the following:

* Read from `stdin` using BufReader, one byte at a time
* Capitalize the character from lower case to upper case
* Write this result to `stdout` using BufWrite, one byte at a time

Included is a sample text file that looks like this:

```
tangram
interview
sample
file
```

After cloning the repo, this can all be run like so:

```bash
cat sample.txt | cargo run
```

The expected output looks like this, making the letters uppercase and also printing the newlines:

```
TANGRAM
INTERVIEW
SAMPLE
FILE
```
