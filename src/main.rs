use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt::Display;
use std::io::{self, Read};
#[derive(Debug)]
struct Square {
    corners: [i64; 4],
    recursive_count: i64,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.corners[0], self.corners[1], self.corners[2], self.corners[3])
    }
}

impl Hash for Square {
    /* hashing based on the sum of corners is necessary to preserve the A == B -> Hash(A) == Hash(B) property */
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash all pairs of numbers in the string to capture order
        let sum_corners: i64 = self.corners.iter().sum();
        sum_corners.hash(state)
    }
}

impl PartialEq for Square {
    /* manual implementation: we want squares to be equal if they have the same corners in the
    same relative order, but we don't care which corner is the starting corner. e.g., [1,3,4,2] is equal
    to [2, 1, 3, 4], [3, 4, 2, 1], [4, 2, 3, 1]. */ 

    fn eq(&self, other: &Self) -> bool {
        let sq = self.corners;
        let other_sq = other.corners;
        if sq == other_sq {
            return true;
        }
        for offset in 1..sq.len() {
            if sq.iter().eq((0..sq.len()).map(|idx| &other_sq[(idx + offset) % 4])) {
                return true;
            }
        }
        return false;
    }

}

impl Eq for Square {}

impl Square {
    fn new(nums:[i64 ; 4]) -> Square {
        Square { corners: nums, recursive_count: -1 }
    }
    // Calculate the next square in the recursive sequence.
    fn iterate_square(&self) -> Square {
        let corners = self.corners;
        Square::new([i64::abs(corners[0] - corners[1]), 
                           i64::abs(corners[1] - corners[2]),
                           i64::abs(corners[2] - corners[3]),
                           i64::abs(corners[3] - corners[0])])
    }
}

/* Gets user input for the value of each corner. Handle all the possible errors. */
fn get_corner(which_corner: &str) -> i64 {
    println!("\nWhat's the {} corner of your square?", which_corner);
    let mut buf = String::new();
    let mut corner: i64 = -1;
    while corner < 0 {
        let try_input = io::stdin().read_line(&mut buf);
        if let Err(_error) = try_input {
            println!("Invalid input. Try again please!\n");
            buf.clear();
            continue;
        }
        let try_corner = buf.trim().parse::<i64>();
        if let Err(_error) = try_corner {
            println!("This is not a valid number. Try again please!\n");
            buf.clear();
            continue;
        }
        if let Ok(int) = try_corner {
            if int < 0 {
                println!("The input must be nonnegative. Try again please!\n");
                buf.clear();
                continue;
            } 
            corner = int;    // success!
        }
        
    }
    corner
}

fn main() {
    let mut solved_squares: HashSet<Square> = HashSet::new();
    solved_squares.insert(Square { corners: [0,0,0,0], recursive_count: 1 });
    println!("Welcome to the Jane Street January Puzzle solver! We're going to ask you for the corners of 
             the square you want to recurse over one at a time. Each square generates a new square that 
             has corners which are the absolute value difference of each pair of corners in the original
             square. The value for a corner must be nonnegative, and the maximum value possible is {}.\n", i64::MAX);
    let corner1 = get_corner("first");
    let corner2 = get_corner("second");
    let corner3 = get_corner("third");
    let corner4 = get_corner("fourth");
    let solved = edge_recurse(Square::new([corner1,corner2,corner3,corner4]), &mut solved_squares);
                    
    println!("\nIt took {} squares to hit the base square.\n", solved);
}

fn edge_recurse(mut square: Square, squares:&mut HashSet<Square>) -> i64 {
    println!("\n{}", square);
    if let Some(sq) = squares.get(&square) {
        return sq.recursive_count;
    }
    let num_squares = 1 + edge_recurse(square.iterate_square(), squares);
    square.recursive_count = num_squares;
    squares.insert(square);
    num_squares
}
