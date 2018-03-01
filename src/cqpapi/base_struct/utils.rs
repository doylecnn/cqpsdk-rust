use byteorder;
use byteorder::ByteOrder;
use encoding::{Encoding, DecoderTrap};
use encoding::all::GB18030;

pub fn get_i16(ref bytes :&[u8], index :usize)->(i16, usize){
    let left_index = index;
    let right_index = left_index + 2;
    (byteorder::BigEndian::read_i16(&bytes[left_index..right_index]), right_index)
}

pub fn get_i32(ref bytes :&[u8], index :usize)->(i32, usize){
    let left_index = index;
    let right_index = left_index + 4;
    (byteorder::BigEndian::read_i32(&bytes[left_index..right_index]), right_index)
}

pub fn get_i64(ref bytes :&[u8], index :usize)->(i64, usize){
    let left_index = index;
    let right_index = left_index + 8;
    (byteorder::BigEndian::read_i64(&bytes[left_index..right_index]), right_index)
}

pub fn get_string(ref bytes :&[u8], index :usize)->(String, usize){
    let (len, left_index) = get_usize(&bytes, index, 2);
    let right_index = left_index + len;
    (GB18030.decode(&bytes[left_index..right_index], DecoderTrap::Ignore).unwrap(), right_index)
}

pub fn get_usize(ref bytes :&[u8], index :usize, len :usize)->(usize, usize){
    match len {
        2=>{
            let (size, index) = get_i16(&bytes, index);
            return (size as usize, index);
        }
        4=>{
            let (size, index) = get_i32(&bytes, index);
            return (size as usize, index);
        }
        _ =>{
            return (0, 0)
        }
    }
}