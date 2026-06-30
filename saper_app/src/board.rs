use rand::Rng;
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Mine,
    Empty(u8), 
}

#[derive(Clone, Debug)]
pub struct Tile {
    cell: Cell,
    revealed: bool,
    flagged: bool,
}
impl Tile {
    pub fn revealed(&self) -> bool {
        self.revealed
    }

    pub fn flagged(&self) -> bool {
        self.flagged
    }

    pub fn cell(&self) -> &Cell {
        &self.cell
    }
}

pub struct Board {
    width: usize,
    height: usize,
    mines: usize,
    grid: Vec<Vec<Tile>>,
}

pub enum RevealResult {
    MineHit,
    Revealed,
    AlreadyRevealed,
    InvalidPosition,
}

impl Board {
    // nowa plansza
    pub fn new(width: usize, height: usize, mines: usize) -> Self{
        let grid = vec![vec![ Tile {cell: Cell::Empty(0),revealed: false, flagged: false}; width]; height];
        let mut board = Board{width, height, mines, grid};
        board.place_mines();
        board
    }

    // umieszczanie min i liczenie sąsiadów
    pub fn place_mines(&mut self){
        let mines = self.mines;
        let mut rng = rand::rng();
        let mut placed = 0;
        let mut positions: Vec<(usize, usize)> = Vec::new();

        while placed < mines {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);

            if let Cell::Mine = self.grid[y][x].cell {
                continue;
            }
            self.grid[y][x].cell = Cell::Mine;
            positions.push((x, y));
            placed += 1;
        }
        for (x, y) in positions {
            for k in -1..=1{
                for w in -1..=1{
                    let x_n = x as i32 + k;
                    let y_n = y as i32+ w;
                    if  x_n < 0 || y_n < 0  || (k == 0 && w == 0){
                        continue;
                    }
                    let x = x_n as usize;
                    let y = y_n as usize;
                    if self.width <= x ||  self.height <= y {
                        continue;
                    }
                    if let Cell::Mine = self.grid[y][x].cell {
                        continue;
                    }
                    let curr = match self.grid[y][x].cell {
                        Cell::Empty(n) => n,
                        _ => 0,
                    };
                    self.grid[y][x].cell = Cell::Empty(curr + 1);
                }
            }
        }
    }


    // odkrywanie pola - if mina oraz if 0
    pub fn reveal(&mut self, x: usize, y: usize) -> RevealResult {
        if self.width <= x || self.height <= y {
            return RevealResult::InvalidPosition;
        }
        let tile = &mut self.grid[y][x];
        if tile.revealed {
            return RevealResult::AlreadyRevealed;
        }
        // tile.revealed = true;
        match tile.cell {
            Cell::Mine => RevealResult::MineHit,
            Cell::Empty(0) => {
                self.dfs_visit(x, y);
                RevealResult::Revealed
            }
            Cell::Empty(_n) => {
                self.grid[y][x].revealed = true;
                RevealResult::Revealed
            }
        }
    }

    pub fn reveal_all(&mut self) {
        for row in &mut self.grid {
            for tile in row {
                tile.revealed = true;
            }
        }
    }
    
    // funkcja do odkrywania sąsiadów 0 - DFS
    fn dfs_visit(&mut self, x: usize, y: usize){
        if self.grid[y][x].revealed {
            return;
        }
        self.grid[y][x].revealed = true;

        for k in -1..=1{
            for w in -1..=1{
                if w == 0 && k == 0{
                    continue;
                }
                let nx =  w + x as i32;
                let ny = k + y as i32;

                if nx < 0 || ny < 0 {
                    continue;
                }
                if self.height as i32 <= ny  || self.width as i32 <= nx {
                    continue;
                }
                if self.grid[ny as usize][nx as usize].revealed {
                    continue;
                }
                if let Cell::Empty(0) = self.grid[ny as usize][nx as usize].cell{
                    self.dfs_visit(nx as usize, ny as usize);
                }                
                
                self.grid[ny as usize][nx as usize].revealed = true;
            }
        }
    }

    // sprawdzanie czy zostały jedynie miny na planszy
    pub fn check_win(&self)-> bool{
        for row in &self.grid {
            for tile in row {
                match tile.cell {
                    Cell::Mine => continue,
                    _ => {
                        if !tile.revealed {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    // opcja oflagowania pola
    pub fn flag(&mut self, x: usize, y: usize){
        if self.width <= x || self.height <= y {
            return;
        }
        let tile = &mut self.grid[y][x];
        if tile.revealed {
            println!("To pole jest już odkryte!");
            return;
        }
        tile.flagged = !tile.flagged;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.grid[y][x]
    }

}
