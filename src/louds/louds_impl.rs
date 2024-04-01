use super::{ChildIndexIter, ChildNodeIter, Louds, LoudsIndex, LoudsNodeNum};
use fid_rs::Fid;

impl From<&str> for Louds {
    /// Prepares for building [Louds](struct.Louds.html) from LBS (LOUDS Bit vector).
    ///
    /// It takes _O(log `s`)_ time for validation.
    ///
    /// # Panics
    /// If `s` does not represent a LOUDS tree. `s` must satisfy the following condition as LBS.
    ///
    /// - Starts from "10"
    /// - In the range of _[0, i]_ for any _i (< length of LBS)_;
    ///     - _<u>the number of '0'</u> <= <u>the number of '1'</u> + 1_, because:
    ///         - Each node, including virtual root (node num = 0), has one '0'.
    ///         - Each node is derived from one '1'.
    /// - In the range of _[0, <u>length of LBS</u>)_;
    ///     - _<u>the number of '0'</u> == <u>the number of '1'</u> + 1_
    fn from(s: &str) -> Self {
        let s: String = s
            .chars()
            .filter(|c| match c {
                '0' | '1' => true,
                '_' => false,
                _ => panic!("not allowed"),
            })
            .collect();
        let fid = Fid::from(s.as_str());
        Self::validate_lbs(&fid);
        Louds { lbs: fid }
    }
}

impl From<&[bool]> for Louds {
    /// Prepares for building [Louds](struct.Louds.html) from LBS (LOUDS Bit vector).
    ///
    /// It takes _O(log `bits`)_ time for validation.
    ///
    /// # Panics
    /// Same as [Louds::from::<&str>()](struct.Louds.html#implementations).
    fn from(bits: &[bool]) -> Self {
        let fid = Fid::from(bits);
        Self::validate_lbs(&fid);
        Louds { lbs: fid }
    }
}

