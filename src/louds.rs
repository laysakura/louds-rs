mod louds;
mod louds_index;
mod louds_node_num;

extern crate fid_rs;
use fid_rs::Fid;

/// LOUDS (Level-Order Unary Degree Sequence).
///
/// This class can handle tree structure of virtually **arbitrary number of nodes**.
///
/// In fact, _N_ (number of nodes in the tree) is designed to be limited to: _N < 2^64 / 2_, while each node is represented in 2bits in average.<br>
/// It should be enough for almost all usecases since a binary data of length of _2^63_ consumes _2^20 = 1,048,576_ TB (terabytes), which is hard to handle by state-of-the-art computer architecture.
///
/// # Examples
/// Say we want to hold the following tree structure in minimum length of bits.
///
/// ```text
/// (1)
///  |
///  |---+---+
///  |   |   |
/// (2) (3) (4)
///  |       |
///  |       |---+-----+
///  |       |   |     |
/// (5)     (6) (7)   (8)
///              |     |
///              |     |----+
///              |     |    |
///             (9)   (10) (11)
/// ```
///
/// This tree has NodeNum (node number of 1-origin, assigned from left node to right & top to bottom) and edges.
/// With LOUDS, this tree is represented as the following LBS (LOUDS Bit String).
///
/// ```text
/// NodeNum       | 0 (virtual root) | 1          | 2    | 3 | 4          | 5 | 6 | 7    | 8       | 9 | 10 | 11 |
/// LBS           | 1  0             | 1  1  1  0 | 1  0 | 0 | 1  1  1  0 | 0 | 0 | 1  0 | 1  1  0 | 0 | 0  | 0  |
/// Child NodeNum | 1  -             | 2  3  4  - | 5  - | - | 6  7  8  - | - | - | 9  - | 10 11 - | - | -  | -  |
/// Index         | 0  1             | 2  3  4  5 | 6  7 | 8 | 9  10 11 12| 13| 14| 15 16| 17 18 19| 20| 21 | 22 |
/// ```
///
/// The same tree is represented as follows using index.
///
/// ```text
/// <0>
///  |
///  |---+---+
///  |   |   |
/// <2> <3> <4>
///  |       |
///  |       |---+-----+
///  |       |   |     |
/// <6>     <9> <10>  <11>
///              |     |
///              |     |----+
///              |     |    |
///             <15>  <17> <18>
/// ```
///
/// Then, create this tree structure with `Louds` and call operations to it.
///
/// ```
/// use louds_rs::{Louds, LoudsIndex, LoudsNodeNum};
///
/// // Construct from LBS.
/// let s = "10_1110_10_0_1110_0_0_10_110_0_0_0";
/// let louds = Louds::from(s);
///
/// // LoudsNodeNum <-> LoudsIndex
/// let node8 = LoudsNodeNum::new(8);
/// let index11 = louds.node_num_to_index(&node8);
/// assert_eq!(louds.index_to_node_num(&index11), node8);
///
/// // Search for children.
/// assert_eq!(louds.parent_to_children(&node8), vec!(LoudsIndex::new(17), LoudsIndex::new(18)));
///
/// // Search for parent.
/// assert_eq!(louds.child_to_parent(&index11), LoudsNodeNum::new(4));
/// ```
pub struct Louds {
    lbs: Fid,
}

#[derive(PartialEq, Eq, Debug)]
/// Node number of [Louds](struct.Louds.html) tree.
pub struct LoudsNodeNum {
    value: u64,
}

#[derive(PartialEq, Eq, Debug)]
/// Index of [Louds](struct.Louds.html) tree.
pub struct LoudsIndex {
    value: u64,
}
