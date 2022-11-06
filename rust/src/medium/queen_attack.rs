#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    fn is_out_of_board(y: i32, x: i32) -> bool {
        x < 0 || y < 0 || x > 7 || y > 7
    }

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if Self::is_out_of_board(rank, file) {
            return None;
        }

        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    fn check_diagonal_attacks(&self, other: &Queen) -> bool {
        let rank = self.position.rank;
        let file = self.position.file;

        let top_left = if rank < file {
            (0, file - rank)
        } else {
            (rank - file, 0)
        };
        dbg!(top_left);

        let bottom_left = (rank + file, 0);
        dbg!(bottom_left);
        false
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.check_diagonal_attacks(other)
    }
}

#[test]
fn queens_on_the_same_diagonal_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());

    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    assert!(white_queen.can_attack(&black_queen));
}