impl Louds {
    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn node_num_to_index(&self, node_num: LoudsNodeNum) -> LoudsIndex {
        assert!(node_num.0 > 0);

        let index = self
            .lbs
            .select(node_num.0)
            .unwrap_or_else(|| panic!("NodeNum({}) does not exist in this LOUDS", node_num.0,));
        LoudsIndex(index)
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    pub fn index_to_node_num(&self, index: LoudsIndex) -> LoudsNodeNum {
        self.validate_index(index);

        let node_num = self.lbs.rank(index.0);
        LoudsNodeNum(node_num)
    }

    /// # Panics
    /// - `index` does not point to any node in this LOUDS.
    /// - `index == 0`: (node#1 is root and doesn't have parent)
    pub fn child_to_parent(&self, index: LoudsIndex) -> LoudsNodeNum {
        self.validate_index(index);
        assert!(index.0 != 0, "node#1 is root and doesn't have parent");

        let parent_node_num = self.lbs.rank0(index.0);
        LoudsNodeNum(parent_node_num)
    }

    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn parent_to_children(&self, node_num: LoudsNodeNum) -> Vec<LoudsIndex> {
        self.parent_to_children_indices(node_num).collect()
    }

    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn parent_to_children_indices(&self, node_num: LoudsNodeNum) -> ChildIndexIter {
        assert!(node_num.0 > 0);

        ChildIndexIter {
            inner: self,
            node: node_num,
            start: None,
            end: None,
        }
    }

    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn parent_to_children_nodes(&self, node_num: LoudsNodeNum) -> ChildNodeIter {
        ChildNodeIter(self.parent_to_children_indices(node_num))
    }

    /// Checks if `lbs` satisfy the LBS's necessary and sufficient condition:
    fn validate_lbs(lbs: &Fid) {
        assert!(lbs[0]);
        assert!(!lbs[1]);

        let (mut cnt0, mut cnt1) = (0u64, 0u64);
        for (i, bit) in lbs.iter().enumerate() {
            if bit {
                cnt1 += 1
            } else {
                cnt0 += 1
            };
            assert!(
                cnt0 <= cnt1 + 1,
                "At index {}, the number of '0' ({}) == (the number of '1' ({})) + 2.",
                i,
                cnt0,
                cnt1,
            );
        }
        assert_eq!(cnt0, cnt1 + 1);
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    fn validate_index(&self, index: LoudsIndex) {
        assert!(
            self.lbs[index.0],
            "LBS[index={:?}] must be '1'",
            index,
        );
    }
}

impl<'a> ChildIndexIter<'a> {
    /// Return the length of the iterator.
    ///
    /// It costs _O(log N)_ if the iterator has not had `.next()` and
    /// `.next_back()` called.
    ///
    /// Question: Why not implement [std::iter::ExactSizeIterator]? One could
    /// but they'd be required to do it one of two ways because its signature is
    /// `fn len(&self) -> usize`; `&self` is not mutable:
    ///
    /// 1. Use interior mutability in [ChildIndexIter]. This was attempted with
    /// a [std::cell::RefCell] but it hurt performance slightly.
    ///
    /// 2. Initialize [ChildIndexIter] with the start and end. However
    ///    initializing start and end costs _O(log N)_ each.
    pub fn len(&mut self) -> usize {
        if self.start.is_none() {
            self.start = Some(
                self.inner.lbs.select0(self.node.0).unwrap_or_else(|| {
                    panic!("NodeNum({}) does not exist in this LOUDS", self.node.0,)
                }) + 1,
            );
        }
        if self.end.is_none() {
            self.end = Some(
                self.inner.lbs.select0(self.node.0 + 1).unwrap_or_else(|| {
                    panic!("NodeNum({}) does not exist in this LOUDS", self.node.0 + 1,)
                }) - 1,
            );
        }
        let start = self.start.unwrap();
        let end = self.end.unwrap();
        (end + 1 - start) as usize
    }

    /// Returns whether the iterator is empty.
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

impl<'a> Iterator for ChildIndexIter<'a> {
    type Item = LoudsIndex;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start.is_none() {
            self.start = Some(
                self.inner.lbs.select0(self.node.0).unwrap_or_else(|| {
                    panic!("NodeNum({}) does not exist in this LOUDS", self.node.0,)
                }) + 1,
            );
        }
        let start = self.start.unwrap();
        self.end
            .map(|last| start <= last)
            .unwrap_or_else(|| self.inner.lbs[start])
            .then(|| {
                self.start = Some(start + 1);
                LoudsIndex(start)
            })
    }
}

impl<'a> DoubleEndedIterator for ChildIndexIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end.is_none() {
            self.end = Some(
                self.inner.lbs.select0(self.node.0 + 1).unwrap_or_else(|| {
                    panic!("NodeNum({}) does not exist in this LOUDS", self.node.0 + 1,)
                }) - 1,
            );
        }
        let end = self.end.unwrap();
        self.start
            .map(|first| first <= end)
            .unwrap_or_else(|| self.inner.lbs[end])
            .then(|| {
                self.end = Some(end - 1);
                LoudsIndex(end)
            })
    }
}

impl<'a> Iterator for ChildNodeIter<'a> {
    type Item = LoudsNodeNum;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|index| self.0.inner.index_to_node_num(index))
    }
}

impl<'a> DoubleEndedIterator for ChildNodeIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|index| self.0.inner.index_to_node_num(index))
    }
}

impl<'a> ChildNodeIter<'a> {
    /// See [ChildIndexIter::len].
    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    /// Returns whether the iterator is empty.
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod validate_lbs_success_tests {
    use crate::Louds;
    use fid_rs::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let s = $value;
                let fid = Fid::from(s);
                Louds::validate_lbs(&fid);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: "10_0",
        t2: "10_10_0",
        t3: "10_1110_10_0_1110_0_0_10_110_0_0_0",
        t4: "10_11111111110_0_0_0_0_0_0_0_0_0_0",
    }
}

#[cfg(test)]
mod validate_lbs_failure_tests {
    use crate::Louds;
    use fid_rs::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let s = $value;
                let fid = Fid::from(s);
                Louds::validate_lbs(&fid);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: "0",
        t2: "1",
        t3: "00",
        t4: "01",
        t5: "10",
        t6: "11",
        t7: "00_0",
        t8: "01_0",
        t9: "11_0",
        t10: "10_1",
        t11: "10_10",
        t12: "10_01",
        t13: "10_1110_10_0_1110_0_0_10_110_0_0_1",
    }
}

