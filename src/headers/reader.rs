use bincode::deserialize;
use num_traits::PrimInt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use uuid::*;

pub fn read_header_from_offset<Header: Sized + DeserializeOwned>(
    file_arg: &str,
    offset: u64,
) -> Header {
    let header: Header = {
        let mut file = File::open(file_arg).unwrap();
        let _res = file.seek(SeekFrom::Start(offset)).unwrap();
        if _res != offset {
            panic!("Failed to seek to offset\n"); //shitty error msg i'm tired
        }
        let size = size_of::<Header>();
        let mut file_data: Vec<u8> = vec![0; size];
        file.read_exact(&mut file_data[..]).unwrap();
        // read the bytes into the struct
        deserialize::<Header>(&file_data[..]).unwrap()
    };
    header
}

pub fn le_u128_deserialize<'de, D>(d: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u128>::deserialize(d)?;
    data = u128::from_le(data);
    Ok(data)
}

pub fn le_u64_deserialize<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u64>::deserialize(d)?;
    data = u64::from_le(data);
    Ok(data)
}

pub fn le_u32_deserialize<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u32>::deserialize(d)?;
    data = u32::from_le(data);
    Ok(data)
}

pub fn le_u16_deserialize<'de, D>(d: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u16>::deserialize(d)?;
    data = u16::from_le(data);
    Ok(data)
}

pub fn le_uuid_deserialize<'de, D>(d: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    // mixed endian field whyy
    // going to discover there was a library that already did this at some point
    let data = <[u8; 16]>::deserialize(d)?;
    let mut reversed = [0u8; 16];
    let first = &data[..4];
    let second = &data[4..6];
    let third = &data[6..8];
    let fourth = &data[8..10];
    let last = &data[10..];
    let mut counter = 0;
    for i in first.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in second.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in third.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in fourth.iter() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in last.iter() {
        reversed[counter] = *i;
        counter += 1;
    }
    Ok(uuid::Builder::from_bytes(reversed).build())
}

pub fn bitfield_fetch<T: Sized + PrimInt>(target: T, bitmask: T) -> bool {
    return (target & bitmask) == bitmask;
}
