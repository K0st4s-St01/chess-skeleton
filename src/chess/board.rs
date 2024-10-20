use std::path::is_separator;

use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
pub enum Piece_enum {
    king,
    queen,
    bishop,
    knight,
    rook,
    pawn,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    black,
    white,
}
#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    piece: Piece_enum,
    color: Color,
}
impl Piece {
    fn new(p: Piece_enum, color: Color) -> Piece {
        Piece {
            piece: p,
            color: color,
        }
    }
}
pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
}
impl Board {
    pub fn new() -> Board {
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let mut rng = rand::thread_rng();

        for i in 0..=7 {
            pieces[i][1] = Some(Piece::new(Piece_enum::pawn, Color::black));
            pieces[i][6] = Some(Piece::new(Piece_enum::pawn, Color::white));
        }

        let mut pieces_left = vec![
            Piece_enum::king,
            Piece_enum::queen,
            Piece_enum::rook,
            Piece_enum::knight,
            Piece_enum::bishop,
            Piece_enum::rook,
            Piece_enum::knight,
            Piece_enum::bishop,
        ];
        pieces_left.shuffle(&mut rng);
        for i in 0..=7 {
            pieces[i][0] = Some(Piece::new(pieces_left[i], Color::black));
            pieces[i][7] = Some(Piece::new(pieces_left[i], Color::white));
        }
        Board { pieces: pieces }
    }

    pub fn select(&mut self, x: usize, y: usize) -> Option<Vec<[usize; 2]>> {
        if x < 8 && y < 8 {
            if self.pieces[x][y] == None {
                return None;
            } else {
                let mut v = Vec::<[usize; 2]>::new();
                match self.pieces[x][y].unwrap().piece {
                    Piece_enum::king => {
                        for i in -1..=1 {
                            for j in -1..=1 {
                                let current =
                                    self.pieces[(x as i32 + i) as usize][(y as i32 + j) as usize];
                                if Board::in_bounds(x as i32 + i, y as i32 + j) {
                                    if current == None
                                        || (current != None
                                            && current.unwrap().color
                                                != self.pieces[x][y].unwrap().color)
                                    {
                                        v.push([(x as i32 + i) as usize, (y as i32 + j) as usize]);
                                    }
                                }
                            }
                        }
                        return Some(v);
                    }
                    Piece_enum::queen => todo!(),
                    Piece_enum::bishop => {
                        let mut v = Vec::<[usize; 2]>::new();
                        for i in x..=10 {
                            if Board::in_bounds((x + i) as i32, (y + i) as i32) {
                                let current = self.pieces[x + i][y + i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x + i, y + i]);
                                }
                            }
                            if Board::in_bounds((x + i) as i32, (y - i) as i32) {
                                let current = self.pieces[x + i][y - i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x + i, y - i]);
                                }
                            }
                            if Board::in_bounds((x - i) as i32, (y + i) as i32) {
                                let current = self.pieces[x - i][y + i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x - i, y + i]);
                                }
                            }
                            if Board::in_bounds((x - i) as i32, (y - i) as i32) {
                                let current = self.pieces[x - i][y - i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x - i, y - i]);
                                }
                            }
                        }

                        return Some(v);
                    }
                    Piece_enum::knight => {
                        let mut v = Vec::<[usize; 2]>::new();
                        for i in [
                            [2, 1],
                            [2, -1],
                            [1, 2],
                            [-1, 2],
                            [-1, -2],
                            [1, -2],
                            [-2, -1],
                            [-2, 1],
                        ] {
                            if Board::in_bounds(x as i32 + i[0], y as i32 + i[1]) {
                                let current = self.pieces[(x as i32 + i[0]) as usize]
                                    [(y as i32 + i[1]) as usize];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([
                                        (x as i32 + i[0]) as usize,
                                        (y as i32 + i[1]) as usize,
                                    ]);
                                }
                            }
                        }
                        return Some(v);
                    }
                    Piece_enum::rook => {
                        let mut v = Vec::<[usize; 2]>::new();
                        for i in 0..=10 {
                            if Board::in_bounds((x + i) as i32, (y) as i32) {
                                let current = self.pieces[x + i][y];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x + i, y]);
                                }
                            }
                            if Board::in_bounds((x - i) as i32, (y) as i32) {
                                let current = self.pieces[x - i][y];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x - i, y]);
                                }
                            }
                            if Board::in_bounds((x) as i32, (y + i) as i32) {
                                let current = self.pieces[x][y + i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x, y + i]);
                                }
                            }
                            if Board::in_bounds((x) as i32, (y - i) as i32) {
                                let current = self.pieces[x][y - i];
                                if current == None
                                    || (current != None
                                        && current.unwrap().color
                                            != self.pieces[x][y].unwrap().color)
                                {
                                    v.push([x, y - i]);
                                }
                            }
                        }
                        return Some(v);
                    }
                    Piece_enum::pawn => {
                        let mut v = Vec::<[usize; 2]>::new();
                        if self.pieces[x][y].unwrap().color == Color::white {
                            if y as i32 - 1 >= 0 {}
                            let current1 = self.pieces[x][y - 1];
                            if y == 6 {
                                let current = self.pieces[x][y - 2];
                                if current == None && current1 == None {
                                    v.push([x, y - 2]);
                                }
                            }
                            if current1 == None {
                                if Board::in_bounds(x as i32, y as i32 - 1) {
                                    v.push([x, y - 1]);
                                }
                            }

                            if x as i32 - 1 >= 0 {
                                let diagonal = self.pieces[x - 1][y - 1];
                                if diagonal != None && diagonal.unwrap().color == Color::black {
                                    v.push([x - 1, y - 1]);
                                }
                            }
                            if x + 1 < 8 {
                                let diagonal2 = self.pieces[x + 1][y - 1];
                                if diagonal2 != None && diagonal2.unwrap().color == Color::black {
                                    v.push([x + 1, y - 1]);
                                }
                            }
                        } else {
                            if y + 1 < 8 {}
                            let current1 = self.pieces[x][y + 1];
                            if (y == 1) {
                                let current = self.pieces[x][y + 2];
                                if current == None && current1 == None {
                                    v.push([x, y + 2]);
                                }
                            }
                            if current1 == None {
                                if Board::in_bounds(x as i32, y as i32 + 1) {
                                    v.push([x, y + 1]);
                                }
                            }

                            if x as i32 - 1 >= 0 {
                                let diagonal = self.pieces[x - 1][y + 1];
                                if diagonal != None && diagonal.unwrap().color == Color::black {
                                    v.push([x - 1, y + 1]);
                                }
                            }
                            if x + 1 < 8 {
                                let diagonal2 = self.pieces[x + 1][y + 1];
                                if diagonal2 != None && diagonal2.unwrap().color == Color::black {
                                    v.push([x + 1, y + 1]);
                                }
                            }
                        }
                        return Some(v);
                    }
                }
            }
        } else {
            return None;
        }
    }
    pub fn move_piece(&mut self,x:usize,y:usize,dx:usize,dy:usize){
        self.pieces[dx][dy] = self.pieces[x][y];
        self.pieces[x][y] = None;
    }
    fn in_bounds(x: i32, y: i32) -> bool {
        return x >= 0 && x < 8 && y >= 0 && x < 8;
    }
}
