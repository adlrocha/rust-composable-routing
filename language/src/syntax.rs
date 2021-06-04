use libipld::{DagCbor, Result, Ipld};
use libipld::prelude::Codec;
use libipld_core::codec::{Encode, Decode};
use libipld::cbor::DagCborCodec;

use std::any::Any;
use anyhow;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// TODO: All impl for basic node types are equal.
//      - First switch Node to enum instead of a trait.
//      - Then try to use generics as much as possible
// Sticking to the trait solution for now
pub trait Node: std::fmt::Debug {
    fn marshal_cbor(&self) -> Result<Vec<u8>>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Copy, Clone, PartialEq, Debug, DagCbor)]
struct Bool(bool);
impl Node for Bool{
    fn marshal_cbor(&self) -> Result<Vec<u8>>{
        return DagCborCodec.encode(&self.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, PartialEq, Debug, DagCbor)]
struct Str(String);
impl Node for Str{
    fn marshal_cbor(&self) -> Result<Vec<u8>>{
        return DagCborCodec.encode(&self.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Bytes(Vec<u8>);
impl Node for Bytes{
    fn marshal_cbor(&self) -> Result<Vec<u8>>{
        return DagCborCodec.encode(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Encode<DagCborCodec> for Bytes {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> Result<()> {
        self.0.as_slice().encode(c, w)
    }
}

impl Decode<DagCborCodec> for Bytes {
    fn decode<R: std::io::Read + std::io::Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        if let Ipld::Bytes(bytes) = Ipld::decode(c, r)? {
            Ok(Self(bytes))
        } else {
            Err(anyhow::anyhow!("unexpected ipld"))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Number {
    Int(i64), //NOTE: In the Go implementation this is an i64. Come back to this.
    Float(f64),
}

impl Encode<DagCborCodec> for Number {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> Result<()> {
        match self {
            Number::Int(s) => s.encode(c, w),
            Number::Float(s) => s.encode(c, w),
        }
    }
}

impl Decode<DagCborCodec> for Number {
    fn decode<R: std::io::Read + std::io::Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        if let Ipld::Integer(n) = Ipld::decode(c, r)? {
            Ok(Number::Int(n as i64))
        } else if let Ipld::Float(n) = Ipld::decode(c, r)? {
            Ok(Number::Float(n))
        } else {
            Err(anyhow::anyhow!("unexpected ipld"))
        }
    }
}

impl Node for Number {
    fn marshal_cbor(&self) -> Result<Vec<u8>>{
        return DagCborCodec.encode(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
// #[derive(Clone, PartialEq, Debug)]
// struct Nodes<N: Node>(Vec<N>);
// #[derive(Clone, PartialEq, Debug)]
// struct List<N:Node> {
//     elements: Nodes<N>
// }
//
// impl<N: Node> Node for List<N> {
//     fn marshal_cbor(&self) -> Result<Vec<u8>>{
//         return DagCborCodec.encode(&self)
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

/*
#[derive(Clone, PartialEq, Debug)]
struct Bytes(Vec<u8>);




impl Encode<DagCborCodec> for Bytes {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> Result<()> {
        self.0.as_slice().encode(c, w)
    }
}
*/
pub fn marshal<N: Node>(n: &N) -> Result<Vec<u8>> {
    n.marshal_cbor()
}

pub fn unmarshal(bytes: &Vec<u8>) -> Result<Box<dyn Node>>{

    let dec = DagCborCodec.decode::<Ipld>(&bytes).unwrap();
    match dec{
        Ipld::Bool(s) => Ok(Box::new(Bool(s))),
        Ipld::Bytes(s) => Ok(Box::new(Bytes(s))),
        Ipld::String(s) => Ok(Box::new(Str(s))),
        Ipld::Float(s) => Ok(Box::new(Number::Float(s))),
        Ipld::Integer(s) => Ok(Box::new(Number::Int(s as i64))),
        _ => {
            println!("{:?}, {:?}", dec, type_of(&dec));
            panic!("Unmarshal for type not implemented: {:?}", dec)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn e2e_bool(){
        let a = Bool(true);
        let bytes = marshal(&a).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Bool = match des.as_any().downcast_ref::<Bool>(){
            Some(b) => b,
            None => panic!("Not Bool Type")
        };
        assert_eq!(&a, out);
    }

    #[test]
    fn e2e_str(){
        let a = Str(String::from("testing"));
        let bytes = marshal(&a).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Str = match des.as_any().downcast_ref::<Str>(){
            Some(b) => b,
            None => panic!("Not Str Type")
        };
        assert_eq!(&a, out);
    }

    #[test]
    fn e2e_bytes(){
        let a = Bytes(vec![1,2,3]);
        let bytes = marshal(&a).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Bytes = match des.as_any().downcast_ref::<Bytes>(){
            Some(b) => b,
            None => panic!("Not Bytes Type")
        };
        assert_eq!(&a, out);
    }

    #[test]
    fn e2e_numbers_int(){
        let a = Number::Float(123.3);
        let bytes = marshal(&a).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Number = match des.as_any().downcast_ref::<Number>(){
            Some(b) => b,
            None => panic!("Not Int Type")
        };
        assert_eq!(&a, out);
    }

    #[test]
    fn e2e_numbers_float(){
        let a = Number::Float(123.3);
        let bytes = marshal(&a).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Number = match des.as_any().downcast_ref::<Number>(){
            Some(b) => b,
            None => panic!("Not Int Type")
        };
        assert_eq!(&a, out);
    }
}
