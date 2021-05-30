// use crate::tree::{LeafNode, AST};
// use crate::position::BytePos;
// use crate::span::{ByteSpan, Span};
// use crate::symbols::{Metasymbol, Terminal};

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum U8SliceTerminal<'a> {
//     Char(char),
//     Str(&'a str),
//     U8Slice(&'a [u8]),
//     // Big Endian
//     BigEndianF32(f32),
//     BigEndianF64(f64),
//     BigEndianU8(u8),
//     BigEndianI8(i8),
//     BigEndianU16(u16),
//     BigEndianI16(i16),
//     BigEndianU32(u32),
//     BigEndianI32(i32),
//     BigEndianU64(u64),
//     BigEndianI64(i64),
//     BigEndianU128(u128),
//     BigEndianI128(i128),
//     BigEndianUsize(usize),
//     BigEndianIsize(isize),
//     // Little Endian
//     LittleEndianF32(f32),
//     LittleEndianF64(f64),
//     LittleEndianU8(u8),
//     LittleEndianI8(i8),
//     LittleEndianU16(u16),
//     LittleEndianI16(i16),
//     LittleEndianU32(u32),
//     LittleEndianI32(i32),
//     LittleEndianU64(u64),
//     LittleEndianI64(i64),
//     LittleEndianU128(u128),
//     LittleEndianI128(i128),
//     LittleEndianUsize(usize),
//     LittleEndianIsize(isize),
// }

// impl<'a, O, V> Terminal<'a, [u8], O, V, ByteSpan, BytePos> for U8SliceTerminal<'a> {
//     fn eval(
//         &'a self,
//         input: &'a [u8],
//         pos: BytePos,
//         all_of_the_span: &ByteSpan,
//     ) -> Result<AST<O, V, ByteSpan>, ()> {
//         match self {
//             U8SliceTerminal::Char(c) => {
//                 let start = pos;
//                 let pos: usize = pos.0 as usize;
//                 let len = c.len_utf8();
//                 if pos + len <= all_of_the_span.hi().0 as usize
//                     && &input[pos..pos + len] == c.to_string()[..].as_bytes()
//                 {
//                     Ok(AST::<O, V, ByteSpan>::from_leaf_node(
//                         LeafNode::from_m(Metasymbol::Epsilon),
//                         ByteSpan::from_start_len(start, len as u16),
//                     ))
//                 } else {
//                     Err(())
//                 }
//             }
//             U8SliceTerminal::Str(s) => {
//                 let start = pos;
//                 let pos: usize = pos.0 as usize;
//                 let s_bytes = s.as_bytes();
//                 let len = s_bytes.len();
//                 if pos + len <= all_of_the_span.hi().0 as usize
//                     && &input[pos..pos + len] == s.as_bytes()
//                 {
//                     Ok(AST::<O, V, ByteSpan>::from_leaf_node(
//                         LeafNode::from_m(Metasymbol::Epsilon),
//                         ByteSpan::from_start_len(start, len as u16),
//                     ))
//                 } else {
//                     Err(())
//                 }
//             }
//             U8SliceTerminal::U8Slice(s) => {
//                 let start = pos;
//                 let pos: usize = pos.0 as usize;
//                 let s_bytes = s.as_bytes();
//                 let len = s_bytes.len();
//                 if pos + len <= all_of_the_span.hi().0 as usize
//                     && &input[pos..pos + len] == s.as_bytes()
//                 {
//                     Ok(AST::<O, V, ByteSpan>::from_leaf_node(
//                         LeafNode::from_m(Metasymbol::Epsilon),
//                         ByteSpan::from_start_len(start, len as u16),
//                     ))
//                 } else {
//                     Err(())
//                 }
//             }
//         }
//     }
// }