use crate::input;
use std::collections::HashMap;

static INPUT: &str = input::_INPUT;

#[derive(Debug,PartialEq,Eq,Clone,Hash)]
struct Tile {
    lines: Vec<Vec<bool>>
}

pub fn corners() -> i64 {
    let data: Vec<(i64, Tile)> = INPUT.split("\n\n").map(|tile| {
        let mut lines = tile.trim().lines();
        let id = lines.next().unwrap().split(" ").skip(1).next().unwrap().strip_suffix(":").unwrap().parse().unwrap();
        let parsed_tile = lines.map(|line| {
            line.trim().chars().map(|symbol| if symbol == '#' {true} else {false}).collect()
        }).collect();
        (id, Tile::new(parsed_tile))
    }).collect();
    let mut counters = HashMap::new();
    let mut edges = HashMap::new();
    for (id, tile) in data.into_iter() {
        let new_edges = tile.edges();
        for edge in new_edges.iter() {
            *counters.entry(edge.clone()).or_insert(0) += 1;
        }
        edges.insert(id, new_edges);
    }
    let mut result = 1;
    for (id, edges) in edges {
        let mut counter = 0;
        for edge in edges.iter() {
            if *counters.get(edge).unwrap() == 1 {
                counter += 1;
            }
        }
        
        if counter == 4 {
            result *= id;
        }
    }
    result
}

pub fn water_roughness() -> i64 {
    let mut data: HashMap<i64, Tile> = INPUT.split("\n\n").map(|tile| {
        let mut lines = tile.trim().lines();
        let id = lines.next().unwrap().split(" ").skip(1).next().unwrap().strip_suffix(":").unwrap().parse().unwrap();
        let parsed_tile = lines.map(|line| {
            line.trim().chars().map(|symbol| if symbol == '#' {true} else {false}).collect()
        }).collect();
        (id, Tile::new(parsed_tile))
    }).collect();
    let mut counters: HashMap<Vec<bool>, i64> = HashMap::new();
    let mut edges: HashMap<i64, Vec<Vec<bool>>> = HashMap::new();
    let mut tiles_by_edge: HashMap<Vec<bool>, Vec<i64>> = HashMap::new();
    for (id, tile) in data.iter() {
        let new_edges = tile.edges();
        for edge in new_edges.iter() {
            *counters.entry(edge.clone()).or_insert(0) += 1;
            tiles_by_edge.entry(edge.clone()).or_insert(Vec::new()).push(*id);
        }
        edges.insert(*id, new_edges);
    }

    let mut final_picture = get_starter(edges, counters, &mut data);
    final_picture = assemble(final_picture, data, tiles_by_edge);
    
    final_picture = final_picture.into_iter().map(|tile| tile.without_edges()).collect();
    let final_picture = final_tile(final_picture);
    final_picture.rough_waters() - final_picture.monsters()
}

fn get_starter( edges: HashMap<i64, Vec<Vec<bool>>>, 
                counters: HashMap<Vec<bool>, i64>, 
                data: &mut HashMap<i64, Tile>) -> Vec<Tile> {
    let mut final_picture = Vec::new();
    let mut starting_id = 0;
    for (id, edges) in edges {
        let mut counter = 0;
        for (index, edge) in edges.iter().enumerate() {
            if *counters.get(edge).unwrap() == 1 {
                counter += 1;
            }
        }
        
        if counter == 4 {
            starting_id = id;
            break;
        }
    }
    let mut starter_corner = data.remove(&starting_id).unwrap();
    let transformations: Vec<fn(Tile)->Tile> = vec![Tile::horiz_flip, 
        Tile::vertic_flip, Tile::horiz_flip, Tile::transpose, Tile::vertic_flip,
        Tile::horiz_flip, Tile::vertic_flip, Tile::transpose];
    
    for transformation in transformations {
        starter_corner = transformation(starter_corner);
        if  *counters.get(&starter_corner.get_left_edge()).unwrap() == 1 &&
            *counters.get(&starter_corner.get_top_edge()).unwrap() == 1 
        {
            final_picture.push(starter_corner);
            break;
        }
    }
    final_picture
}

fn assemble(mut final_picture: Vec<Tile>, mut data: HashMap<i64, Tile>, tiles_by_edge: HashMap<Vec<bool>, Vec<i64>>) -> Vec<Tile> {
    let tile_num = data.len() + 1;
    let line_len = (tile_num as f64).sqrt().floor() as usize;
    let transformations: Vec<fn(Tile)->Tile> = vec![Tile::horiz_flip, 
        Tile::vertic_flip, Tile::horiz_flip, Tile::transpose, Tile::vertic_flip,
        Tile::horiz_flip, Tile::vertic_flip, Tile::transpose];
    while final_picture.len() < tile_num {
        if final_picture.len() % line_len == 0 {
            let edge_to_match = final_picture[final_picture.len() - line_len].get_bottom_edge();
            let mut new_tile_id = 0;
            for (id, tile) in data.iter() {
                if tiles_by_edge.get(&edge_to_match).unwrap().contains(&id) {
                    let mut new_tile = tile.clone();
                    for transformation in transformations.iter() {
                        new_tile = transformation(new_tile);
                        if new_tile.get_top_edge() == edge_to_match {
                            final_picture.push(new_tile);
                            new_tile_id = *id;
                            break;
                        }
                    }
                }
            }
            data.remove(&new_tile_id);
        } else {
            let edge_to_match = final_picture[final_picture.len()-1].get_right_edge();
            let mut new_tile_id = 0;
            for (id, tile) in data.iter() {
                if tiles_by_edge.get(&edge_to_match).unwrap().contains(&id) {
                    let mut new_tile = tile.clone();
                    for transformation in transformations.iter() {
                        new_tile = transformation(new_tile);
                        if new_tile.get_left_edge() == edge_to_match {
                            final_picture.push(new_tile);
                            new_tile_id = *id;
                            break;
                        }
                    }
                }
            }
            data.remove(&new_tile_id);
        }
    }
    final_picture
}

