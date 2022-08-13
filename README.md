# disjoint-words

This program searches for words with disjoint sets of characters, given a word
list and a number of words.

This is inspired by
[a video of Matt Parker](https://www.youtube.com/watch?v=_-AfhLQfb6w),
where he searches for sets of 5 words, all of length 5, with 25 distinct
characters. He used the words from `words_alpha.txt` from following repository:
[https://github.com/dwyl/english-words](https://github.com/dwyl/english-words).

This repository contains a file `words_alpha_5.txt` with all words of length 5
from `words_alpha.txt`. To run the program, use:

    cargo run --release -- -n 5 words_alpha_5.txt
