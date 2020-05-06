#[derive(Debug, PartialEq, Clone)]
pub struct Stage {
  raw_array: [[i32; 8]; 8],
  turn: i32,
}

pub struct Position {
  x: usize,
  y: usize,
}

impl Position {
  pub fn validate_int(x: isize, y: isize) -> bool {
    if x < 0 || x >= 8 || y < 0 || y >= 8 {
      false
    } else {
      true
    }
  }

  pub fn validate(&self) -> bool {
    Position::validate_int(self.x as isize, self.y as isize)
  }

  pub fn to_string(&self) -> String {
    format!("x: {}, y: {}", self.x, self.y)
  }
}

struct Direction {
  x: isize,
  y: isize,
}
static ALL_DIRECTION: [Direction; 8] = [
  Direction { x: -1, y: -1 },
  Direction { x: -1, y: 0 },
  Direction { x: -1, y: 1 },
  Direction { x: 0, y: -1 },
  Direction { x: 0, y: 1 },
  Direction { x: 1, y: -1 },
  Direction { x: 1, y: 0 },
  Direction { x: 1, y: 1 },
];

impl Stage {
  pub fn new() -> Stage {
    let default_array = [
      [0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, -1, 1, 0, 0, 0],
      [0, 0, 0, 1, -1, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    Stage {
      raw_array: default_array,
      turn: 1,
    }
  }

  pub fn new_with_array(array: [[i32; 8]; 8], turn: i32) -> Stage {
    Stage {
      raw_array: array,
      turn: turn,
    }
  }

  pub fn put_disc(&mut self, pos: &Position) -> Result<(), String> {
    if self.raw_array[pos.x][pos.y] != 0 {
      return Err(format!("That position({}) is already put a disc", pos.to_string()));
    }

    let mut num_reversed_discs: u32 = 0;
    for dir in ALL_DIRECTION.iter() {
      let (new_x, new_y) = (pos.x as isize + dir.x, pos.y as isize + dir.y);
      if Position::validate_int(new_x, new_y) {
        let new_pos = Position {
          x: new_x as usize,
          y: new_y as usize,
        };
        let _ = self.reverse(&new_pos, &dir, &mut num_reversed_discs);
      }
    }

    if num_reversed_discs > 0 {
      self.raw_array[pos.x][pos.y] = self.turn;
      self.turn *= -1;
      Ok(())
    } else {
      Err(format!("No discs to revers"))
    }
  }

  fn reverse(&mut self, pos: &Position, dir: &Direction, num_reversed_discs: &mut u32) -> i32 {
    if self.raw_array[pos.x][pos.y] == 0 {
      return 0;
    }
    if self.raw_array[pos.x][pos.y] == self.turn {
      return self.turn;
    }
    let (new_x, new_y) = (pos.x as isize + dir.x, pos.y as isize + dir.y);
    if !Position::validate_int(new_x, new_y) {
      return 0;
    }

    let new_pos = Position {
      x: new_x as usize,
      y: new_y as usize,
    };
    if self.reverse(&new_pos, dir, num_reversed_discs) == self.turn {
      self.raw_array[pos.x][pos.y] *= -1;
      *num_reversed_discs += 1;
      self.turn
    } else {
      0
    }
  }

  pub fn can_put_disc(&self, pos: &Position) -> bool {
    let mut cloned_stage = self.clone();
    !cloned_stage.put_disc(pos).is_err()
  }

  pub fn is_pass(&self) -> bool {
    for x in 0..8 {
      for y in 0..8 {
        if self.can_put_disc(&Position{x: x, y: y}) {
          println!("x:{}, y:{}", x, y);
          return false;
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::Position;
  use super::Stage;

  #[test]
  fn initialized_stage() {
    let stage = Stage::new();
    assert_eq!(
      stage.raw_array,
      [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, 1, 0, 0, 0],
        [0, 0, 0, 1, -1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
      ]
    );
  }

  #[test]
  fn first_turn() {
    let mut stage = Stage::new();
    assert_eq!(stage.put_disc(&Position { x: 4, y: 5 }), Ok(()));
    assert_eq!(
      stage.raw_array,
      [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
      ]
    );
  }

  #[test]
  fn second_turn() {
    let mut stage = Stage::new();
    assert_eq!(stage.put_disc(&Position { x: 4, y: 5 }), Ok(()));
    assert_eq!(stage.put_disc(&Position { x: 5, y: 5 }), Ok(()));
    assert_eq!(
      stage.raw_array,
      [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, 1, 0, 0, 0],
        [0, 0, 0, 1, -1, 1, 0, 0],
        [0, 0, 0, 0, 0, -1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
      ]
    );
  }

  #[test]
  fn pass_detection() {
    let turn = 1;
    let stage = Stage::new_with_array(
      [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, -1, 1, 0, 0],
        [0, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
      ],
      turn,
    );
    assert!(stage.is_pass());
  }
}
