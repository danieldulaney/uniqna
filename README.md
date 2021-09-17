# uniqna

Just like `uniq`, but works when there are non-adjacent unique lines.

Suppose you have `fruits.txt` and you want the unique fruits:

```
$ cat fruits.txt
blueberry
apple
blueberry
apple
cherry
cherry
```

You could try passing it through `uniq`, but `uniq` only gets adjacent lines.

```
$ cat fruits.txt | uniq
blueberry
apple
blueberry
apple
cherry
```

To get the actually unique lines, use `uniqna`.

```
$ cat fruits.txt | uniqna
blueberry
apple
cherry
```

## Extra Features

Another useful feature is verbose mode, which prints out summary statistics to stderr every few lines. This is great for parsing very long streams with lots of repeated lines, because you can see how much progress has been made and get a sense for how much uniqueness there is.

For example, `yes` prints out an infinite sequence of lines containing `y`. Using `head` to get the first million lines, `uniqna` only prints out the first line, but continues to output statistics to stderr.

```
$ yes | head -n 1M | uniqna -v 100000
y
lines: 100000, uniques: 1, 0.00100% unique
lines: 200000, uniques: 1, 0.00050% unique
lines: 300000, uniques: 1, 0.00033% unique
lines: 400000, uniques: 1, 0.00025% unique
lines: 500000, uniques: 1, 0.00020% unique
lines: 600000, uniques: 1, 0.00017% unique
lines: 700000, uniques: 1, 0.00014% unique
lines: 800000, uniques: 1, 0.00012% unique
lines: 900000, uniques: 1, 0.00011% unique
lines: 1000000, uniques: 1, 0.00010% unique
```

## Limitations

Unlike `uniq`, `uniqna` has to keep track of every line it has seen so far. If every line is unique, that means keeping all of stdin in memory. This means that large streams with lots of uniqueness can cause issues. Small datasets and large datasets with lots of repetition work well.

## Why not use `sort | uniq`?

Two reasons:

First, speed. `uniqna` only keeps track of unique lines. If we get N lines, in the best case (every line is identical) that's just one line to track, but in the worst case (every line is unique) that's N lines to track. However, `sort | uniq` has to track every line no matter what -- every case is the worst case. If you have a multi-gigabyte stream with lots of repetition, `uniqna` will probably handle it fine, but `sort | uniq` will definitely be slower and might even OOM.

Second, semantics. `uniqna` preserves the order of the first time the line is seen. If that's meaningful for your use-case, `sort | uniq` won't work for you because it produces the lines in sorted order. If you want the `sort | uniq` semantics with the speed advantages of `uniqna`, consider `uniqna | sort`.
