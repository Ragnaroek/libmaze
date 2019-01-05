
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

use super::square_maze::{SquareMaze, WallDirection};

/// plm = _p_ortable _l_abyrinth for_m_at
pub fn output(path: &Path, maze: &SquareMaze) -> io::Result<()> {
    if path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "output path must be a file"))
    }

    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

    file.write_all(&['p' as u8, 'l' as u8, 'm' as u8])?;
    file.write_all(&[0x01, 0x00])?; //version & flags
    file.write_all(&[maze.width as u8, maze.height as u8,
                     maze.entry.x as u8, maze.entry.y as u8,
                     maze.exit.x as u8, maze.exit.y as u8])?;

    let num_bits = 2 * ((maze.width + 1) * (maze.height + 1));
    let num_bytes = f32::ceil(num_bits as f32 / 8.0) as usize;

    let mut data = vec![0; num_bytes];

    let mut bit_cnt = 0;
    let mut byte_cnt = 0;
    let mut wall_byte = data[0];

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
                data[byte_cnt] = wall_byte;
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

//==> vertical edges always even numbered,
//==> horizontal edges always odd numbered
