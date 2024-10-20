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
    pub piece: Piece_enum,
    pub color: Color,
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
    pub pieces: [[Option<Piece>; 8]; 8],
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

    pub fn select(&mut self, x: i32, y: i32) -> Option<Vec<[i32; 2]>> {
        if x < 8 && y < 8 {
            if self.pieces[x as usize][y as usize] == None {
                return None;
            } else {
                let mut v:Vec<[i32;2]> = Vec::<[i32;2]>::new();
                let selected_piece = self.pieces[x as usize][y as usize].unwrap();

                match selected_piece.piece{
                    Piece_enum::king => {
                        for i in -1..=1{
                            for j in -1..=1{
                                if Board::in_bounds(x + i, y + j){
                                    let square = self.pieces[(x+i) as usize][(y+j) as usize];
                                    if square == None 
                                    || (square !=None && square.unwrap().color!= selected_piece.color)  {
                                        v.push([x+i,y+j]);
                                    }
                                }
                            }
                        }
                    },
                    Piece_enum::bishop => {
                        for j in [[1,1],[1,-1],[-1,1],[-1,-1]]{
                            for i in 1..10{
                                if Board::in_bounds(x + i*j[0], y + i*j[1]){
                                    let square = self.pieces[(x+i*j[0]) as usize][(y+i*j[1]) as usize];
                                    if square == None {
                                        v.push([x+i*j[0],y+i*j[1]]);
                                    }
                                    if square != None{
                                        if square.unwrap().color == selected_piece.color{
                                            break;
                                        }else{
                                            v.push([x+i*j[0],y+i*j[1]]);
                                            break;
                                        }

                                    }
                                }
                            }
                    }
                    },
                    Piece_enum::knight => {
                        for j in [[2,1],[1,2]]{
                            for i in [[1,1],[1,-1],[-1,1],[-1,-1]]{
                                let square_x = x + j[0]*i[0];
                                let square_y = y+j[1]*i[1];
                                if Board::in_bounds(square_x, square_y){
                                    let square = self.pieces[square_x as usize][square_y as usize];
                                    if square == None{
                                        v.push([square_x,square_y]);
                                    }
                                }
                            }
                        }
                    },
                    Piece_enum::rook=> {
                        for j in [[1,0],[-1,0],[0,1],[0,-1]]{
                            for i in 1..10{
                                if Board::in_bounds(x + i*j[0], y + i*j[1]){
                                    let square = self.pieces[(x+i*j[0]) as usize][(y+i*j[1]) as usize];
                                    if square == None {
                                        v.push([x+i*j[0],y+i*j[1]]);
                                    }
                                    if square != None{
                                        if square.unwrap().color == selected_piece.color{
                                            break;
                                        }else{
                                            v.push([x+i*j[0],y+i*j[1]]);
                                            break;
                                        }

                                    }
                                }
                            }
                    }
                    },
                    Piece_enum::pawn => {
                        match selected_piece.color{
                            Color::black => {
                                    if Board::in_bounds(x, y+1){
                                        let square = self.pieces[x as usize][(y+1) as usize];
                                        if square == None{
                                            v.push([x,y+1]);
                                            if y==1{
                                                let square = self.pieces[x as usize][(y+2) as usize];
                                                if square == None{
                                                    v.push([x,y+2]);
                                                }
                                            }
                                        }
                                    }
                                    for i in [-1,1]{
                                        if Board::in_bounds(x + i, y+1){
                                            let square = self.pieces[(x + i) as usize][(y+1) as usize];
                                            if square!= None && square.unwrap().color==Color::white{
                                                v.push([x+i,y+1]);
                                            }
                                        }
                                    }  
                            },
                            Color::white => {

                                    if Board::in_bounds(x, y-1){
                                        let square = self.pieces[x as usize][(y-1) as usize];
                                        if square == None{
                                            v.push([x,y-1]);
                                            if y==6{
                                                let square = self.pieces[x as usize][(y-2) as usize];
                                                if square == None{
                                                    v.push([x,y-2]);
                                                }
                                            }
                                        }
                                    }
                                
                                    for i in [-1,1]{
                                        if Board::in_bounds(x + i, y-1){
                                            let square = self.pieces[(x + i) as usize][(y-1) as usize];
                                            if square!= None && square.unwrap().color==Color::black{
                                                v.push([x+i,y-1]);
                                            }
                                        }
                                    }  
                            },
                        }
                    },
                    Piece_enum::queen => {

                        for j in [[1,0],[-1,0],[0,1],[0,-1]]{
                            for i in 1..10{
                                if Board::in_bounds(x + i*j[0], y + i*j[1]){
                                    let square = self.pieces[(x+i*j[0]) as usize][(y+i*j[1]) as usize];
                                    if square == None {
                                        v.push([x+i*j[0],y+i*j[1]]);
                                    }
                                    if square != None{
                                        if square.unwrap().color == selected_piece.color{
                                            break;
                                        }else{
                                            v.push([x+i*j[0],y+i*j[1]]);
                                            break;
                                        }

                                    }
                                }
                            }
                }
                        for j in [[1,1],[1,-1],[-1,1],[-1,-1]]{
                            for i in 1..10{
                                if Board::in_bounds(x + i*j[0], y + i*j[1]){
                                    let square = self.pieces[(x+i*j[0]) as usize][(y+i*j[1]) as usize];
                                    if square == None {
                                        v.push([x+i*j[0],y+i*j[1]]);
                                    }
                                    if square != None{
                                        if square.unwrap().color == selected_piece.color{
                                            break;
                                        }else{
                                            v.push([x+i*j[0],y+i*j[1]]);
                                            break;
                                        }

                                    }
                                }
                            }
                        }
            },
        }
                return Some(v);
            }

        } else {
            return None;
        }
    }
    pub fn move_piece(&mut self,x:i32,y:i32,dx:i32,dy:i32){
        self.pieces[dx as usize][dy as usize] = self.pieces[x as usize][y as usize];
        self.pieces[x as usize][y as usize] = None;
    }
    pub fn in_bounds(x: i32, y: i32) -> bool {
        return x >= 0 && x < 8 && y >= 0 && y < 8;
    }
}
