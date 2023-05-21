struct GameState {
    board: Grid2D,
    players: Vec<Player>
}

struct Grid2D {
    grid: Vec<Vec<Cell>>
}

enum Cell {
    Empty,
    Colored(Color)
}

enum Color {
    Red,
    Blue,
    Green,
    Yellow
}

struct Player {
    color: Color,
    shapes: Vec<Shape>
}

enum Shape {
    Todo // semi linked list idea sounds cool
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