#[cfg(test)]
mod node_num_to_index_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_index) = $value;
                let louds = Louds::from(in_s);
                let index = louds.node_num_to_index(LoudsNodeNum(node_num));
                assert_eq!(index, LoudsIndex(expected_index));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, 0),

        t2_1: ("10_10_0", 1, 0),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, 6),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 9),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, 10),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, 11),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 15),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 17),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 18),
    }
}

#[cfg(test)]
mod node_num_to_index_failure_tests {
    use crate::{Louds, LoudsNodeNum};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, node_num) = $value;
                let louds = Louds::from(in_s);
                let _ = louds.node_num_to_index(LoudsNodeNum(node_num));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 0),
        t1_2: ("10_0", 2),

        t2_1: ("10_10_0", 0),
        t2_2: ("10_10_0", 3),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
    }
}

#[cfg(test)]
mod index_to_node_num_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index, expected_node_num) = $value;
                let louds = Louds::from(in_s);
                let node_num = louds.index_to_node_num(LoudsIndex(index));
                assert_eq!(node_num, LoudsNodeNum(expected_node_num));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 0, 1),

        t2_1: ("10_10_0", 0, 1),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0, 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 5),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 6),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 7),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 8),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 15, 9),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 17, 10),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 18, 11),
    }
}

#[cfg(test)]
mod index_to_node_num_failure_tests {
    use crate::{Louds, LoudsIndex};

    macro_rules! parameterized_index_not_point_to_node_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, index) = $value;
                let louds = Louds::from(in_s);
                let _ = louds.index_to_node_num(LoudsIndex(index));
            }
        )*
        }
    }

    parameterized_index_not_point_to_node_tests! {
        t1_1: ("10_0", 1),
        t1_2: ("10_0", 3),

        t2_1: ("10_10_0", 1),
        t2_2: ("10_10_0", 3),
        t2_3: ("10_10_0", 4),
        t2_4: ("10_10_0", 5),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 13),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 14),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 16),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 19),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 20),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 21),
        t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 22),
        t3_13: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 23),
        t3_14: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 24),
    }
}

#[cfg(test)]
mod child_to_parent_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index, expected_parent) = $value;
                let louds = Louds::from(in_s);
                let parent = louds.child_to_parent(LoudsIndex(index));
                assert_eq!(parent, LoudsNodeNum(expected_parent));
            }
        )*
        }
    }

    parameterized_tests! {
        t2_1: ("10_10_0", 2, 1),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 1),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 1),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 2),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 4),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 4),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 4),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 15, 7),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 17, 8),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 18, 8),
    }
}

#[cfg(test)]
mod child_to_parent_failure_tests {
    use crate::{Louds, LoudsIndex};

    macro_rules! parameterized_index_not_point_to_node_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, index) = $value;
                let louds = Louds::from(in_s);
                let _ = louds.child_to_parent(LoudsIndex(index));
            }
        )*
        }
    }

    parameterized_index_not_point_to_node_tests! {
        t1_1: ("10_0", 1),
        t1_2: ("10_0", 3),

        t2_1: ("10_10_0", 1),
        t2_2: ("10_10_0", 3),
        t2_3: ("10_10_0", 4),
        t2_4: ("10_10_0", 5),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 13),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 14),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 16),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 19),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 20),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 21),
        t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 22),
        t3_13: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 23),
        t3_14: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 24),
    }

    macro_rules! parameterized_root_not_have_parent_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let in_s = $value;
                let louds = Louds::from(in_s);
                let _ = louds.child_to_parent(LoudsIndex(0));
            }
        )*
        }
    }

    parameterized_root_not_have_parent_tests! {
        t1: "10_0",
        t2: "10_10_0",
        t3: "10_1110_10_0_1110_0_0_10_110_0_0_0",
    }
}

