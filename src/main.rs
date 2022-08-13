use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead, stdin};
use std::num::NonZeroUsize;
use std::thread::{self, available_parallelism};

use clap::{AppSettings, Arg, command};

mod tree;
use tree::{Tree, FastTree};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg_required_else_help(true)
        .setting(AppSettings::DeriveDisplayOrder)

        .arg(Arg::new("number")
            .short('n')
            .long("number")
            .value_name("N")
            .value_parser(clap::value_parser!(usize))
            .required(true)
            .help("Number of distinct words to search for"))

        .arg(Arg::new("naive")
            .short('N')
            .long("naive")
            .help("Use naive brute-force search"))

        .arg(Arg::new("tree")
            .short('T')
            .long("tree")
            .help("Use tree-based search (this is the default)"))

        .arg(Arg::new("threads")
            .short('j')
            .long("threads")
            .value_name("N")
            .value_parser(clap::value_parser!(usize))
            .help("Use N threads (defaults to number of CPU cores)")) 

        .arg(Arg::new("input")
            .index(1)
            .value_name("INPUT")
            .help("Input file."))

        .get_matches();

    let number = *matches.get_one::<usize>("number").unwrap();

    let naive = matches.contains_id("naive");

    let threads = matches.get_one::<usize>("threads").copied()
        .unwrap_or_else(|| available_parallelism()
            .map_or(1, NonZeroUsize::get));

    let input_file = matches.get_one::<String>("input").cloned()
        .unwrap_or_else(|| "-".to_string());



    let words = if input_file == "-" {
        stdin().lines()
            .collect::<Result<Vec<String>, _>>()?
    } else {
        BufReader::new(File::open(&input_file)?).lines()
            .collect::<Result<Vec<String>, _>>()?
    };
    eprintln!("{} words", words.len());

    let words_dist = words.into_iter()
        .filter(|s| distinct_letters(s))
        .collect::<Vec<_>>();
    eprintln!("{} words with distinct letters", words_dist.len());

    eprintln!("using {} threads", threads);

    if naive {
        find_words_naive(&words_dist, number, threads);
    } else {
        find_words_tree(&words_dist, number, threads);
    }

    Ok(())
}

fn distinct_letters(w: &str) -> bool {
    let mut chars = w.chars();
    while let Some(c) = chars.next() {
        if chars.clone().any(|c2| c == c2) {
            return false;
        }
    }
    true
}

fn find_words_naive(words: &[String], n: usize, threads: usize) {
    thread::scope(|scope| {
        for offset in 0..threads {
            scope.spawn(move || {
                for i in 0.. {
                    let i = offset + i * threads;
                    if i >= words.len() {
                        break;
                    }
                    find_words_naive_impl(&words[i + 1..], n - 1, &[&words[i]]);
                }
            });
        }
    });
}

fn find_words_naive_impl(mut words: &[String], n: usize, group: &[&String]) {
    if n == 0 {
        print_words(group);
        return;
    }
    let mut group = group.to_vec();
    while let Some(next) = words.first() {
        if group.iter().all(|w| !overlaps(w, next)) {
            group.push(next);
            find_words_naive_impl(&words[1..], n - 1, &group);
            group.pop();
        }
        words = &words[1..];
    }
}

fn overlaps(s1: &str, s2: &str) -> bool {
    for c1 in s1.chars() {
        if s2.contains(c1) {
            return true;
        }
    }
    false
}

fn print_words(words: &[&String]) {
    let mut words = words.iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>();
    words.sort();
    println!("{}", words.join(", "));
}

fn find_words_tree(words: &[String], n: usize, threads: usize) {
    let mut tree = Tree::new();
    for w in words {
        tree.insert(w.to_string());
    }
    //eprintln!("{:#?}", tree);

    let ftree = tree.build_fast_tree();
    //eprintln!("{:#?}", ftree);

    let root = &ftree;
    let nodes = root.get_all_word_nodes();
    eprintln!("{} nodes", nodes.len());

    thread::scope(|scope| {
        for offset in 0..threads {
            let nodes = &nodes;
            scope.spawn(move || {
                for i in 0.. {
                    let i = offset + i * threads;
                    if i >= nodes.len() {
                        break;
                    }
                    let node = nodes[i];
                    find_words_tree_impl(root, n - 1, &mut [node].to_vec(), &mut node.chars().to_vec(), root, node.chars().first().copied());
                }
            });
        }
    });
}

fn find_words_tree_impl<'a>(root: &'a FastTree, n: usize, group: &mut Vec<&'a FastTree>, letters: &mut Vec<char>, node: &'a FastTree, mut skip: Option<char>) {
    if n == 0 {
        print_tree_group(group, &mut Vec::new());
        return;
    }

    if !node.words().is_empty() {
        group.push(node);
        find_words_tree_impl(root, n - 1, group, letters, root, node.chars().first().copied());
        group.pop().unwrap();
    }

    for (ch, child) in node.childs() {
        if let Some(skip_ch) = skip {
            if ch != skip_ch {
                continue;
            }
            skip = None;
        }
        if !letters.contains(&ch) {
            letters.push(ch);
            find_words_tree_impl(root, n, group, letters, child, None);
            letters.pop().unwrap();
        }
    }
}

fn print_tree_group<'a>(group: &[&'a FastTree], words: &mut Vec<&'a String>) {
    if group.is_empty() {
        let mut words = words.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>();
        words.sort();
        println!("{}", words.join(", "));
    } else {
        for w in group[0].words() {
            words.push(w);
            print_tree_group(&group[1..], words);
            words.pop().unwrap();
        }
    }
}
