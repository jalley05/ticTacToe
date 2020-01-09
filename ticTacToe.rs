#![allow(non_snake_case)]

const XLEN: usize = 3;
const YLEN: usize = 3;
const NUMWINS: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Token
{
    Blank = 0,
    X = 1,
    O = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum WinType
{
    XWIN,
    OWIN,
    CATS,
    NONE,
}

// Rust, struct + impl = "class"
struct GameBoard
{
    board: [[Token; XLEN]; YLEN],

    winCheckTable: [[[u32; XLEN]; YLEN]; NUMWINS],
}

impl GameBoard 
{
    fn new() -> Self
    {
        GameBoard {
            // Init game board to 0s
            board: [[Token::Blank; XLEN]; YLEN],

            winCheckTable: [[[0; XLEN]; YLEN]; NUMWINS],
        }
    }

    fn initWinnerTable(&mut self)
    {
        let mut incr = 0;
        for x in 0..XLEN
        {
            for y in 0..YLEN
            {
                self.winCheckTable[incr][x][y] = 1;
            }
            incr = incr + 1;
        }

        for y in 0..YLEN
        {
            for x in 0..XLEN
            {
                self.winCheckTable[incr][x][y] = 1;
            }
            incr = incr + 1;
        }

        self.winCheckTable[incr][0][0] = 1;
        self.winCheckTable[incr][1][1] = 1;
        self.winCheckTable[incr][2][2] = 1;

        incr = incr + 1;
        self.winCheckTable[incr][0][2] = 1;
        self.winCheckTable[incr][1][1] = 1;
        self.winCheckTable[incr][2][0] = 1;

        for ii in 0..NUMWINS
        {
            for x in 0..XLEN 
            {
                println!("|{0}|{1}|{2}|", self.winCheckTable[ii][x][0],
                    self.winCheckTable[ii][x][1],
                    self.winCheckTable[ii][x][2]);
            }
            println!("");
        }
    }

    // Mark board, mutable self reference
    fn mark(&mut self, x: usize, y: usize, token: Token) -> bool
    {
        if (x > XLEN-1) || (y > YLEN-1)
        {
            //println!("Invalid space placement");
            return false;
        }

        if self.board[x][y] != Token::Blank
        {
            //println!("Occupied space!");
            return false;
        }

        // Else store the token
        self.board[x][y] = token;
        return true;
    }

    fn tokenToStr(&self, token: Token) -> &str
    {
        if token == Token::X
        {
            return "X";
        }
        else if token == Token::O
        {
            return "O";
        }
        else
        {
            return " ";
        }
    }

    fn printTheBoard(&self)
    {
        println!("=============");
        for y in 0..YLEN 
        {
            let s1 = self.tokenToStr(self.board[0][y]);
            let s2 = self.tokenToStr(self.board[1][y]);
            let s3 = self.tokenToStr(self.board[2][y]);

            println!("| {0} | {1} | {2} |", s1, s2, s3);
        }
        println!("=============");
    }

    fn checkWin(&self) -> (WinType, Token)
    {
        for ii in 0..NUMWINS
        {
            // Check again game winning patter "ii"
            let mut xcount = 0;
            let mut ocount = 0;
            for x in 0..XLEN 
            {
                for y in 0..YLEN
                {
                    // If this is a winning spot check for X or O
                    if self.winCheckTable[ii][x][y] == 1
                    {
                        if self.board[x][y] == Token::X
                        {
                            xcount = xcount + 1;
                        }
                        else if self.board[x][y] == Token::O
                        {
                            ocount = ocount + 1;
                        }
                    } 
                }
            }

            if xcount == 3
            {
                println!("X wins");
                return (WinType::XWIN, Token::X);
            }

            if ocount == 3
            {
                println!("O wins");
                return (WinType::OWIN, Token::O);
            }
        }

        // Check for CATS game. If no blanks are found and no winner before
        let mut found = 0;
        for x in 0..XLEN
        {
            for y in 0..YLEN
            {
                if self.board[x][y] == Token::Blank 
                {
                    // Blank found
                    found = 1;
                }
            }
        }

        // If a blank is found, we have no winner yet
        if found == 1
        {
            //println!("no winner");
            return (WinType::NONE, Token::Blank);
        }

        // We are here, we didn't find a winner or a blank tile left
        println!("Cats");
        return (WinType::CATS, Token::Blank);
    }
}


use std::io::{self, BufRead};

fn main()
{
    // Setup game board with empty spaces
    let mut gB = GameBoard::new();
    gB.initWinnerTable();

    let mut winnerFound = 0;

    let mut tokenTurn = Token::X;

    while winnerFound == 0
    {
        println!(" ");
        println!(" ");
        println!(" ");

        gB.printTheBoard();

        let playerStr = gB.tokenToStr(tokenTurn);
        println!("Player {0} Turn", playerStr);

        println!("Enter [X Y] coordinates");
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().expect("error1").expect("error2");

        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        if vec.len() != 2
        {
            println!("Invalid input, 2 coordinate not detected");
            continue;
        }

        let xspot: usize = match vec[0] {
            "0"     => 0,
            "1"     => 1,
            "2"     => 2,
            _       => 3,
        };

        if xspot == 3
        {
            println!("Detected invalid X value, try again");
            continue;
        }


        let yspot: usize = match vec[1] {
            "0"     => 0,
            "1"     => 1,
            "2"     => 2,
            _       => 3,
        };

        if yspot == 3
        {
            println!("Detected invalid Y value, try again");
            continue;
        }


        if false == gB.mark(xspot, yspot, tokenTurn)
        {
            println!("!!!!!!!!!!!!!!!!!!!!");
            println!("Invalid spot");
            continue;
        }

        // Check for winner
        let (winType, winner) = gB.checkWin();
        if winType != WinType::NONE
        {
            //println!("Wintype {0} winner {1}", winType as u32, winner as u32);
            winnerFound = 1;

            if winType == WinType::CATS
            {
                println!("CATS game, end");
            }
            else
            {
                println!("\n\n");
                println!("WINNER {0} {0} {0} {0} {0}", gB.tokenToStr(winner));
                gB.printTheBoard();
                println!("WINNER {0} {0} {0} {0} {0}", gB.tokenToStr(winner));
            }
        }

        // Toggle player
        if tokenTurn == Token::X
        {
            tokenTurn = Token::O;
        }
        else
        {
            tokenTurn = Token::X;
        }
    }
}
