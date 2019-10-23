
use std::path::Path;
use std::io;
use std::io::{Write, Read};
use std::fs::OpenOptions;
use std::convert::TryInto;

use super::square_maze::{SquareMaze, MazeCell};

/// plm = _p_ortable _l_abyrinth for_m_at
pub fn output(path: &Path, maze: &SquareMaze) -> io::Result<()> {
    if path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "output path must be a file"))
    }

    let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path)?;
    file.write_all(&['p' as u8, 'l' as u8, 'm' as u8])?;
    file.write_all(&[0x01, 0x00])?; //version & flags
    file.write_all(&[maze.width as u8, maze.height as u8,
                     maze.entry.x as u8, maze.entry.y as u8,
                     maze.exit.x as u8, maze.exit.y as u8])?;
    file.write_all(&maze.horizontal_walls)?;
    file.write_all(&maze.vertical_walls)?;
    return Ok(())
}

pub fn read(path: &Path) -> io::Result<SquareMaze> {

    let mut file = OpenOptions::new()
            .read(true)
            .open(path)?;

    let mut meta_data = [0;11];
    let n = file.read(&mut meta_data)?;
    if n != 11 {
        return read_err("invalid file");
    }
    if meta_data[0] != 'p' as u8 || meta_data[1] != 'l' as u8 || meta_data[2] != 'm' as u8 {
        return read_err("not a plm file");
    }
    if meta_data[3] != 0x01 {
        return read_err("illegal maze type");
    }
    if meta_data[4] != 0x0 {
        return read_err("illegal flags");
    }

    let width = meta_data[5].try_into().unwrap();
    let height = meta_data[6].try_into().unwrap();
    let entry_x = meta_data[7];
    let entry_y = meta_data[8];
    let exit_x = meta_data[9];
    let exit_y = meta_data[10];

    let h_size = ((width*(height+1)) as f32 / 8.0).ceil() as usize;
    let v_size = (((width+1)*height) as f32 / 8.0).ceil() as usize;

    let mut maze_data = Vec::new();
    let n_data = file.read_to_end(&mut maze_data)?;
    if n_data != h_size + v_size {
        return read_err("invalid file, unexpected number of maze data");
    }

    let mut h_walls = Vec::with_capacity(h_size);
    for i in 0..h_size {
        h_walls.insert(i, maze_data[i]);
    }

    let mut v_walls = Vec::with_capacity(v_size);
    for i in 0..v_size {
        v_walls.insert(i, maze_data[h_size+i]);
    }

    Ok(SquareMaze{
        horizontal_walls: h_walls,
        vertical_walls: v_walls,
        width,
        height,
        entry: MazeCell::new(entry_x as usize, entry_y as usize),
        exit: MazeCell::new(exit_x as usize, exit_y as usize)
    })
}

fn read_err(message: &str) -> io::Result<SquareMaze> {
    Err(io::Error::new(io::ErrorKind::InvalidInput, message))
}
