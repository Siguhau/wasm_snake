pub struct SnakeGame {
    width: i32,
    height: i32,
    snake: Vec<Position>,
    direction: Direction,
    new_direction: Direction,
    food: Position,
    game_win: bool,
    game_over: bool,
}

pub type Position = (i32, i3);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeGame {
    pub fn new(width: i32, height: i32) -> Self {
        let mut snake = Vec::new();
        snake.push((width / 2, height / 2));
        Self {
            width,
            height,
            snake,
            direction: Direction::Right,
            newDirection: Direction::Right,
            food: (0, height / 2),
            game_win: false,
            game_over: false,
        }
    }

    pub fn set_new_direction(&mut self, direction: Direction) {
        // check if the new direction is orthogonal to the current direction
        if self.direction == Direction::Up || self.direction == Direction::Down {
            if direction == Direction::Left || direction == Direction::Right {
                self.new_direction = direction;
            }
        } else if self.direction == Direction::Left || self.direction == Direction::Right {
            if direction == Direction::Up || direction == Direction::Down {
                self.new_direction = direction;
            }
        }
    }

    pub fn is_position_valid(&self, position: Position) -> bool {
        position.0 >= 0 && position.0 < self.width && position.1 >= 0 && position.1 < self.height
    }

    pub fn spawn_food(&mut self) {
        let mut available_positions = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if !self.snake.contains(&(x, y)) {
                    available_positions.push((x, y));
                }
            }
        }
        // if there are no available positions, the player has won the game
        if available_positions.is_empty() {
            self.game_win = true;
            self.game_over = true;
            return;
        }
        // set the food at random position from the available positions
        self.food = available_positions[rand::thread_rng().gen_range(0, available_positions.len())];
    }

    pub fn get_next_head(&self) -> Position {
        let mut new_head = self.snake[0];
        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }
        new_head
    }

    pub fn move_snake(&mut self) {
        let mut new_head = self.get_next_head();
        // if any of the new head coordinates are out of bounds, game over
        if !self.is_position_valid(new_head) {
            self.game_over = true;
            return;
        }
        // if the new head is on the food, grow the snake
        self.snake.insert(0, new_head);
        if new_head == self.food {
            self.snake.push(self.food);
            self.spawn_food();
        } else {
            self.snake.pop();
        }
        // if the new head is on the snake, game over
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }
    }

    pub fn game_tick(&mut self) {
        if self.game_over {
            return;
        }
        set_direction();
        move_snake();
    }
}
