#[derive(Eq, PartialEq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

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

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.can_attack_by_diagonal(other) || self.can_attack_by_rank_or_file(other)
    }

    fn diag_top_left(rank: i32, file: i32) -> ChessPosition {
        if rank < file {
            ChessPosition {
                file: 0,
                rank: file - rank,
            }
        } else {
            ChessPosition {
                file: rank - file,
                rank: 0,
            }
        }
    }
    fn diag_bottom_left(rank: i32, file: i32) -> ChessPosition {
        if rank + file > 7 {
            ChessPosition {
                file: 7,
                rank: 7 - rank + file,
            }
        } else {
            ChessPosition {
                file: rank + file,
                rank: 0,
            }
        }
    }

    fn can_attack_by_diagonal(&self, other: &Queen) -> bool {
        let self_top_left = Queen::diag_top_left(self.position.rank, self.position.file);
        let other_top_left = Queen::diag_top_left(other.position.rank, other.position.file);
        let self_bottom_left = Queen::diag_bottom_left(self.position.rank, self.position.file);
        let other_bottom_left = Queen::diag_bottom_left(other.position.rank, other.position.file);

        self_top_left == other_top_left || self_bottom_left == other_bottom_left
    }

    fn can_attack_by_rank_or_file(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank || self.position.file == other.position.file
    }
}

#[test]
fn queens_on_the_same_diagonal_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());

    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    assert!(white_queen.can_attack(&black_queen));
}

#[test]
fn queens_on_the_same_rank_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());

    let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());

    assert!(white_queen.can_attack(&black_queen));
}

#[test]
fn queens_on_the_same_file_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());

    let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());

    assert!(white_queen.can_attack(&black_queen));
}
