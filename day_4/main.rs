mod input;

fn main() {
  part1();
  // part2();
}

fn part1(){
  let mut boards = input::boards().iter().map(|b| Board::new(b.clone())).collect::<Vec<Board>>();

  for a_move in input::moves() {
    send_move_to_boards(&mut boards, a_move);
    let winner = find_winner(&boards);

    if winner != None {
      println!("winner!\n{:?}", boards[winner.unwrap()]);
      println!("value: {}", boards[winner.unwrap()].unmarked_number() * a_move as i32);
      break;
    }
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