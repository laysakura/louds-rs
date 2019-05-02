mod louds;

extern crate fid_rs;
use fid_rs::Fid;

/// LOUDS (Level-Order Unary Degree Sequence).
///
/// This class can handle tree structure of virtually **arbitrary number of nodes**.
///
/// In fact, _N_ (number of nodes in the tree) is designed to be limited to: _N < 2^64 / 2_, while each node is represented in 2bits in average.<br>
/// It should be enough for almost all usecases since a binary data of length of _2^63_ consumes _2^20 = 1,048,576_ TB (terabytes), which is hard to handle by state-of-the-art computer architecture.
pub struct Louds {
    lbs: Fid,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
/// Node number of [Louds](struct.Louds.html) tree.
pub struct LoudsNodeNum(pub u64);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
/// Index of [Louds](struct.Louds.html) tree.
pub struct LoudsIndex(pub u64);
