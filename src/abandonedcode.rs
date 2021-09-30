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

       /*mod lib; 
use lib::Game;
impl Extension for lib::Game {
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
}*/

/*fn ai_get_random_move(&mut self) -> (String, String, Vec<i8>, Vec<i8>) {
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
}*/

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

                // Chose to avoid checking for check here, because it would be very computationally expensive
            //
            // This was the beginning of integrating check rules (and sorting out impossible moves)
            //let evennewervector : Vec<Vec<i8>> = vec![]
            /*for i in 0..new_new_position.iter().count() {
                    let saved_boardstate = self.board;
                    self.make_move(
                        &Game::convert_vec_to_string(&vec![
                            useful_hashmap[&myall_possible_moves[i].clone()].clone()
                        ])[0]
                            .clone(),
                        Game::convert_vec_to_string(&vec![myall_possible_moves[i].clone()])[0].clone(), false
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
            //let even_newer_converted_new_vector = Game::convert_vec_to_string(&evennewervector);

            impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}