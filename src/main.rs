use std::{collections::HashMap, env};

fn main() {
    let lookup = {
        let mut lookup = HashMap::new();

        lookup.insert('a', ":a: ");
        lookup.insert('b', ":b: ");
        lookup.insert('c', ":regional_indicator_c: ");
        lookup.insert('d', ":regional_indicator_d: ");
        lookup.insert('e', ":regional_indicator_e: ");
        lookup.insert('f', ":regional_indicator_f: ");
        lookup.insert('g', ":regional_indicator_g: ");
        lookup.insert('h', ":regional_indicator_h: ");
        lookup.insert('i', ":regional_indicator_i: ");
        lookup.insert('j', ":regional_indicator_j: ");
        lookup.insert('k', ":regional_indicator_k: ");
        lookup.insert('l', ":regional_indicator_l: ");
        lookup.insert('m', ":regional_indicator_m: ");
        lookup.insert('n', ":regional_indicator_n: ");
        lookup.insert('o', ":regional_indicator_o: ");
        lookup.insert('p', ":regional_indicator_p: ");
        lookup.insert('q', ":regional_indicator_q: ");
        lookup.insert('r', ":regional_indicator_r: ");
        lookup.insert('s', ":regional_indicator_s: ");
        lookup.insert('t', ":regional_indicator_t: ");
        lookup.insert('u', ":regional_indicator_u: ");
        lookup.insert('v', ":regional_indicator_v: ");
        lookup.insert('w', ":regional_indicator_w: ");
        lookup.insert('x', ":regional_indicator_x: ");
        lookup.insert('y', ":regional_indicator_y: ");
        lookup.insert('z', ":regional_indicator_z: ");
        lookup.insert('1', ":one: ");
        lookup.insert('2', ":two: ");
        lookup.insert('3', ":three: ");
        lookup.insert('4', ":four: ");
        lookup.insert('5', ":five: ");
        lookup.insert('6', ":six: ");
        lookup.insert('7', ":seven: ");
        lookup.insert('8', ":eight: ");
        lookup.insert('9', ":nine: ");
        lookup.insert('0', ":zero: ");
        lookup.insert('?', ":question: ");
        lookup.insert('!', ":exclamation: ");
        lookup.insert('$', ":chart: ");
        lookup.insert('£', ":chart: ");
        lookup.insert('#', ":hash: ");
        lookup.insert('"', ":chart: ");
        lookup.insert('@', ":middle_finger: ");
        lookup
    };

    let mut buffer = String::new();

    for arg in env::args().skip(1) {
        for c in arg.to_ascii_lowercase().chars() {
            if let Some(regional_indicator) = lookup.get(&c) {
                buffer += regional_indicator;
            }
        }

        buffer += "\n";
    }

    println!("{buffer}");

    // TODO:
    // Use set_contents_for_duration so that it sticks around for longer on linux.
    // In the mean time we will call this and print to stdout
    match cli_clipboard::set_contents(buffer) {
        Ok(_) => {
            println!("Copied to clipboard!");
        }

        Err(_) => {
            // println!("{buffer}");
        }
    }
}
