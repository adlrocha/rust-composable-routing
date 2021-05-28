use libipld::{DagCbor, Result, Ipld};
use libipld::prelude::Codec;
use libipld::cbor::DagCborCodec;

use std::any::Any;

#[derive(Copy, Clone, PartialEq, Debug, DagCbor)]
struct Bool(bool);

pub trait Node: std::fmt::Debug {
    fn marshal_cbor(&self) -> Result<Vec<u8>>;
    fn as_any(&self) -> &dyn Any;
}

impl Node for Bool{
    fn marshal_cbor(&self) -> Result<Vec<u8>>{
        return DagCborCodec.encode(&self.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn marshal(n: Box<dyn Node>) -> Result<Vec<u8>> {
    n.marshal_cbor()
}

pub fn unmarshal(bytes: &Vec<u8>) -> Result<Box<dyn Node>>{

    let dec = DagCborCodec.decode::<Ipld>(&bytes).unwrap();
    let out = match dec{
        Ipld::Bool(s) => Bool(s),
        _ => {
            println!("{:?}", dec);
            panic!("Unmarshal for type not implemented: {:?}", dec)
        }
    };
    println!("{:?}", dec);
    Ok(Box::new(out))
}


#[cfg(test)]
mod tests {
    use crate::syntax::{Bool, unmarshal, marshal};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn e2e_bool(){
        let a = Bool(true);
        // let bytes = a.marshal_cbor().unwrap(); // TODO: Add ? instead of unwrap
        let bytes = marshal(Box::new(a)).unwrap();
        let des = unmarshal(&bytes).unwrap();
        let out: &Bool = match des.as_any().downcast_ref::<Bool>(){
            Some(b) => b,
            None => panic!("Not Bool Type")
        };
        assert_eq!(&a, out);
    }
}