fn final_tile(tiles: Vec<Tile>) -> Tile {
    let tile_len = (tiles.len() as f64).sqrt().floor() as usize;
    let mut final_tile = Tile {
        lines: Vec::new()
    };
    for line_index in 0..tile_len {
        let mut new_lines = tiles[line_index * tile_len].clone();
        for column_index in 1..tile_len {
            new_lines = new_lines.extend_lines(&tiles[line_index * tile_len + column_index]);
        }
        final_tile = final_tile.add_lines(new_lines);
    }
    final_tile
}

impl Tile {
    fn transpose(self) -> Tile {
        Tile::new((0..self.lines[0].len())
                        .map(|i| self.lines.iter()
                            .map(|inner| inner[i].clone())
                            .collect::<Vec<bool>>())
                        .collect())
    }

    fn horiz_flip(self) -> Tile {
        Tile::new(self.lines.into_iter().map(|line| line.into_iter().rev().collect()).collect())
    }

    fn vertic_flip(self) -> Tile {
        Tile::new(self.lines.into_iter().rev().collect())
    }

    fn without_edges(self) -> Tile {
        let size = self.lines[0].len();
        Tile::new(self.lines.into_iter().skip(1).take(size-2)
                .map(|line| line.into_iter()
                    .skip(1).take(size-2).collect())
                .collect())
    }

    fn edges(&self) -> Vec<Vec<bool>> {
        let width = self.lines[0].len();
        let edge1 = self.lines[0].clone();
        let edge4 = self.lines[self.lines.len()-1].clone();
        let mut edge2 = Vec::new();
        let mut edge3 = Vec::new();
        for line in self.lines.iter() {
            edge2.push(line[0]);
            edge3.push(line[width-1]);
        }
        vec![edge1.clone(), edge1.into_iter().rev().collect(), 
            edge2.clone(), edge2.into_iter().rev().collect(), 
            edge3.clone(), edge3.into_iter().rev().collect(), 
            edge4.clone(), edge4.into_iter().rev().collect()]
    }

    fn new(v: Vec<Vec<bool>>) -> Tile {
        Tile {
            lines: v
        }
    }

    fn get_left_edge(&self) -> Vec<bool> {
        let mut edge = Vec::new();
        for line in self.lines.iter() {
            edge.push(line[0]);
        }
        edge 
    }

    fn get_right_edge(&self) -> Vec<bool> {
        let mut edge = Vec::new();
        for line in self.lines.iter() {
            edge.push(line[line.len()-1]);
        }
        edge 
    }

    fn get_top_edge(&self) -> Vec<bool> {
        self.lines[0].clone()
    }

    fn get_bottom_edge(&self) -> Vec<bool> {
        self.lines[self.lines.len()-1].clone()
    }

    fn extend_lines(self, new_tile: &Tile) -> Tile {
        if self.lines.len() != new_tile.lines.len() {
            println!("{} {}", self.lines.len(), new_tile.lines.len());
            panic!("Wrong size Tiles");
        }
        let mut new_lines = self.lines;
        for index in 0..new_lines.len() {
            new_lines[index].extend(new_tile.lines[index].iter());
        }
        Tile {
            lines: new_lines
        }
    }

    fn add_lines(self, new_tile: Tile) -> Tile {
        let mut new_lines = self.lines;
        new_lines.extend(new_tile.lines.iter().cloned());
        Tile {
            lines: new_lines
        }
    }

    fn rough_waters(&self) -> i64 {
        self.lines.iter().flatten().filter(|value| **value).count() as i64
    }

    fn monsters(&self) -> i64 {
        let mut tile = self.clone();
        let tile_size_x = tile.lines[0].len();
        let tile_size_y = tile.lines.len();
        let monster_coords = ["                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "];
        let monster_size_x = monster_coords[0].len();
        let monster_size_y = monster_coords.len();
        let monster_coords: Vec<(usize, usize)> = monster_coords
            .iter().enumerate().map(|(y, line)| {
                line.chars().enumerate()
                    .filter(|(_,c)| *c == '#')
                    .map(move |(x,_)| (x,y))
            }).flatten().collect();
        let mut monster_counter = 0;
        for index_y in 0..=(tile_size_y-monster_size_y) {
            for index_x in 0..=(tile_size_x-monster_size_x) {
                let mut monster_found = true;
                for (monster_x, monster_y) in monster_coords.iter() {
                    if !monster_found {
                        break;
                    }
                    monster_found = monster_found && tile.lines[index_y + monster_y][index_x + monster_x]
                }
                if monster_found {
                    monster_counter += monster_coords.len();
                }
            }
        }
        let transformations: Vec<fn(Tile)->Tile> = vec![Tile::horiz_flip, 
        Tile::vertic_flip, Tile::horiz_flip, Tile::transpose, Tile::vertic_flip,
        Tile::horiz_flip, Tile::vertic_flip, Tile::transpose];
        for transformation in transformations {
            if monster_counter == 0 {
                tile = transformation(tile);
                for index_y in 0..=(tile_size_y-monster_size_y) {
                    for index_x in 0..=(tile_size_x-monster_size_x) {
                        let mut monster_found = true;
                        for (monster_x, monster_y) in monster_coords.iter() {
                            if !monster_found {
                                break;
                            }
                            monster_found = monster_found && tile.lines[index_y + monster_y][index_x + monster_x]
                        }
                        if monster_found {
                            monster_counter += monster_coords.len();
                        }
                    }
                }
            }
        }
        monster_counter as i64
    }
}