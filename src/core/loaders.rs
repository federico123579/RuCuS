use crate::core::{CubeElement, TileColor};
use std::{fs::read_to_string, path::Path};

pub trait CubeLoader {
    fn to_model_elements(self) -> [[[CubeElement; 3]; 3]; 3];
}

/// This enum is used in the hand-crafter map of tiles to CubeElements.
/// The parameters of each are the indexes of the tiles that are used to create the CubeElement.
enum CubeElementType {
    Vertix(usize, usize, usize),
    Edge(usize, usize),
    Face(usize),
    Kernel,
}

impl CubeElementType {
    fn get_tiles_indexes(&self) -> Vec<usize> {
        match self {
            CubeElementType::Vertix(a, b, c) => vec![*a, *b, *c],
            CubeElementType::Edge(a, b) => vec![*a, *b],
            CubeElementType::Face(a) => vec![*a],
            CubeElementType::Kernel => vec![],
        }
    }

    fn get_sorted_tiles(&self, tiles: &Vec<TileColor>) -> Vec<TileColor> {
        let mut sorted_tiles = self
            .get_tiles_indexes()
            .iter()
            .map(|i| tiles[*i])
            .collect::<Vec<TileColor>>();
        sorted_tiles.sort();
        sorted_tiles
    }

    fn compose_element(&self, tiles: &Vec<TileColor>) -> CubeElement {
        let tiles = self.get_sorted_tiles(tiles);

        if let Self::Vertix(_, _, _) = self {
            match tiles.as_slice() {
                [TileColor::White, TileColor::Orange, TileColor::Green] => {
                    CubeElement::WhiteOrangeGreen
                }
                [TileColor::White, TileColor::Orange, TileColor::Blue] => {
                    CubeElement::WhiteBlueOrange
                }
                [TileColor::White, TileColor::Green, TileColor::Red] => CubeElement::WhiteGreenRed,
                [TileColor::White, TileColor::Red, TileColor::Blue] => CubeElement::WhiteRedBlue,
                [TileColor::Orange, TileColor::Green, TileColor::Yellow] => {
                    CubeElement::YellowOrangeGreen
                }
                [TileColor::Orange, TileColor::Blue, TileColor::Yellow] => {
                    CubeElement::YellowBlueOrange
                }
                [TileColor::Green, TileColor::Red, TileColor::Yellow] => {
                    CubeElement::YellowGreenRed
                }
                [TileColor::Red, TileColor::Blue, TileColor::Yellow] => CubeElement::YellowRedBlue,
                _ => panic!("Invalid vertix"),
            }
        } else if let Self::Edge(_, _) = self {
            match tiles.as_slice() {
                [TileColor::White, TileColor::Orange] => CubeElement::WhiteOrange,
                [TileColor::White, TileColor::Green] => CubeElement::WhiteGreen,
                [TileColor::White, TileColor::Red] => CubeElement::WhiteRed,
                [TileColor::White, TileColor::Blue] => CubeElement::WhiteBlue,
                [TileColor::Orange, TileColor::Yellow] => CubeElement::YellowOrange,
                [TileColor::Green, TileColor::Yellow] => CubeElement::YellowGreen,
                [TileColor::Red, TileColor::Yellow] => CubeElement::YellowRed,
                [TileColor::Blue, TileColor::Yellow] => CubeElement::YellowBlue,
                [TileColor::Red, TileColor::Blue] => CubeElement::RedBlue,
                [TileColor::Green, TileColor::Red] => CubeElement::RedGreen,
                [TileColor::Orange, TileColor::Blue] => CubeElement::OrangeBlue,
                [TileColor::Orange, TileColor::Green] => CubeElement::OrangeGreen,
                _ => panic!("Invalid edge"),
            }
        } else if let Self::Face(_) = self {
            match tiles.as_slice() {
                [TileColor::White] => CubeElement::White,
                [TileColor::Yellow] => CubeElement::Yellow,
                [TileColor::Red] => CubeElement::Red,
                [TileColor::Green] => CubeElement::Green,
                [TileColor::Blue] => CubeElement::Blue,
                [TileColor::Orange] => CubeElement::Orange,
                _ => panic!("Invalid face"),
            }
        } else {
            CubeElement::Kernel
        }
    }
}

/// CubeTextLoader map of elements and indexes of tile array.
const CTL_MAP: [[[CubeElementType; 3]; 3]; 3] = [
    [
        [
            CubeElementType::Vertix(6, 24, 42),
            CubeElementType::Edge(25, 39),
            CubeElementType::Vertix(26, 33, 36),
        ],
        [
            CubeElementType::Edge(3, 43),
            CubeElementType::Face(40),
            CubeElementType::Edge(34, 37),
        ],
        [
            CubeElementType::Vertix(0, 44, 51),
            CubeElementType::Edge(41, 48),
            CubeElementType::Vertix(35, 38, 45),
        ],
    ],
    [
        [
            CubeElementType::Edge(7, 21),
            CubeElementType::Face(22),
            CubeElementType::Edge(23, 30),
        ],
        [
            CubeElementType::Face(4),
            CubeElementType::Kernel,
            CubeElementType::Face(31),
        ],
        [
            CubeElementType::Edge(1, 52),
            CubeElementType::Face(49),
            CubeElementType::Edge(32, 46),
        ],
    ],
    [
        [
            CubeElementType::Vertix(8, 15, 18),
            CubeElementType::Edge(16, 19),
            CubeElementType::Vertix(17, 20, 27),
        ],
        [
            CubeElementType::Edge(5, 12),
            CubeElementType::Face(13),
            CubeElementType::Edge(14, 28),
        ],
        [
            CubeElementType::Vertix(2, 9, 53),
            CubeElementType::Edge(10, 50),
            CubeElementType::Vertix(11, 29, 47),
        ],
    ],
];

