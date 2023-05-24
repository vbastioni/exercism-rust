#[rustfmt::skip]
const COORDS: &[(i32, i32); 8] = &[
    (-1, -1), (-1, 0), (-1, 1), 
    ( 0, -1),          ( 0, 1), 
    ( 1, -1), ( 1, 0), ( 1, 1), 
];
const MINE: u8 = b'*';

struct ByteField<'a> {
    field: &'a Vec<&'a [u8]>,
    h: i32,
}

impl<'a> ByteField<'a> {
    fn get(&self, x: i32, y: i32, w: i32) -> u8 {
        if self.field[y as usize][x as usize] == MINE {
            return MINE;
        }

        match COORDS
            .iter()
            .map(|(ly, lx)| (*ly + y, *lx + x))
            .filter(|(ly, lx)| *ly >= 0 && *ly < self.h && *lx >= 0 && *lx < w)
            .filter(|(ly, lx)| self.field[*ly as usize][*lx as usize] == MINE)
            .count()
        {
            0 => b' ',
            v @ _ => b'0' + v as u8,
        }
    }

    fn row(&self, y: i32) -> Vec<u8> {
        let bytes = self.field[y as usize];
        let w = bytes.len() as i32;
        (0..w).map(|x| self.get(x, y, w)).collect::<Vec<_>>()
    }

    fn annotate(&self) -> Vec<String> {
        (0..self.h)
            .map(|y| String::from_utf8(self.row(y)).unwrap())
            .collect::<Vec<_>>()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let h = minefield.len() as i32;
    let mf = minefield.iter().map(|f| f.as_bytes()).collect::<Vec<_>>();
    ByteField { field: &mf, h }.annotate()
}
