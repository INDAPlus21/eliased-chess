# eliased-chess

Observe that the AI isn't finished (and that that part of the code looks horrific), and that I haven't implement en passant, castling, nor deadposition. 

8 ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖ 8
7 ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙ 7
6 .  .  .  .  .  .  .  . 6
5 .  .  .  .  .  .  .  . 5
4 .  .  .  .  .  .  .  . 4
3 .  .  .  .  .  .  .  . 3
2 ♟︎  ♟︎  ♟︎  ♟︎  ♟︎  ♟︎  ♟︎  ♟︎ 2
1 ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜ 1
#-A--B--C--D--E--F--G--H-#

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &String, _to: String, changecolor: bool) -> GameState | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game. changecolor is whether the game.color should be changed or not by moving the piece. Positions are given as strings, e.g. "A1". |
| `pub fn set_promotion(&mut self, _piece: PieceType) -> ()` | Promote any potential pawns, and set the piece type that a peasant becames following a promotion. PieceType is e.g PieceType::Pawn |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&mut self, _position: &Vec<i8>, should_check: bool) -> (Vec<String>, Vec<Vec<i8>>)` | If a piece is standing on the given tile, return all possible new positions of that piece. should_check is whether it should sort away moves that result in check. Positions are given as a vector with the format vec![x position, y position], starting from zero from the left/top. |
| `pub fn play_the_game(&mut self)` | Plays the game in the terminal with string inputs. |
| `chess_ai(&mut self)` | A terrible AI that plays against itself. |
| `pub fn better_chess_ai(&mut self)` | A terrible AI that plays against itself while looking forward 2 steps (and crashes for some reason). |
| `fn checkmate(&mut self) -> bool` | Returns if it's checkmate or not. |
| `pub fn print(&self)` | Print the board in unicode. |

