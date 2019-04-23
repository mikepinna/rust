extern crate rayon;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use rayon::prelude::*;


fn make_bitmasks_box(a: &mut [[bool;81]]) {
    for n in 0..9 {
        let row_offset = (n / 3) * 9 * 3;
        let col_offset = (n % 3) * 3;
        let offset = row_offset + col_offset;
        let an = &mut a[n];
        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;
            an[offset + row*9 + col] = true;
        }
    }
}

fn make_bitmasks_vertical(a: &mut [[bool;81]]) {
    for n in 0..9 {
        let an = &mut a[n];
        for i in 0..9 {
            an[n + 9*i] = true;
        }
    }
}

fn make_bitmasks_horizontal(a: &mut [[bool;81]]) {
    for n in 0..9 {
        let an = &mut a[n];
        for i in 0..9 {
            an[9*n + i] = true;
        }
    }
}

fn make_bitmasks() -> [[bool;81];27] {
    let mut a = [[false;81];27];
    make_bitmasks_vertical(&mut a[0..9]);
    make_bitmasks_horizontal(&mut a[9..18]);
    make_bitmasks_box(&mut a[18..27]);
    a
}

fn print81<T,F>(a: &[T;81], f: F) where F : Fn(&T) -> char {
    for i in 0..81 {
        if i % 27 == 0 { print!("+---+---+---+"); }
        if i %  9 == 0 { print!("\n"); }
        if i %  3 == 0 { print!("|"); }

        print!("{}", f(&a[i]));
        
        if i %  9 ==  8 { print!("|")}
        if i % 27 == 26 { print!("\n")}
    }
    println!("+---+---+---+");
}

//fn print_bit_mask(a: &[bool;81]) {
//    print81(a, |b| if *b {'o'} else {'.'});
//}

fn print_board(board: &[Option<u8>;81]) {
    print81(&board, |o| match o {
        Some(0) => '1',
        Some(1) => '2',
        Some(2) => '3',
        Some(3) => '4',
        Some(4) => '5',
        Some(5) => '6',
        Some(6) => '7',
        Some(7) => '8',
        Some(8) => '9',
        None => ' ',
        _ => panic!("bad board")
    })
}

fn parse_board(s: &str) -> [Option<u8>;81] {
    let mut ret = [None;81];
    let mut counter = 0;

    for c in s.chars() {
        if c == ' ' || c == '0' {
            // no need to set this entry as is already set to None by default
            counter += 1;
        }
        else if let Some (d) = c.to_digit(10) {
            if d == 0 { panic!("zero not handled") }
            ret[counter] = Some ((d - 1) as u8);
            counter += 1;
        }
    }

    if counter != 81 {
        panic!("board did not contain 81 elements")
    }

    ret
}

fn board_is_valid_for_bitmask(board: &[Option<u8>;81], bitmask: &[bool;81]) -> bool {
    let mut values_seen = [false;9];
    let mut counter = 0;

    for i in 0..81 {
        if bitmask[i] {
            counter += 1;
            if let Some (n) = board[i] {
                let vs = &mut values_seen[n as usize];
                if *vs { return false; }
                *vs = true;
            }
        }
    }

    if counter != 9 { panic!("bitmask didn't contain 9 entries") }

    true
}

fn board_is_valid(board: &[Option<u8>;81], bitmasks: &[[bool;81];27]) -> bool {
    for bitmask in bitmasks {
        if !board_is_valid_for_bitmask(board, bitmask) { return false }
    }
    
    true
}

fn solve_board(board: &[Option<u8>;81], bitmasks: &[[bool;81];27]) -> Option<[Option<u8>;81]> {
    if !board_is_valid(board, bitmasks) {
        return None;
    }

    let mut first_empty = None;

    for i in 0..81 {
        if board[i] == None {
            first_empty = Some (i);
            break;
        }
    }

    if let Some (first_empty) = first_empty {
        let range: Vec<u8> = (0..9).collect();
        return 
            range.par_iter().map (|i| {
                let mut board_copy = board.clone();
                board_copy[first_empty] = Some (*i);
                solve_board(&board_copy, bitmasks)
            }).reduce_with(|r1, r2| { if r1.is_some() { r1 } else { r2 }}).unwrap();
    } else {
        return Some (board.clone());
    }
}

fn parse_boards(filename : String) -> std::io::Result<Vec<[Option<u8>;81]>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines: Vec<_> = contents.lines().collect();
    let grouped = lines.chunks_exact(10);
    
    let mut ret = Vec::new();
    for group in grouped {
        if !group[0].starts_with("Grid") {
            panic!("file format error");
        }
        ret.push(parse_board(&group[1..].join("\n")));
    }

    Ok(ret)
}

fn main() {
    let bitmasks = make_bitmasks();

    let boards = parse_boards("p096_sudoku.txt".to_string()).expect("board parse fail");

    let mut sum = 0;

    for board in boards {
        print_board(&board);
        if let Some (solved) = solve_board(&board, &bitmasks) {
            print_board(&solved);
            let a = solved[0].unwrap() as i32;
            let b = solved[1].unwrap() as i32;
            let c = solved[2].unwrap() as i32;
            let n = 100*(1+a) + 10*(1+b) + 1*(1+c);
            println!("n={}", n);
            sum += n;
        } else {
            panic!("Solver failed :(");
        }
    }

    println!("Final sum: {}", sum);

}