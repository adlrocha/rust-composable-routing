use libipld::{DagCbor, Result, Ipld};
use libipld::prelude::Codec;
use libipld::cbor::DagCborCodec;

use std::any::Any;

// TODO: All impl for basic node types are equal. Use
// generics to simplify the code.
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

pub fn marshal<N: Node>(n: &N) -> Result<Vec<u8>> {
    n.marshal_cbor()
}

pub fn unmarshal(bytes: &Vec<u8>) -> Result<Box<dyn Node>>{

    let dec = DagCborCodec.decode::<Ipld>(&bytes).unwrap();
    match dec{
        Ipld::Bool(s) => Ok(Box::new(Bool(s))),
        Ipld::String(s) => Ok(Box::new(Str(s))),
        _ => {
            println!("{:?}", dec);
            panic!("Unmarshal for type not implemented: {:?}", dec)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::syntax::{Bool, Str, unmarshal, marshal};
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
}