#[cfg(test)]
mod parent_to_children_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_children) = $value;
                let louds = Louds::from(in_s);
                let children: Vec<_> = louds.parent_to_children(LoudsNodeNum(node_num));
                assert_eq!(children, expected_children.iter().map(|c| LoudsIndex(*c)).collect::<Vec<LoudsIndex>>());
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, vec!()),

        t2_1: ("10_10_0", 1, vec!(2)),
        t2_2: ("10_10_0", 2, vec!()),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, vec!(2, 3, 4)),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, vec!(6)),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, vec!()),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, vec!(9, 10, 11)),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, vec!()),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, vec!()),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, vec!(15)),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, vec!(17, 18)),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, vec!()),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, vec!()),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, vec!()),
    }
}

#[cfg(test)]
mod parent_to_children_indices_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_children) = $value;
                let louds = Louds::from(in_s);
                let children: Vec<_> = louds.parent_to_children_indices(LoudsNodeNum(node_num)).collect();
                assert_eq!(children, expected_children.iter().map(|c| LoudsIndex(*c)).collect::<Vec<LoudsIndex>>());
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, vec!()),

        t2_1: ("10_10_0", 1, vec!(2)),
        t2_2: ("10_10_0", 2, vec!()),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, vec!(2, 3, 4)),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, vec!(6)),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, vec!()),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, vec!(9, 10, 11)),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, vec!()),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, vec!()),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, vec!(15)),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, vec!(17, 18)),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, vec!()),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, vec!()),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, vec!()),
    }
}

#[cfg(test)]
mod parent_to_children_indices_rev_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_children) = $value;
                let louds = Louds::from(in_s);
                let children: Vec<_> = louds.parent_to_children_indices(LoudsNodeNum(node_num)).rev().collect();
                assert_eq!(children, expected_children.iter().map(|c| LoudsIndex(*c)).collect::<Vec<LoudsIndex>>());
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, vec!()),

        t2_1: ("10_10_0", 1, vec!(2)),
        t2_2: ("10_10_0", 2, vec!()),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, vec!(4, 3, 2)),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, vec!(6)),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, vec!()),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, vec!(11, 10, 9)),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, vec!()),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, vec!()),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, vec!(15)),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, vec!(18, 17)),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, vec!()),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, vec!()),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, vec!()),
    }
}

#[cfg(test)]
mod parent_to_children_indices_len_success_tests {
    use crate::{Louds, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_size) = $value;
                let louds = Louds::from(in_s);
                let mut iter = louds.parent_to_children_indices(LoudsNodeNum(node_num));
                assert_eq!(iter.len(), expected_size);
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, 0),

        t2_1: ("10_10_0", 1, 1),
        t2_2: ("10_10_0", 2, 0),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, 3),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 1),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 0),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 3),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, 0),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 0),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, 1),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, 2),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 0),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 0),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 0),
    }
}

#[cfg(test)]
mod parent_to_children_indices_next_back_success_tests {
    use crate::{Louds, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_children) = $value;
                let louds = Louds::from(in_s);
                let mut front = Vec::new();
                let mut back = Vec::new();
                let mut iter = louds.parent_to_children_indices(LoudsNodeNum(node_num));
                while let Some(x) = iter.next() {
                    front.push(x);
                    if let Some(y) = iter.next_back() {
                        back.push(y);
                    }
                }
                front.extend(back.into_iter().rev());
                assert_eq!(front, expected_children.iter().map(|c| LoudsIndex(*c)).collect::<Vec<LoudsIndex>>());
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, vec!()),

        t2_1: ("10_10_0", 1, vec!(2)),
        t2_2: ("10_10_0", 2, vec!()),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, vec!(2, 3, 4)),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, vec!(6)),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, vec!()),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, vec!(9, 10, 11)),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, vec!()),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, vec!()),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, vec!(15)),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, vec!(17, 18)),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, vec!()),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, vec!()),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, vec!()),
    }
}

#[cfg(test)]
mod parent_to_children_failure_tests {
    use crate::{Louds, LoudsNodeNum};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, node_num) = $value;
                let louds = Louds::from(in_s);
                let _: Vec<_> = louds.parent_to_children(LoudsNodeNum(node_num));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 0),
        t1_2: ("10_0", 2),

        t2_1: ("10_10_0", 0),
        t2_2: ("10_10_0", 3),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
    }
}
