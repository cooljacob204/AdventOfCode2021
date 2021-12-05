mod input;  

fn main() {
  part1();
  part2();
}

fn part1() {
  println!("Part 1");
  let mut gamma = 0b000000000000;
  let mut epsilon = 0b111111111111;

  for bit in 0..12 {
    if most_common_bit(&input::input(), bit).positive {
      // invert bit at position
      gamma = gamma ^ (1 << bit);
      epsilon = epsilon ^ (1 << bit);
    }

  }

  println!("gamma: {}, {:b}", gamma, gamma);
  println!("epsilon: {}, {:b}", epsilon, epsilon);
  println!("power consumption: {}", gamma * epsilon);
}

fn part2(){
  println!("Part 2");
  // let mut value = 0;
  let oxygen_rating = get_oxygen_rating(input::input(), 12);
  let c02_scrubber_rating = get_c02_scrubber_rating(input::input(), 12);
  println!("oxygen rating: {}, {:b}", oxygen_rating, oxygen_rating);
  println!("c02 rating: {}, {:b}", c02_scrubber_rating, c02_scrubber_rating);
  println!("life support rating: {}", c02_scrubber_rating as i32 * oxygen_rating as i32);
}

fn get_c02_scrubber_rating(input: Vec<i16>, total_bits: u8) -> i16{
  let mut input = input;

  for bit in (0..total_bits).rev() {
    if input.len() <= 1 {
      break;
    }

    let most_common = most_common_bit(&input, bit);
    let comparison = most_common.positive || most_common.evenly_spread;

    input = input.iter().filter(|x| get_bit(x, bit) != comparison).cloned().collect();
  }

  return input[0]
}

fn get_oxygen_rating(input: Vec<i16>, total_bits: u8) -> i16{
  let mut input = input;

  for bit in (0..total_bits).rev() {
    if input.len() <= 1 {
      break;
    }

    let most_common = most_common_bit(&input, bit);
    let comparison = most_common.positive || most_common.evenly_spread;

    input = input.iter().filter(|x| get_bit(x, bit) == comparison).cloned().collect();
  }

  return input[0]
}

struct MostCommonBit {
  evenly_spread: bool, 
  positive: bool,
}

fn most_common_bit(input: &Vec<i16>, bit: u8) -> MostCommonBit {
  let mut value = 0;

  for number in input {
    if get_bit(&number, bit) {
      value += 1;
    } else {
      value -= 1;
    }
  }

  if value > 0 {
    MostCommonBit {
      evenly_spread: false,
      positive: true,
    }
  } else if value < 0 {
    MostCommonBit {
      evenly_spread: false,
      positive: false,
    }
  } else {
    MostCommonBit {
      evenly_spread: true,
      positive: false,
    }
  }
}

fn get_bit(input: &i16, n: u8) -> bool {
  if n < 32 {
    input & (1 << n) != 0
  } else {
    false
  }
}