
use std::path::Path;
use std::io;
use std::io::{Write, Read};
use std::fs::OpenOptions;
use std::convert::TryInto;

use super::square_maze::{SquareMaze, WallDirection, MazeCell};

//TODO Just serialise the internal SquareMaze bits and read them back.
//Avoid having two complicated formats.

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

    let num_bytes = num_bytes(maze.width, maze.height);
    let mut data = vec![0; num_bytes];

    let mut bit_cnt = 0;
    let mut byte_cnt = 0;
    let mut wall_byte:u8 = data[0];

    for y in 0..maze.height + 1 {
        for x in 0..maze.width + 1 {

            if x == maze.width && y == maze.height {
                //no-op, write zeros for east and south
            } else if x == maze.width {
                if maze.wall(WallDirection::EAST, x-1, y) {
                    wall_byte = wall_byte | (1 << bit_cnt);
                }
            } else if y == maze.height {
                if maze.wall(WallDirection::SOUTH, x, y-1) {
                    wall_byte = wall_byte | (1 << (bit_cnt + 1));
                }
            } else {
                if maze.wall(WallDirection::WEST, x, y) {
                    wall_byte = wall_byte | (1 << bit_cnt);
                }

                if maze.wall(WallDirection::NORTH, x, y) {
                    wall_byte = wall_byte | (1 << (bit_cnt + 1));
                }
            }

            if bit_cnt >= 6 {
                data[byte_cnt] = wall_byte.to_be_bytes()[0];
                bit_cnt = 0;
                byte_cnt = byte_cnt + 1;
                if byte_cnt >= num_bytes {
                    break;
                } else {
                    wall_byte = data[byte_cnt];
                }
            } else {
                bit_cnt = bit_cnt + 2;
            }
        }
    }

    file.write_all(&data)?;
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

    let num_bytes = num_bytes(width, height);
    let mut maze_data = Vec::new();
    let n_data = file.read_to_end(&mut maze_data)?;
    if n_data != num_bytes {
        return read_err("invalid file, unexpected number of maze data");
    }

    let mut maze = SquareMaze::new_filled_with_entry_exit(
        width,
        height,
        MazeCell::new(entry_x.try_into().unwrap(), entry_y.try_into().unwrap()),
        MazeCell::new(exit_x.try_into().unwrap(), exit_y.try_into().unwrap()));

    let mut x = 0;
    let mut y = 0;
    let mut bit_n = 0;
    for i in 0..num_bytes {
        let wall_byte = maze_data[i];
        for bit_cnt in 0..8 {
            let wall_bit = wall_bit(wall_byte, bit_cnt);
            if x < width && y < height && !wall_bit {
                let dir = if bit_n % 2 == 0 {WallDirection::WEST} else {WallDirection::NORTH};
                maze.carve(dir, x, y);
            }
            if x >= width && bit_n % 2 == 1 {
                x = 0;
                y += 1;
            } else if bit_n % 2 == 1 {
                x += 1;
            }
            bit_n += 1;
        }
    }

    Ok(maze)
}

pub fn wall_bit(wall_byte: u8, bit_cnt: usize) -> bool {
    return (wall_byte & (1 << bit_cnt)) > 0;
}

fn num_bits(width: usize, height: usize) -> usize {
    2 * ((width + 1) * (height + 1))
}

fn num_bytes(width: usize, height: usize) -> usize {
    let num_bits = num_bits(width, height);
    f32::ceil(num_bits as f32 / 8.0) as usize
}

fn read_err(message: &str) -> io::Result<SquareMaze> {
    Err(io::Error::new(io::ErrorKind::InvalidInput, message))
}

//header
//PLM (ASCII)
//<Version> (8bit version information), 1 = rectangular
//Flags (8bit) reserved, all 0 as for now

//data for version 1
//width (32bit)
//height (32bit)
//entry_x = 32bit
//entry_y = 32bit
//exit_x = 32bit
//exit_y = 32bit

//labyrinth layout (padded to full byte length, with zeros)
//   1     3       5=0 always 0 for top right edges
//----------------
//|<0    |<2    |<4
//|7     |9     | 11
//----------------
//|<6    |<8    |<10
//|13    |15    |16
//----------------
//12, 14, 16 always 0 for lower sides

//11011011 11100101 00000000

//==> vertical edges always even numbered,
//==> horizontal edges always odd numbered
