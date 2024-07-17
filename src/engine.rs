use game::Game;

pub mod game;

pub fn render_game(game: &game::Game) {
    println!("Rendering game...");
    for i in 0..3 {
        let row: [game::movement::Move; 3] = game.board[i];
        let pos0: String = row[0].to_string();
        let text0 = if pos0 == " " {
            (i * 3 + 1).to_string()
        } else {
            pos0
        };
        let pos1: String = row[1].to_string();
        let text1 = if pos1 == " " {
            (i * 3 + 2).to_string()
        } else {
            pos1
        };
        let pos2: String = row[2].to_string();
        let text2 = if pos2 == " " {
            (i * 3 + 3).to_string()
        } else {
            pos2
        };
        println!("{} | {} | {}", text0, text1, text2);
    }
}

pub fn ask_input(game: &game::Game) {
    let player = game.current_player.to_string();
    println!("It's {}'s turn. Enter a number from 1 to 9. > ", player);
}

pub fn update_game(input: String, game: &game::Game) -> Result<Game, String> {
    let number = input.trim().parse::<usize>().unwrap();
    match number {
        1..=9 => Game::play(game, number),
        _ => Result::Err("Invalid input. Please enter a number from 1 to 9.".to_string()),
    }
}

pub fn check_winner(game: &game::Game) -> Option<game::state::State> {
    game.winner
}