pub struct CubeTextLoader {
    text: String,
}

/// CubeTextLoader is a loader for a cube model from a text file.
/// The format of the text to be loaded as a complete cube file is as follows:
/// B B B
/// B B B
/// B B B
/// O O O
/// O O O
/// O O O
/// Y Y Y
/// Y Y Y
/// Y Y Y
/// G G G
/// G G G
/// G G G
/// R R R
/// R R R
/// R R R
/// W W W
/// W W W
/// W W W
impl CubeLoader for CubeTextLoader {
    fn to_model_elements(self) -> [[[CubeElement; 3]; 3]; 3] {
        let mut tile_colors = Vec::with_capacity(54);
        let mut lines = self.text.lines();

        // from formatted text to tile_colors 1D array
        for _ in 0..6 {
            for _ in 0..3 {
                let line = lines.next().unwrap().trim();
                for _ in 0..3 {
                    let tile_color = match line.chars().next().unwrap() {
                        'B' => TileColor::Blue,
                        'O' => TileColor::Orange,
                        'Y' => TileColor::Yellow,
                        'G' => TileColor::Green,
                        'R' => TileColor::Red,
                        'W' => TileColor::White,
                        _ => panic!("Invalid tile color"),
                    };
                    tile_colors.push(tile_color);
                }
            }
        }

        CTL_MAP.map(|x| x.map(|y| y.map(|z| z.compose_element(&tile_colors))))
    }
}

impl CubeTextLoader {
    fn from_string(text: String) -> Self {
        Self { text }
    }

    pub fn from_file(path: &Path) -> Self {
        let text = read_to_string(path).unwrap();
        CubeTextLoader { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn load_solved_cube_elems() -> [[[CubeElement; 3]; 3]; 3] {
        // get the path of the file
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("solved_cube.txt");
        let loader = CubeTextLoader::from_file(path.as_path());
        loader.to_model_elements()
    }

    #[test]
    fn solved_cube_loaded_from_file() {
        let cube_elems = load_solved_cube_elems();
        assert_eq!(cube_elems[0][0][0], CubeElement::YellowRedBlue);
        assert_eq!(cube_elems[0][0][1], CubeElement::YellowRed);
        assert_eq!(cube_elems[0][0][2], CubeElement::YellowGreenRed);
        assert_eq!(cube_elems[0][1][0], CubeElement::RedBlue);
        assert_eq!(cube_elems[0][1][1], CubeElement::Red);
        assert_eq!(cube_elems[0][1][2], CubeElement::RedGreen);
        assert_eq!(cube_elems[0][2][0], CubeElement::WhiteRedBlue);
        assert_eq!(cube_elems[0][2][1], CubeElement::WhiteRed);
        assert_eq!(cube_elems[0][2][2], CubeElement::WhiteGreenRed);
        assert_eq!(cube_elems[1][0][0], CubeElement::YellowBlue);
        assert_eq!(cube_elems[1][0][1], CubeElement::Yellow);
        assert_eq!(cube_elems[1][0][2], CubeElement::YellowGreen);
        assert_eq!(cube_elems[1][1][0], CubeElement::Blue);
        assert_eq!(cube_elems[1][1][1], CubeElement::Kernel);
        assert_eq!(cube_elems[1][1][2], CubeElement::Green);
        assert_eq!(cube_elems[1][2][0], CubeElement::WhiteBlue);
        assert_eq!(cube_elems[1][2][1], CubeElement::White);
        assert_eq!(cube_elems[1][2][2], CubeElement::WhiteGreen);
        assert_eq!(cube_elems[2][0][0], CubeElement::YellowBlueOrange);
        assert_eq!(cube_elems[2][0][1], CubeElement::YellowOrange);
        assert_eq!(cube_elems[2][0][2], CubeElement::YellowOrangeGreen);
        assert_eq!(cube_elems[2][1][0], CubeElement::OrangeBlue);
        assert_eq!(cube_elems[2][1][1], CubeElement::Orange);
        assert_eq!(cube_elems[2][1][2], CubeElement::OrangeGreen);
        assert_eq!(cube_elems[2][2][0], CubeElement::WhiteBlueOrange);
        assert_eq!(cube_elems[2][2][1], CubeElement::WhiteOrange);
        assert_eq!(cube_elems[2][2][2], CubeElement::WhiteOrangeGreen);
    }
}
