// Import modules 
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piecetype: PieceType,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Queen,
    King,
    Knight,
    Bishop,
    Corpse,
}

// Defines a value() function for PieceTypes, which the AI uses   
impl PieceType {
    fn value(&self) -> i32 {
        match *self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100,
            _ => 0,
        }
    }
} 

pub struct Game {
    state: GameState,
    pub board: [[Option<Piece>; 8]; 8],
    pub color: Color,
}

impl Game {
    /// Initialises a new game with a board and the starting color white 
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
            color: Color::White,
            board: Game::generate_board(), 
        }
    }

    fn generate_board() -> [[Option<Piece>; 8]; 8] {
        let mut currentboard = [[None; 8]; 8];
        let pieces = [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::King, PieceType::Queen, PieceType::Bishop, PieceType::Knight, PieceType::Rook];

        // Fills the second and seventh row with pawns 
        for i in 0..8 {
            currentboard[1][i] = Some(Piece {
                piecetype: PieceType::Pawn,
                color: Color::Black,
            });
            currentboard[6][i] = Some(Piece {
                piecetype: PieceType::Pawn,
                color: Color::White,
            });
        }

        // Fills the first and last row with the right pieces
        for i in 0..8 {
            currentboard[0][i] = Some(Piece {
                piecetype: pieces[i],
                color: Color::Black,
            });
            currentboard[7][i] = Some(Piece {
                piecetype: pieces[i],
                color: Color::White,
            });
        }

        currentboard
    }

    fn convert_letter_coordinates(_position: String) -> Vec<i8> {
        //let mut vectorcoordinates : Vec<>= vec![];
        let mut coordinate_hashmap: HashMap<String, i8> = HashMap::new();
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        for i in 0..alphabet.iter().count() {
            coordinate_hashmap.insert(alphabet[i].to_string(), i as i8);
        }
        //println!("{:?}", coordinate_hashmap);
        let first_letter = &_position.chars().nth(0).unwrap();
        let first_letter: &char = &*first_letter;
        //println!("{:?}", first_letter);
        let second_letter = _position.chars().nth(1).unwrap().to_string();
        let second_letter: i8 = second_letter.trim().parse().unwrap();
        //println!("{:?}", second_letter);
        //println!("{:?}", my_num);
        let new_coordinate = vec![
            coordinate_hashmap[&first_letter.to_string()],
            8 - second_letter,
        ];
        //println!("{:?}", new_coordinate);
        return new_coordinate;
    }

    // Next step is check and GameOver (!)

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &String, _to: String, changecolor: bool) -> GameState {
        if Game::get_game_state(self) == GameState::InProgress {
            let piece_to_move = Game::convert_letter_coordinates(_from.to_string());
            let own_color = self.board[piece_to_move[1] as usize][piece_to_move[0] as usize]
                .unwrap()
                .color;
            let mut made_move = false;
            if own_color == self.color {
                let square_to_move_to = Game::convert_letter_coordinates(_to);
                /*println!(
                    "To move, move to: {:?} {:?}",
                    piece_to_move, square_to_move_to
                );*/
                let (_irrelevant, possible_moves) = Game::get_possible_moves(self, &piece_to_move);
                //let possible_movesvector = [];
                //println!("Possible: {:?}", possible_moves);
                for i in 0..possible_moves.iter().count() {
                    if square_to_move_to[0] == possible_moves[i][0]
                        && square_to_move_to[1] == possible_moves[i][1]
                    {
                        let ownpiecetype = self.board[piece_to_move[1] as usize]
                            [piece_to_move[0] as usize]
                            .unwrap()
                            .piecetype;
                        self.board[piece_to_move[1] as usize][piece_to_move[0] as usize] = None;
                        self.board[square_to_move_to[1] as usize][square_to_move_to[0] as usize] =
                            Some(Piece {
                                piecetype: ownpiecetype,
                                color: own_color,
                            });
                        made_move = true;
                    }
                }
                if !made_move {
                    println!("Invalid move!")
                } else {
                    //Game::print(&self);
                    if changecolor {
                        println!("Changing color");
                        // You changed this because it interfered with checkmate (!)
                        if self.color == Color::White {
                            self.color = Color::Black;
                        } else {
                            self.color = Color::White;
                        }
                    }
                }
            } else {
                println!("It's {:?}'s turn, you know", self.color)
            }
        }
        Game::set_promotion(self);
        Game::get_game_state(&self)
    }

    /*pub fn gui() -> Result<(), PlatformError>{
    {
               let main_window = WindowDesc::new(ui_builder);
               let data = 0_u32;
               AppLauncher::with_window(main_window)
                   .use_simple_logger()
                   .launch(data)
           }

           fn ui_builder() -> impl Widget<u32> {
               // The label text will be computed dynamically based on the current locale and count
               let text =
                   LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
               let label = Label::new(text).padding(5.0).center();
               let button = Button::new("increment")
                   .on_click(|_ctx, data, _env| *data += 1)
                   .padding(5.0);

               Flex::column().with_child(label).with_child(button)
           }
       }*/

    /*#[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PieceValues {

    }*/

    fn evaluate_board_state(&mut self) -> (i32, i32) {
        let mut white_value_sum = 0;
        let mut black_value_sum = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    {
                        let currentcolor = self.board[j as usize][i as usize].unwrap().color;
                        let piecetype = self.board[j as usize][i as usize].unwrap().piecetype;
                        let value = piecetype.value();
                        if currentcolor == Color::White {
                            white_value_sum += value;
                        } else {
                            black_value_sum += value;
                        }
                        //println!("{:?}", piecetype)
                    }
                }
            }
        }
        //println!("White: {:?}", white_value_sum);
        //println!("Black: {:?}", black_value_sum);
        return (white_value_sum, black_value_sum);
    }

    fn ai_get_random_move(&mut self) -> (String, String, Vec<i8>, Vec<i8>) {
        let owncolor = self.color;
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        let randommoveto = all_possible_moves.choose(&mut rand::thread_rng()).unwrap();
        let randommove = &very_useful_map[randommoveto]; //.clone()].clone()
                                                         //println!("A random move: {:?}", randommove);
                                                         //println!("A random move to: {:?}", randommoveto);
        let randommovestring = Game::convert_coordinates(&vec![randommove.to_vec()])[0].clone();
        //println!("A random move string: {:?}", randommovestring);
        let randommovetostring = Game::convert_coordinates(&vec![randommoveto.to_vec()])[0].clone();
        //println!("A random move to string: {:?}", randommovetostring);
        //println!("All possible: {:?}", very_useful_map);
        return (
            randommovestring,
            randommovetostring,
            randommove.to_vec(),
            randommoveto.to_vec(),
        );
    }

    fn ai_get_sequential_move(&mut self, i: usize) -> (String, String, Vec<i8>, Vec<i8>) {
        let owncolor = self.color;
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        //println!("{:?}", owncolor);
        let mut reali = i; 
        if i >= all_possible_moves.len() {
            reali = all_possible_moves.len() - 1
        }
        let randommoveto = &all_possible_moves[reali].clone();
        let randommove = very_useful_map[randommoveto].clone(); 
        let randommovestring = Game::convert_coordinates(&vec![randommove.to_vec()])[0].clone();
        //println!("A random move string: {:?}", randommovestring);
        let randommovetostring = Game::convert_coordinates(&vec![randommoveto.to_vec()])[0].clone();
        //println!("A random move to string: {:?}", randommovetostring);
        //println!("All possible: {:?}", very_useful_map);
        return (
            randommovestring,
            randommovetostring,
            randommove.to_vec(),
            randommoveto.to_vec(),
        );
    }

    /*pub fn eval_best_move(&mut self) {
        let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
        if self.color == Color::White && best_evaluation > black_value_sum {
            best_evaluation = black_value_sum;
            best_move = randommovestring;
            best_move_to = randommovetostring;
        } else if self.color == Color::Black && best_evaluation > white_value_sum {
            best_evaluation = white_value_sum;
            best_move = randommovestring;
            best_move_to = randommovetostring;
        }
        self.board = saved_boardstate;
    }

    pub fn make_actual_ai_move(&mut self) {
        println!("Hellooooo!");
        Game::print(self);
        //println!("testing board: {:?}", self.board);
        println!("Whose turn: {:?}", owncolor);
        println!("Best evaulation: {:?}", best_evaluation);
        println!("White: {:?}", white_value_sum);
        println!("Black: {:?}", black_value_sum);
        Game::make_move(self, &best_move, best_move_to.to_string(), true);
        Game::print(self);
    }*/

    fn get_data_of_opposite_color(&mut self) -> i32 {
        let mut owncolor = self.color;
        let mut best_evaluation = 150;
        let mut best_move: String = "A2".to_string();
        let mut best_move_to: String = "A3".to_string();
        let opposite_color = Game::opposite_func(owncolor);
        self.color = opposite_color; 
        let saved_boardstate = self.board;
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        for mut j in 0..all_possible_moves.iter().count() {
            let saved_boardstate = self.board;
            //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
            let (randommovestring, randommovetostring, randommove, randommoveto) =
                Game::ai_get_sequential_move(self, j);

            Game::make_move(
                self,
                &randommovestring,
                randommovetostring.to_string(),
                false,
            );

            // Evals best move 
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            if owncolor == Color::White && best_evaluation > black_value_sum {
                best_evaluation = black_value_sum;
                best_move = randommovestring;
                best_move_to = randommovetostring;
            } else if owncolor == Color::Black && best_evaluation > white_value_sum {
                best_evaluation = white_value_sum;
                best_move = randommovestring;
                best_move_to = randommovetostring;
            }

            self.board = saved_boardstate;
        }
        return best_evaluation
    }

    fn get_data_of_my_color(&mut self) {
        let mut owncolor = self.color;
        let mut best_evaluation = 150;
        let mut best_move: String = "A2".to_string();
        let mut best_move_to: String = "A3".to_string();
        let opposite_color = Game::opposite_func(owncolor);
        let saved_boardstate = self.board;
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        //let mut best_outcome_for_opposite = 150; 
        let all_equal_moves : Vec<i32> = vec![];
        for mut j in 0..all_possible_moves.iter().count() {
            let saved_boardstate = self.board;
            //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
            let (randommovestring, randommovetostring, randommove, randommoveto) =
                Game::ai_get_sequential_move(self, j);

            Game::make_move(
                self,
                &randommovestring,
                randommovetostring.to_string(),
                false,
            );

            // Evals best move 
            let best_outcome_for_opposite = Game::get_data_of_opposite_color(self);

            self.color = owncolor; 

            if best_evaluation > best_outcome_for_opposite {
                best_evaluation = best_outcome_for_opposite;
                best_move = randommovestring;
                best_move_to = randommovetostring;
            }

            self.board = saved_boardstate;
        }

        // Makes actual AI move
        println!("Hellooooo!");
        //Game::print(self);
        //println!("testing board: {:?}", self.board);
        println!("Whose turn: {:?}", owncolor);
        println!("Best evaulation: {:?}", best_evaluation);
        //println!("White: {:?}", white_value_sum);
        //println!("Black: {:?}", black_value_sum);
        Game::make_move(self, &best_move, best_move_to.to_string(), true);
        Game::print(self);
    }

    pub fn better_chess_ai(&mut self) {
        for mut i in 0..100 {
            Game::get_data_of_my_color(self);
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            if white_value_sum < 100 || black_value_sum < 100 {
                    println!("{:?} lost on Turn {:?}", self.color, i);
                    break;
            }
        }
    }

    fn chess_ai(&mut self) {
        for mut i in 0..200 {
            let mut owncolor = self.color;
            let mut checkmate = false;
            let mut best_evaluation = 150;
            let mut best_move: String = "A2".to_string();
            let mut best_move_to: String = "A3".to_string();
            let opposite_color = Game::opposite_func(owncolor);
            let saved_boardstate = self.board;
            let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
    
            for j in 0..all_possible_moves.iter().count() {
                let saved_boardstate = self.board;
                //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
                let (randommovestring, randommovetostring, randommove, randommoveto) =
                    Game::ai_get_sequential_move(self, j);
    
                Game::make_move(
                    self,
                    &randommovestring,
                    randommovetostring.to_string(),
                    false,
                );
    
                let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
                //println!("Current color : {:?}", owncolor);
                if self.color == Color::White && best_evaluation > black_value_sum {
                    best_evaluation = black_value_sum;
                    best_move = randommovestring;
                    best_move_to = randommovetostring;
                } else if self.color == Color::Black && best_evaluation > white_value_sum {
                    best_evaluation = white_value_sum;
                    best_move = randommovestring;
                    best_move_to = randommovetostring;
                }
    
                self.board = saved_boardstate;
                //self.color = owncolor;
                //self.color = opposite_color;
                //println!("Current color : {:?}", self.color);
                //Game::print(self);
    
                /*if Game::check_check(self) {
                    if Game::checkmate(self) {
                        println!("Checkmate!!!");
                        println!("{:?}'s turn", self.color);
                        i += 1000;
                        checkmate = true;
                    }
                    println!("Check!!!");
                    //i += 1000;
                    //self.board = saved_boardstate;
                }*/
                //if !checkmate {
                if j == all_possible_moves.iter().count() - 1 {
                    println!("Hellooooo!");
                    Game::print(self);
                    //println!("testing board: {:?}", self.board);
                    println!("Whose turn: {:?}", owncolor);
                    println!("Best evaulation: {:?}", best_evaluation);
                    println!("White: {:?}", white_value_sum);
                    println!("Black: {:?}", black_value_sum);
                    Game::make_move(self, &best_move, best_move_to.to_string(), true);
                    Game::print(self);
                }
            }
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            //println!({"{:?} to {:?}"}, best_move, best_move_to);
            //println!("{:?} {:?}", {"{:?}"}, &best_move);
            //println!("{:?} {:?}", {"{:?}"}, &best_move_to);
            //let hmm = best_move.clone();
    
            if white_value_sum < 100 || black_value_sum < 100 {
                println!("{:?} lost on Turn {:?}", self.color, i);
                break;
            }
        }
        //Game::min_max(self);
    }

    /// Set the piece type that a peasant becames following a promotion.
    // Differerent than Queen
    pub fn set_promotion(&mut self) -> () {
        // Why was there a string there?
        for mut i in 0..16 {
            let mut preffered_color = Color::White;
            let mut row_to_check = 0;
            if i > 7 {
                i -= 8;
                preffered_color = Color::Black;
                row_to_check = 7;
            }
            if self.board[row_to_check][i] != None
            /*|| self.board[i][j] == Some(Piece::Rook(Color::Black))*/
            {
                let own_color = self.board[row_to_check][i].unwrap().color;
                if self.board[row_to_check][i].unwrap().piecetype == PieceType::Pawn
                    && own_color == preffered_color
                {
                    let own_color = self.board[row_to_check][i].unwrap().color;
                    self.board[row_to_check][i as usize] = Some(Piece {
                        piecetype: PieceType::Queen,
                        color: own_color,
                    });
                }
            }
        }
    }

    pub fn play_the_game(&mut self) {
        let stdin = io::stdin();
        println!("White, enter your first move: ");
        let mut all_valid = vec![];
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        //println!("{:?}", "A".push_str(1.to_string()));
        for i in 1..9 {
            for j in 1..9 {
                all_valid.push(format!("{}{}", alphabet[i-1].to_string(), j.to_string())/*alphabet[i].to_string().push_str(&(i.to_string()))*/);
            }
        }
        println!("all_valid: {:?}", all_valid);
        for line in stdin.lock().lines() {
            let unwrapped = line.unwrap();
            let unwrapped: &str = &*unwrapped;
            if &unwrapped.len() > &(4 as usize) {
                let start_position = &unwrapped[0..2].to_uppercase(); //{ &"None".to_string() };
                let finalposition = &unwrapped[3..5].to_uppercase(); // &"None".to_string() };
                if all_valid.contains(&start_position) && all_valid.contains(&finalposition) {
                    //if start_position
                    //println!("{} {}", start_position, finalposition);
                    Game::make_move(
                        self,
                        &start_position.to_string(),
                        finalposition.to_string(),
                        true,
                    );
                    println!("{:?}, enter your move: ", self.color);
                } else {
                    println!("Invalid move! Enter new:")
                }
            } else {
                println!("Invalid move! Enter new:")
            }
        }
    }

    pub fn print(&self) {
        //let board = [['*'; 8]; 8];
        println!("#-A--B--C--D--E--F--G--H-#");
        let mut lineiter = 9;
        for line in self.board {
            lineiter -= 1;
            print!("{:?}", lineiter);
            for piece in line {
                if piece != None {
                    if piece.unwrap().color == Color::Black {
                        if piece.unwrap().piecetype == PieceType::Pawn {
                            print!(" {} ", "♙");
                        } else if piece.unwrap().piecetype == PieceType::Rook {
                            print!(" {} ", "♖");
                        } else if piece.unwrap().piecetype == PieceType::Knight {
                            print!(" {} ", "♘");
                        } else if piece.unwrap().piecetype == PieceType::Queen {
                            print!(" {} ", "♕");
                        } else if piece.unwrap().piecetype == PieceType::King {
                            print!(" {} ", "♔");
                        } else if piece.unwrap().piecetype == PieceType::Bishop {
                            print!(" {} ", "♗");
                        }
                    } else {
                        if piece.unwrap().piecetype == PieceType::Pawn {
                            print!(" {} ", "♟︎");
                        } else if piece.unwrap().piecetype == PieceType::Rook {
                            print!(" {} ", "♜");
                        } else if piece.unwrap().piecetype == PieceType::Knight {
                            print!(" {} ", "♞");
                        } else if piece.unwrap().piecetype == PieceType::Queen {
                            print!(" {} ", "♛");
                        } else if piece.unwrap().piecetype == PieceType::King {
                            print!(" {} ", "♚");
                        } else if piece.unwrap().piecetype == PieceType::Bishop {
                            print!(" {} ", "♝");
                        } else if piece.unwrap().piecetype == PieceType::Corpse {
                            print!(" {} ", "x"); //☠️
                        }
                    }
                } else {
                    print!(" . ");
                }
            }
            print!("{:?}", lineiter);
            println!();
        }
        println!("#-A--B--C--D--E--F--G--H-#");
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
        //println!("{:?}", self.state)
    }

    fn opposite_func(own_color: Color) -> Color {
        if own_color == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }

    fn get_all_possible_moves(
        &mut self,
        opposite_color: &Color,
    ) -> (Vec<Vec<i8>>, HashMap<Vec<i8>, Vec<i8>>) {
        let mut all_possible_moves = vec![];
        let mut coordinate_hashmap: HashMap<Vec<i8>, Vec<i8>> = HashMap::new();
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    if self.board[j as usize][i as usize].unwrap().color == *opposite_color {
                        let (_irrelevant, possible_moves) =
                            Game::get_possible_moves(self, &vec![i, j]);
                        for n in possible_moves {
                            coordinate_hashmap.insert(n.clone(), vec![i, j]);
                            all_possible_moves.push(n);
                        }
                    }
                }
            }
        }
        /*println!(
            "coordinate_hashmap coordinate_hashmap: {:?}",
            coordinate_hashmap
        );*/
        return (all_possible_moves, coordinate_hashmap);
    }

    fn convert_coordinates(_position: &Vec<Vec<i8>>) -> Vec<String> {
        let mut letter_coordinate_array = vec![];
        //let mut coordinate_hashmap : HashMap<Vec<i8>, String> = HashMap::new();
        let letter_array = ["A", "B", "C", "D", "E", "F", "G", "H"];
        for i in 0.._position.iter().count() {
            letter_coordinate_array.push(
                letter_array[_position[i][0] as usize].to_string()
                    + &(8 - _position[i][1]).to_string(),
            )
        }
        /*coordinate_hashmap.insert(
            vec![0,1],
            letter_array.to_string(),
        );*/
        //letter_coordinate_array.push(coordinate_hashmap[&vec![0, 1]].clone());
        return letter_coordinate_array;
    }

    fn get_king_position(&mut self) -> Vec<i8> {
        let mut king_position = vec![];
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    if self.board[j as usize][i as usize].unwrap().color == self.color
                        && self.board[j as usize][i as usize].unwrap().piecetype == PieceType::King
                    {
                        king_position = vec![i, j]
                    }
                }
            }
        }
        println!("{:?}", king_position);
        return king_position;
    }

    fn check_check(&mut self) -> bool {
        //let mut all_possible_moves = vec![];
        let own_color = self.color;
        let opposite_color = Game::opposite_func(own_color);
        let king_position = Game::get_king_position(self);
        /*for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                if self.board[j as usize][i as usize].unwrap().color == own_color && self.board[j as usize][i as usize].unwrap().piecetype == PieceType::King {
                    king_position = vec![i, j]
                }
            }
            }
        }
        println!("{:?}", king_position);*/
        let (all_possible_moves, _irrelevantmap) = self.get_all_possible_moves(&opposite_color);
        println!("All possible moves: {:?}", all_possible_moves);
        if all_possible_moves.contains(&king_position) {
            println!("CHESS! CHESS! CHESS!");
            self.state = GameState::Check;
            return true;
        } else {
            println!("NO CHESS! NO CHESS! NO CHESS!");
            return false;
        }
        /*for i in 0..all_possible_king_moves_letters.iter().count() {
            //self.board[_position[1] as usize][_position[0] as usize]
        }*/
        // This is god damn complicated
    }

    fn checkmate(&mut self) -> bool {
        //let king_position = Game::get_king_position(self);
        /*let (all_possible_king_moves_letters, _allpossiblekingmovescoordinates) =
        Game::get_possible_moves(self, &king_position);*/
        //println!("King moves: {:?}", all_possible_king_moves_letters);
        // change this !!!
        let own_color = self.color; //Color::Black;
        let (myall_possible_moves, useful_hashmap) = self.get_all_possible_moves(&own_color);
        println!("My possible moves: {:?}", myall_possible_moves);
        /*let king_positionstring: String =
        Game::convert_coordinates(&vec![king_position])[0].to_string();*/
        //println!("This is the realz");
        self.print();
        //println!("Possible letters: {:?}", all_possible_king_moves_letters);
        //println!("Possible moves: {:?}", all_possible_king_moves_letters[1]);
        let mut checkmate = true;
        for i in 0..myall_possible_moves.iter().count() {
            let saved_boardstate = self.board;
            //println!("{:?}", i);
            //let cloned = &all_possible_king_moves_letters[i];
            self.make_move(
                &Game::convert_coordinates(&vec![
                    useful_hashmap[&myall_possible_moves[i].clone()].clone()
                ])[0]
                    .clone(),
                Game::convert_coordinates(&vec![myall_possible_moves[i].clone()])[0].clone(),
                false,
            );
            //println!("Immidiately after move: ");
            //self.print();
            //self.make_move(&king_positionstring, cloned.to_string());
            //self.make_move(cloned.to_string(), king_positionstring);
            let check = Game::check_check(self);
            println!("{:?}", check);
            println!("Changed move: {:?}", myall_possible_moves[i]);
            println!("Non-Restored: ");
            self.print();
            self.board = saved_boardstate;
            if !check {
                checkmate = false;
            }
        }
        println!("checkmate: {:?} for {:?}", checkmate, self.color);
        if checkmate {
            self.state = GameState::GameOver
        }
        println!("testing board: {:?}", self.board);
        println!("{:?}", self.state);
        return checkmate;
    }

    // cargo test -- --nocapture --test-threads=1

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&mut self, _position: &Vec<i8>) -> (Vec<String>, Vec<Vec<i8>>) {
        //println!("{:?}", self.color);
        /*println!(
            "{:?}",
            self.board[_position[1] as usize][_position[0] as usize]
        );*/
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize]
                    == Some(Piece {
                        piecetype: PieceType::Corpse,
                        color: Color::White,
                    })
                {
                    self.board[j as usize][i as usize] = None
                }
            }
        }
        let my_piece = self.board[_position[1] as usize][_position[0] as usize];
        if my_piece == None {
            //println!("Do nothing");
            return (vec!["".to_string()], vec![vec![0, 1]]); //return option
        } else {
            let own_color = self.board[_position[1] as usize][_position[0] as usize]
                .unwrap()
                .color;
            let current_piecetype = self.board[_position[1] as usize][_position[0] as usize]
                .unwrap()
                .piecetype;
            //println!("{:?}", current_piecetype);
            let opposite_color = Game::opposite_func(own_color);

            fn add_function(
                current_vector: Vec<Vec<i8>>,
                mut possible_moves: Vec<Vec<i8>>,
                position: &Vec<i8>,
            ) -> Vec<Vec<i8>> {
                //println!("{:?}", current_vector.iter().count());
                for i in 0..current_vector.iter().count() {
                    if position[0] + current_vector[i][0] < 8
                        && position[0] + current_vector[i][0] >= 0
                        && position[1] + current_vector[i][1] >= 0
                        && position[1] + current_vector[i][1] < 8
                    {
                        possible_moves.push(vec![
                            position[0] + current_vector[i][0],
                            position[1] + current_vector[i][1],
                        ]);
                    }
                }
                return possible_moves;
            }

            // Next step is to figure out how to access the (1st) pawn color and (2nd, or maybe obvious once you figure out color) board state from within this function
            fn bishop_function(
                bishop_vector: &mut Vec<Vec<i8>>,
                board: &[[Option<Piece>; 8]; 8],
                color: &Color,
                position: &Vec<i8>,
            ) -> Vec<Vec<i8>> {
                let mut continue_xloop = true;
                for fakei in 0..32 {
                    if fakei == 8 || fakei == 16 || fakei == 24 {
                        continue_xloop = true;
                    }
                    let i = if fakei < 8 {
                        fakei
                    } else if fakei < 16 {
                        -(fakei - 8)
                    } else if fakei < 24 {
                        -(fakei - 16)
                    } else {
                        fakei - 24
                    };
                    let j = if fakei < 8 {
                        fakei
                    } else if fakei < 16 {
                        fakei - 8
                    } else if fakei < 24 {
                        -(fakei - 16)
                    } else {
                        -(fakei - 24)
                    };
                    //println!("{} {}", i, j);
                    if board[convert_usize(position[1], i)][convert_usize(position[0], j)] != None
                        && continue_xloop
                    {
                        //println!("Hello 4");
                        if board[convert_usize(position[1], i)][convert_usize(position[0], j)]
                            .unwrap()
                            .color
                            != *color
                        {
                            /*println!(
                                "Add captured white: {:?}",
                                vec![position[0] + j, position[1] + i]
                            );*/
                            bishop_vector.push(vec![position[0] + j, position[1] + i]);
                            //continue_xloop = false;
                            if i != 0 {
                                continue_xloop = false;
                            }
                        } else {
                            /*println!(
                                //"Found a black piece: {:?}",
                                vec![position[0] + i, position[1] + i]
                            );*/
                            if i != 0 {
                                continue_xloop = false;
                            }
                        }
                    } else if continue_xloop {
                        /*println!(
                            "Added empty: {:?}",
                            vec![position[0] + j, position[1] + i]
                        );*/
                        bishop_vector.push(vec![position[0] + j, position[1] + i]);
                    }
                    //bishop_vector.push(vec![i, i]);
                    //bishop_vector.push(vec![-i, i]);
                    //bishop_vector.push(vec![-i, -i]);
                }
                //println!("bishop_vector: {:?}", bishop_vector.to_vec());
                return bishop_vector.to_vec();
            }

            fn rook_function(
                possible_moves: &mut Vec<Vec<i8>>,
                position: &Vec<i8>,
                board: &[[Option<Piece>; 8]; 8],
                color: &Color,
            ) -> Vec<Vec<i8>> {
                let mut continue_xloop = true;
                let mut continue_yloop = true;
                let mut go_up_iter = 0;
                let mut go_up = true;
                for mut i in -7..8 {
                    if i == 0 {
                        continue_xloop = true;
                        continue_yloop = true;
                    }
                    //i = -i;
                    if go_up {
                        go_up_iter -= 1;
                        i = go_up_iter;
                    }
                    // I think it becomes a bug if there's a block on both x and y
                    if (go_up_iter > 8 || go_up_iter < -8) && go_up {
                        go_up_iter = 0;
                        go_up = false
                    }
                    if !go_up {
                        go_up_iter += 1;
                        i = go_up_iter;
                    }
                    //i = position + loopiterhmm
                    if position[0] + i < 8 && position[0] + i >= 0 && continue_xloop {
                        if board[position[1] as usize][convert_usize(position[0], i)] != None {
                            //println!("Hello 1");
                            //println!("{:?}", board[position[1] as usize]
                            //[position[0] as usize + i as usize]
                            //.unwrap()
                            //.color);
                            if board[position[1] as usize][convert_usize(position[0], i)]
                                .unwrap()
                                .color
                                != *color
                            {
                                //println!("Hello 2");
                                possible_moves.push(vec![position[0] + i, position[1]]);
                                if i != 0 {
                                    continue_xloop = false;
                                }
                            } else {
                                //println!("Hello 3");
                                if i != 0 {
                                    continue_xloop = false;
                                }
                            }
                        } else {
                            possible_moves.push(vec![position[0] + i, position[1]]);
                        }
                        // vec![vec![_position[0], _position[1] + 1], vec![_position[0], _position[1] + 2]]
                    }
                    //println!("{} {}", position[1] + i, position[0] as usize);
                    //println!("{:?}", board[0][3]);
                    if position[1] + i < 8 && position[1] + i >= 0 && continue_yloop {
                        if board[convert_usize(position[1], i)][position[0] as usize] != None {
                            //println!("Hello 4");
                            if board[convert_usize(position[1], i)][position[0] as usize]
                                .unwrap()
                                .color
                                != *color
                            {
                                //println!("Hello 5");
                                possible_moves.push(vec![position[0], position[1] + i]);
                                if i != 0 {
                                    continue_yloop = false;
                                }
                            } else {
                                //println!("Hello 6");
                                if i != 0 {
                                    continue_yloop = false;
                                }
                            }
                        } else {
                            //println!("Hello 7");
                            possible_moves.push(vec![position[0], position[1] + i]);
                        }
                        // vec![vec![_position[0], _position[1] + 1], vec![_position[0], _position[1] + 2]]
                    }
                }
                return possible_moves.to_vec();
            }

            fn convert_usize(possiblenegative: i8, othertoconvert: i8) -> usize {
                let sum = possiblenegative + othertoconvert;
                if sum < 0 || sum > 7 {
                    return 7 as usize;
                } else {
                    return sum as usize;
                }
            }

            let mut new_position = if current_piecetype == PieceType::Pawn {
                let to_add_one = if own_color == Color::Black { 1 } else { -1 };
                let to_add_two = if own_color == Color::Black { 2 } else { -2 };
                let start_position = if own_color == Color::Black { 1 } else { 6 };
                let promotion_position = if own_color == Color::Black { 7 } else { 0 };
                let mut possible_moves = vec![];
                if self.board[convert_usize(_position[1], to_add_one)][_position[0] as usize]
                    == None
                {
                    possible_moves.push(vec![_position[0], _position[1] + to_add_one]);
                }
                /*println!(
                    "Test: {:?}",
                    self.board[convert_usize(_position[1], to_add_two)][_position[0] as usize]
                );*/
                if _position[1] == start_position
                    && self.board[convert_usize(_position[1], to_add_two)][_position[0] as usize]
                        == None
                    && self.board[convert_usize(_position[1], to_add_one)][_position[0] as usize]
                        == None
                {
                    possible_moves.push(vec![_position[0], _position[1] + to_add_two]);
                    //vec![vec![_position[0], _position[1] + 1], vec![_position[0], _position[1] + 2]]
                }
                if _position[1] == promotion_position {
                    self.set_promotion()
                }
                //println!("{:?}", self.board[1][0].unwrap().piecetype);
                for i in 0..8 {
                    for j in 0..8 {
                        if self.board[i][j] != None
                        /*|| self.board[i][j] == Some(Piece::Rook(Color::Black))*/
                        {
                            if self.board[i][j].unwrap().color == opposite_color {
                                if i as i8 == _position[1] + to_add_one
                                    && (j as i8 == _position[0] + 1 || j as i8 == _position[0] - 1)
                                {
                                    //println!("Index {} {}", j, i);
                                    //println!("{:?}", self.board[i][j].unwrap());
                                    possible_moves.push(vec![j as i8, i as i8]);
                                }
                            }
                            // En passant below
                            /*if self.board[i][j].unwrap().color == opposite_color {
                                if j as i8 == 6 - start_position && j as i8 == _position[1] + to_add_one
                                    && i as i8 == _position[1] + to_add_one
                                {
                                    //println!("Index {} {}", j, i);
                                    //println!("{:?}", self.board[i][j].unwrap());
                                    possible_moves.push(vec![j as i8, i as i8]);
                                }
                            }*/
                        }
                    }
                }

                possible_moves
            } else if current_piecetype == PieceType::Rook {
                let mut possible_moves = vec![];
                possible_moves =
                    rook_function(&mut possible_moves, &_position, &self.board, &own_color);
                possible_moves
            } else if current_piecetype == PieceType::Knight {
                let mut possible_moves = vec![];
                /*let knight_vector = vec![-1, 1];
                let otherknight_vector = vec![-2, 2]
                for i in 0..2 {
                    possible_moves.push(vec![_position[0]+knight_vector[i], _position[1] + otherknight_vector[i]]);
                }*/
                let knight_vector = vec![
                    vec![1, 2],
                    vec![-1, 2],
                    vec![-1, -2],
                    vec![1, -2],
                    vec![2, -1],
                    vec![-2, 1],
                    vec![2, 1],
                    vec![-2, -1],
                ];
                possible_moves = add_function(knight_vector, possible_moves, &_position);
                possible_moves
            } else if current_piecetype == PieceType::King {
                let mut possible_moves = vec![];
                let kingvector = vec![
                    vec![1, 1],
                    vec![-1, -1],
                    vec![-1, 0],
                    vec![1, 0],
                    vec![0, -1],
                    vec![0, 1],
                    vec![1, -1],
                    vec![-1, 1],
                ];
                possible_moves = add_function(kingvector, possible_moves, &_position);
                possible_moves
            } else if current_piecetype == PieceType::Bishop {
                let mut bishop_vector = vec![];
                let possible_moves =
                    bishop_function(&mut bishop_vector, &self.board, &own_color, &_position);
                // You'll need to add limits for it
                //println!("{:?}", bishop_vector);
                //possible_moves = add_function(bishop_vector, possible_moves, &_position);
                possible_moves
            } else if current_piecetype == PieceType::Queen {
                let mut queen_vector = vec![];
                let mut possible_moves =
                    bishop_function(&mut queen_vector, &self.board, &own_color, &_position);
                //println!("{:?}", queen_vector);
                //possible_moves = add_function(queen_vector, possible_moves, &_position);
                //println!("{:?}", possible_moves);
                rook_function(&mut possible_moves, &_position, &self.board, &own_color);
                /*for i in 0..vectorfromrookqueen.iter().count() {
                    println!("hello");
                }*/
                //possible_moves.push(rook_function(&mut possible_moves, &_position, &self.board, &own_color));
                possible_moves
            } else {
                vec![vec![0, 3]]
            };
            new_position.retain(|x| !(x[0] == _position[0] && x[1] == _position[1]));
            new_position.dedup();
            //println!("{:?}", new_position);
            /*let mut newarrayiseasier = vec![];
            for someposition in new_position {
                if self.board[someposition[1] as usize][someposition[0] as usize] != None {
                    if self.board[someposition[1] as usize][someposition[0] as usize].unwrap().color != own_color {
                        newarrayiseasier.push(someposition);
                    }
                } else {
                    newarrayiseasier.push(someposition);
                }
            }*/
            // The Pawn test is maybe unneccessary
            let mut new_new_position: Vec<Vec<i8>> = vec![];
            for i in 0..8 {
                for j in 0..8 {
                    if new_position.contains(&vec![i, j]) {
                        //println!("hello 0");
                        if self.board[j as usize][i as usize] != None {
                            //println!("hello 0.5");
                            if self.board[j as usize][i as usize].unwrap().color != own_color {
                                //println!("hello 1");
                                new_new_position.push(vec![i, j]);
                            }
                        } else {
                            //println!("hello");
                            new_new_position.push(vec![i, j]);
                        }
                    }
                }
            }

            // Comment out the for and if below if you don't want corpses
            /*for i in 0..8 {
                for j in 0..8 {
                    if new_new_position.contains(&vec![i, j]) {
                        self.board[j as usize][i as usize] = Some(Piece {
                            piecetype: PieceType::Corpse,
                            color: Color::White,
                        });
                    }
                }
            }

            println!("{:?}", new_new_position);
            if new_new_position.iter().count() > 0 {
                Game::print(&self)
            }*/
            //let evennewervector : Vec<Vec<i8>> = vec![];
            let converted_new_vector = Game::convert_coordinates(&new_new_position);

            /*for i in 0..new_new_position.iter().count() {
                    let saved_boardstate = self.board;
                    self.make_move(
                        &Game::convert_coordinates(&vec![
                            useful_hashmap[&myall_possible_moves[i].clone()].clone()
                        ])[0]
                            .clone(),
                        Game::convert_coordinates(&vec![myall_possible_moves[i].clone()])[0].clone(), false
                    );
                    //println!("Immidiately after move: ");
                    //self.print();
                    //self.make_move(&king_positionstring, cloned.to_string());
                    //self.make_move(cloned.to_string(), king_positionstring);
                    let check = Game::check_check(self);
                    println!("{:?}", check);
                    self.board = saved_boardstate;
                    println!("Restored: ");
                    self.print();
                    if !check {
                        checkmate = false;
                    }
                }
            }*/
            //Game::checkmate(self);

            //let even_newer_converted_new_vector = Game::convert_coordinates(&evennewervector);
            return (converted_new_vector, new_new_position);

            // You need to check for checkmate in Get Possible Moves
        }
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|

/*impl Print for Game {
    none
}*/

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    /*fn move_a_pawn() {
        let game = Game::new();
        Game::make_move(&mut game, "A2".to_string(), "A3".to_string());
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }*/

    #[test]
    fn get_possible_moves_pawn() {
        let mut game = Game::new();
        let (irrelevant, possible_moves) = Game::get_possible_moves(&mut game, &vec![0, 1]);
        println!("{:?}", irrelevant);
        assert_eq!(vec!["A6".to_string(), "A5".to_string()], irrelevant);
    }

    #[test]
    fn get_possible_moves_knight() {
        let mut game = Game::new();
        let (irrelevant, possible_moves) = Game::get_possible_moves(&mut game, &vec![1, 0]);
        println!("{:?}", irrelevant);
        assert_eq!(vec!["A6".to_string(), "C6".to_string()], irrelevant);
    }
}