use std::{collections::HashMap, env};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Args {
    /// Paste the result in the terminal
    Terminal,

    /// Copy the reuslt to your clipboard
    Clipboard,
}

impl Default for Args {
    fn default() -> Self {
        Self::Terminal
    }
}

fn main() {
    let args = Args::parse();

    let letter_lookup = {
        let mut letters = HashMap::with_capacity(44);

        letters.insert('a', ":a: ");
        letters.insert('b', ":b: ");
        letters.insert('c', ":regional_indicator_c: ");
        letters.insert('d', ":regional_indicator_d: ");
        letters.insert('e', ":regional_indicator_e: ");
        letters.insert('f', ":regional_indicator_f: ");
        letters.insert('g', ":regional_indicator_g: ");
        letters.insert('h', ":regional_indicator_h: ");
        letters.insert('i', ":regional_indicator_i: ");
        letters.insert('j', ":regional_indicator_j: ");
        letters.insert('k', ":regional_indicator_k: ");
        letters.insert('l', ":regional_indicator_l: ");
        letters.insert('m', ":regional_indicator_m: ");
        letters.insert('n', ":regional_indicator_n: ");
        letters.insert('o', ":o2: ");
        letters.insert('p', ":regional_indicator_p: ");
        letters.insert('q', ":regional_indicator_q: ");
        letters.insert('r', ":regional_indicator_r: ");
        letters.insert('s', ":regional_indicator_s: ");
        letters.insert('t', ":regional_indicator_t: ");
        letters.insert('u', ":regional_indicator_u: ");
        letters.insert('v', ":regional_indicator_v: ");
        letters.insert('w', ":regional_indicator_w: ");
        letters.insert('x', ":regional_indicator_x: ");
        letters.insert('y', ":regional_indicator_y: ");
        letters.insert('z', ":regional_indicator_z: ");
        letters.insert('1', ":one: ");
        letters.insert('2', ":two: ");
        letters.insert('3', ":three: ");
        letters.insert('4', ":four: ");
        letters.insert('5', ":five: ");
        letters.insert('6', ":six: ");
        letters.insert('7', ":seven: ");
        letters.insert('8', ":eight: ");
        letters.insert('9', ":nine: ");
        letters.insert('0', ":zero: ");
        letters.insert('?', ":question: ");
        letters.insert('!', ":exclamation: ");
        letters.insert('$', ":chart: ");
        letters.insert('£', ":chart: ");
        letters.insert('#', ":hash: ");
        letters.insert('*', ":asterisk: ");
        letters.insert('"', ":chart: ");
        letters.insert('@', ":middle_finger: ");
        letters
    };

    let word_lookup = {
        let mut words = HashMap::new();

        words.insert("moon", ":u6708: ");
        words.insert("having", ":u6709: ");
        words.insert("have", ":u6709: ");
        words.insert("has", ":u6709: ");
        words.insert("had", ":u6709: ");
        words.insert("vs", ":vs: ");
        words.insert("id", ":id: ");
        words.insert("ab", ":ab: ");
        words.insert("cl", ":cl: ");
        words.insert("100", ":100: ");
        words.insert("10", ":10: ");
        words.insert("cool", ":cool: ");
        words.insert("new", ":new: ");
        words.insert("free", ":free: ");
        words.insert("ok", ":ok: ");
        words.insert("1st", ":first_place: ");
        words.insert("2nd", ":second_place: ");
        words.insert("3rd", ":third_place: ");
        words.insert("first", ":first_place: ");
        words.insert("second", ":second_place: ");
        words.insert("third", ":third_place: ");
        words.insert("gey", ":people_hugging: :couple_mm: :kiss_mm: :point_right: :fist: :sweat_drops: :gay_pride_flag: :wedding: ");
        words.insert("steampunk", ":tophat:\n:face_with_monocle: ");
        words
    };

    let mut buffer = String::new();

    for arg in env::args().skip(1) {
        let lower = arg.to_ascii_lowercase();
        let words = lower.split(' ');

        for word in words {
            if let Some(replacement) = word_lookup.get(word) {
                buffer += replacement;
            } else {
                for c in word.chars() {
                    if let Some(replacement) = letter_lookup.get(&c) {
                        buffer.push_str(replacement);
                    }
                }
            }

            buffer += "\n";
        }
    }

    match args {
        Args::Terminal => {
            println!("{buffer}");
        },

        Args::Clipboard => {
            // TODO:
            // Use set_contents_for_duration so that it sticks around for longer on linux.
            // In the mean time we will call this and print to stdout
            match cli_clipboard::set_contents(buffer.clone()) {
                Ok(_) => {
                    println!("Copied to clipboard!");
                },

                Err(_) => {
                    println!("Unable to copy to clipboard!");
                    println!("{buffer}");
                },
            }
        },
    }
}
