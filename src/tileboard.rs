use std::{error::Error, path::Path};

use csv::{ReaderBuilder, StringRecord};

// Second column determines eels vs escalators
const TYPE_COLUMN_INDEX: usize = 1;
// Column determines where if eel or escalator, where the destination is
const DESTINATION_COLUMN_INDEX: usize = 2;

pub fn read_tile_board_from_csv(csv_path: &Path) -> Result<Vec<Tile>, Box<dyn Error>> {
    let mut tileboard = Vec::<Tile>::with_capacity(60);
    let mut rows = ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_path)?
        .into_records();

    while let Some(result) = rows.next() {
        let row = result?;

        let tile: Tile = match get_type_from_row(&row)? {
            -1 => Tile::Eel(get_destination_from_row(&row)?),
            0 => Tile::Normal,
            1 => Tile::Escalator(get_destination_from_row(&row)?),
            _ => return Err("Type out of range".into()),
        };
        tileboard.push(tile);
    }

    Ok(tileboard)
}

fn get_type_from_row(row: &StringRecord) -> Result<i8, Box<dyn Error>> {
    let tile_type = row
        .get(TYPE_COLUMN_INDEX)
        .ok_or(format!(
            "Missing tile type column (index {})",
            TYPE_COLUMN_INDEX
        ))?
        .parse()?;

    Ok(tile_type)
}

pub fn find_next_eel(tileboard: &[Tile], start: usize) -> Option<usize> {
    let remaining = &tileboard[start..];

    for (index, tile) in remaining.iter().enumerate() {
        match tile {
            Tile::Eel(_) => return Some(start + index),
            _ => continue,
        };
    }
    None
}

pub fn find_next_escalator(tileboard: &[Tile], start: usize) -> Option<usize> {
    let remaining = &tileboard[start..];

    for (index, tile) in remaining.iter().enumerate() {
        match tile {
            Tile::Escalator(_) => return Some(start + index),
            _ => continue,
        };
    }
    None
}

fn get_destination_from_row(row: &StringRecord) -> Result<u8, Box<dyn Error>> {
    let dest: u8 = row
        .get(DESTINATION_COLUMN_INDEX)
        .ok_or(format!(
            "Missing destination column (index {})",
            DESTINATION_COLUMN_INDEX
        ))?
        .parse()?;

    Ok(dest)
}

#[repr(i8)]
pub enum Tile {
    Eel(u8) = -1,
    Normal = 0,
    Escalator(u8) = 1,
}
