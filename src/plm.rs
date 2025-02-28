extern crate bytes;

use self::bytes::{BufMut, BytesMut};
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use super::square_maze::{MazeCell, SquareMaze};

/// plm = _p_ortable _l_abyrinth for_m_at
pub fn output(path: &Path, maze: &SquareMaze) -> io::Result<()> {
    if path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "output path must be a file",
        ));
    }

    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(path)?;

    let buf = output_to_buf(maze);
    file.write_all(&buf)?;
    return Ok(());
}

pub fn output_to_buf(maze: &SquareMaze) -> BytesMut {
    let mut buf =
        BytesMut::with_capacity(11 + maze.horizontal_walls.len() + maze.vertical_walls.len());
    buf.put_slice(&['p' as u8, 'l' as u8, 'm' as u8]);
    buf.put_slice(&[0x01, 0x00]); //version & flags
    buf.put_slice(&[
        maze.width as u8,
        maze.height as u8,
        maze.entry.x as u8,
        maze.entry.y as u8,
        maze.exit.x as u8,
        maze.exit.y as u8,
    ]);
    buf.put_slice(&maze.horizontal_walls);
    buf.put_slice(&maze.vertical_walls);
    return buf;
}

fn read_maze_size(buf: &BytesMut) -> io::Result<(usize, usize)> {
    let width = buf[5].try_into().unwrap();
    let height = buf[6].try_into().unwrap();
    Ok((width, height))
}

pub fn read_from_buf(buf: &BytesMut) -> io::Result<SquareMaze> {
    let n = buf.len();
    if n < 11 {
        return read_err("invalid file");
    }
    if buf[0] != 'p' as u8 || buf[1] != 'l' as u8 || buf[2] != 'm' as u8 {
        return read_err("not a plm file");
    }
    if buf[3] != 0x01 {
        return read_err("illegal maze type");
    }
    if buf[4] != 0x0 {
        return read_err("illegal flags");
    }

    let width = buf[5].try_into().unwrap();
    let height = buf[6].try_into().unwrap();
    let entry_x = buf[7];
    let entry_y = buf[8];
    let exit_x = buf[9];
    let exit_y = buf[10];

    let h_size = ((width * (height + 1)) as f32 / 8.0).ceil() as usize;
    let v_size = (((width + 1) * height) as f32 / 8.0).ceil() as usize;

    if n != 11 + h_size + v_size {
        return read_err("invalid file, unexpected number of maze data");
    }
    let offset = 11;
    let mut h_walls = Vec::with_capacity(h_size);
    for i in 0..h_size {
        h_walls.insert(i, buf[offset + i]);
    }

    let mut v_walls = Vec::with_capacity(v_size);
    for i in 0..v_size {
        v_walls.insert(i, buf[offset + h_size + i]);
    }

    Ok(SquareMaze {
        horizontal_walls: h_walls,
        vertical_walls: v_walls,
        width,
        height,
        entry: MazeCell::new(entry_x as usize, entry_y as usize),
        exit: MazeCell::new(exit_x as usize, exit_y as usize),
    })
}

pub fn read(path: &Path) -> io::Result<SquareMaze> {
    let mut buf_size = BytesMut::with_capacity(11);
    buf_size.resize(11, 0);
    let mut file_size = OpenOptions::new().read(true).open(path)?;

    let n = file_size.read(&mut buf_size)?;
    if n != 11 {
        return read_err("invalid file header");
    }
    let (width, height) = read_maze_size(&buf_size)?;

    let h_size = ((width * (height + 1)) as f32 / 8.0).ceil() as usize;
    let v_size = (((width + 1) * height) as f32 / 8.0).ceil() as usize;

    let data_size = 11 + h_size + v_size;
    let mut buf = BytesMut::with_capacity(data_size);
    buf.resize(data_size, 0);
    let mut file = OpenOptions::new().read(true).open(path)?;
    let n = file.read(&mut buf)?;
    println!("### n = {}, data_size = {}", n, data_size);
    if n != data_size {
        return read_err("invalid file, unexpected number of maze data");
    }

    read_from_buf(&buf)
}

fn read_err(message: &str) -> io::Result<SquareMaze> {
    Err(io::Error::new(io::ErrorKind::InvalidInput, message))
}
