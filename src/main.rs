use std::io::{self, Write};

const WIN_PATTERNS: [[usize; 3]; 8] = [
	[0, 1, 2],
	[3, 4, 5],
	[6, 7, 8],
	[0, 3, 6],
	[1, 4, 7],
	[2, 5, 8],
	[0, 4, 8],
	[2, 4, 6]
];


fn check_win(board: &Vec<&str>) -> bool
{
	let mut win = false;

	for player in 1..3
	{
		let sym = if player==1 {"O"} else {"X"};

		for pattern in WIN_PATTERNS
		{
			if pattern.iter().all(|&i| board[i] == sym)
			{
				win = true;
			}
		}
	}

	return win;
}

fn main() {
	let mut board: Vec<&str> = vec![" "; 9];

	'game: loop
	{
		for player in 1..3
		{
			println!("
{}|{}|{}
-----
{}|{}|{}
-----
{}|{}|{}", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);

			let mut input = String::new();
			print!("Player {}: ", player);
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input).unwrap();

			let input_num: usize = input.trim().parse().unwrap();
			let input_num = input_num-1;

			if input_num > 9 && input_num < 1
			{
				println!("Invalid number!");
				continue;
			}

			if board[input_num] != " "
			{
				println!("Space already taken!");
			} else
			{
				board[input_num] = if player==1 {"O"} else {"X"};
			}
			let check = check_win(&board);
			
			if check {
				println!("
{}|{}|{}
-----
{}|{}|{}
-----
{}|{}|{}", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
				println!("We have a winner!");
				break 'game;
			}
		}
	}
}
