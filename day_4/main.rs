mod input;

fn main() {
  println!("Part 1");
  part1();
  println!("Part 2");
  part2();
}

fn part1(){
  let mut boards = input::boards().iter().map(|b| Board::new(b.clone())).collect::<Vec<Board>>();

  for a_move in input::moves() {
    send_move_to_boards(&mut boards, a_move);
    let winner = find_winner(&boards);

    if winner != None {
      println!("value: {}", boards[winner.unwrap()].unmarked_number() * a_move as i32);
      break;
    }
  }
}

fn part2(){
  let mut boards = input::boards().iter().map(|b| Board::new(b.clone())).collect::<Vec<Board>>();
  let mut winners = Vec::new();

  struct Winner {
    board: Board,
    winning_value: i32
  }

  impl PartialEq for Winner {
    fn eq(&self, other: &Self) -> bool {
      self.board.board == other.board.board &&
      self.board.state == other.board.state &&
      self.winning_value == other.winning_value
    }
  }

  for a_move in input::moves() {
    send_move_to_boards(&mut boards, a_move);
    let mut winner = find_winner(&boards);

    while winner != None {
      winners.push(
        Winner {
          board: boards[winner.unwrap()].clone(),
          winning_value: boards[winner.unwrap()].unmarked_number() * a_move as i32
        }
      );
      
      boards.remove(winner.unwrap());

      winner = find_winner(&boards);
    }
  }

  if winners.last() != None {
    println!("value: {}", winners.last().unwrap().winning_value);
  }
}

fn send_move_to_boards(boards: &mut Vec<Board>, a_move: i8) {
  for board in boards {
    board.provide_move(a_move);
  }
}

fn find_winner(boards: &Vec<Board>) -> Option<usize> {
  boards.iter().position(|b| b.has_won())
}

struct Board {
  board: [[i8; 5]; 5],
  state: [[bool; 5]; 5]
}

impl Board {
  fn new(board: [[i8; 5]; 5]) -> Self {
    Board {
      board: board,
      state: [[false; 5]; 5]
    }
  }

  fn provide_move(&mut self, a_move: i8) {
    for (index, row) in self.board.iter().enumerate() {
      for (index2, value) in row.iter().enumerate() {
        if value.clone() == a_move {
          self.state[index][index2] = true;
        }
      }
    }
  }

  fn has_won(&self) -> bool {
    for i in 0..5 {
      if !IntoIterator::into_iter(self.state[i]).any(|val| val.clone() == false) || !IntoIterator::into_iter(self.state.map(|row| row[i])).any(|val| val.clone() == false) {
        return true;
      }
    }

    false
  }

  fn unmarked_number(&self) -> i32 {
    let mut unmarked_numbers = 0;

    for (row_index, row) in self.state.iter().enumerate() {
      for (col_index, col) in row.iter().enumerate() {
        if !*col {
          unmarked_numbers += self.board[row_index][col_index] as i32;
        }
      }
    }

    unmarked_numbers
  }

  fn clone(&self) -> Self {
    Board {
      board: self.board.clone(),
      state: self.state.clone()
    }
  }
}

impl std::fmt::Debug for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "board\n\n")?;
    for row in &self.board {
      for cell in row {
        write!(f, "\t{} ", cell)?;
      }
      write!(f, "\n")?;
      write!(f, "\n")?;
    }

    write!(f, "state\n\n")?;
    for row in &self.state {
      for cell in row {
        write!(f, "\t{} ", cell)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}