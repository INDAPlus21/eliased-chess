mod lib;
use std::fmt;
use std::io;
use std::io::prelude::*;
mod test;

//use ai;

fn main() {
    // cargo test -- --nocapture --test-threads=1
    //console_log::init_with_level(Level::Debug);
    //info!("It works!");
    println!("{:?}", line!());
    let mut game = lib::Game::new();
    lib::Game::print(&game);
    println!("{:?}", game.color);
    let owncolor = game.color;
    //lib::Game::get_all_possible_moves(&mut game, &owncolor);
    //println!("{:?}", game.board);
    println!("{:?}", lib::Game::get_game_state(&game));
    //lib::Game::check_check(&mut game);
    //println!("{:?}", lib::Game::checkmate(&mut game));
    //lib::tests::get_possible_moves_pawn();
    //lib::Game::chess_ai(&mut game);
    //lib::Game::better_chess_ai(&mut game);
    //lib::Game::evaluate_board_state(&mut game);
    //lib::Game::gui();
    //lib::Game::play_the_game(&mut game);
    /*for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let integer1 = nums[0];
        let integer2 = nums[1];
        println!("{}", difference.abs());
    }*/
    lib::Game::get_possible_moves(&mut game, &vec![0, 1], false);
    //lib::Game::make_move(&mut game, "E6".to_string(), "E5".to_string());
    /*lib::Game::set_promotion(&mut game);
    lib::Game::print(&game);*/
    //println!("{:?}", lib::Game::convert_coordinates(vec![vec![4, 5]]));
    //println!("{:?}", lib::Game::set_promotion(&mut game));
    //println!("{:?}", lib::Game::make_move(&game));
    //println!("{:?}", game.board);
    //lib::Game::printforrealz();
    /*fn printforrealz() {
        println!("|:----------------------:|");
        for line in &game.board {
            print!("|");
            for piece in line {
                print!(" {:?} ", piece);
            }
            print!("|");
            println!();
        }
        println!("|:----------------------:|");
    }
    printforrealz();*/
}
