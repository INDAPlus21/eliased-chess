// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use crate::lib::Game;
    use crate::lib::GameState;
    use crate::lib::Piece;
    use crate::lib::PieceType;
    use crate::lib::Color;

    // cargo test -- --nocapture --test-threads=1

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

    #[test]
    fn move_a_pawn() {
        let mut game = Game::new();
        Game::make_move(&mut game, &"A2".to_string(), "A3".to_string(), true);
        let a_boardstate =    [
            [
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Queen,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::King,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: Color::Black,
                }),
            ],
            [
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::Black,
                }),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [Some(Piece {
                piecetype: PieceType::Pawn,
                color: Color::White,
            }), None, None, None, None, None, None, None],
            [
                None,
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                }),
            ],
            [
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Queen,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::King,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: Color::White,
                }),
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: Color::White,
                }),
            ],
        ];
        Game::print(&game);
        assert_eq!(game.board, a_boardstate);
    }

    #[test]
    fn test_promotion() {
        let mut game = Game::new();
        game.board[0][0] = 
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: Color::White,
                });
        Game::set_promotion(&mut game, PieceType::Queen);
        assert_eq!(game.board[0][0], Some(Piece {
            piecetype: PieceType::Queen,
            color: Color::White,
        }));
    }

    #[test]
    fn test_check() {
        let mut game = Game::new();
        game.board = [[Some(Piece { piecetype: PieceType::Rook, color: Color::Black }), None, Some(Piece { piecetype: PieceType::Bishop, color: Color::Black }), None, Some(Piece { piecetype: PieceType::King, color: Color::Black }), Some(Piece { piecetype: PieceType::Bishop, color: Color::Black }), Some(Piece { piecetype: PieceType::Knight, color: Color::Black }), None], [Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Queen, color: Color::White }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None], [None, None, Some(Piece { piecetype: PieceType::Knight, color: Color::Black }), None, None, None, None, None], [Some(Piece { piecetype: PieceType::Queen, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black })], [None, Some(Piece { piecetype: PieceType::Rook, color: Color::Black }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None], [Some(Piece { piecetype: PieceType::Bishop, color: Color::White }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, None, None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White })], [None, Some(Piece { piecetype: PieceType::Rook, color: Color::White }), None, None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, Some(Piece { piecetype: PieceType::Rook, color: Color::White })], [None, Some(Piece { piecetype: PieceType::Knight, color: Color::White }), None, Some(Piece { piecetype: PieceType::King, color: Color::White }), None, Some(Piece { piecetype: PieceType::Bishop, color: Color::White }), Some(Piece { piecetype: PieceType::Knight, color: Color::White }), None]];
        Game::print(&game);
        game.color = Color::Black;
        let check = Game::check_check(&mut game);
        assert_eq!(true, check);
    }

    #[test]
    fn test_game_state() {
        let mut game = Game::new();
        assert_eq!(        game.state 
            , GameState::InProgress);
    }

    #[test]
    fn get_possible_moves_pawn() {
        let mut game = Game::new();
        let (irrelevant, possible_moves) = Game::get_possible_moves(&mut game, &vec![0, 1], true);
        println!("{:?}", irrelevant);
        assert_eq!(vec!["A6".to_string(), "A5".to_string()], irrelevant);
    }

    #[test]
    fn get_possible_moves_knight() {
        let mut game = Game::new();
        let (irrelevant, possible_moves) = Game::get_possible_moves(&mut game, &vec![1, 0], true);
        println!("{:?}", irrelevant);
        assert_eq!(vec!["A6".to_string(), "C6".to_string()], irrelevant);
    }

    #[test]
    fn test_print() {
        let mut game = Game::new();
        Game::print(&game);
        println!("Does it look right?");
    }

    #[test]
    fn test_it_yourself() {
        let mut game = Game::new();
        Game::play_the_game(&mut game);
    }
}
