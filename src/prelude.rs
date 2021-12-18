#![allow(dead_code)]

pub use std::borrow::Cow;
pub use std::cell::{Cell, RefCell};
pub use std::cmp::{self, max, min, Ordering, Reverse};
pub use std::collections::{
    hash_map::Entry, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
pub use std::convert::{TryFrom, TryInto};
pub use std::hash::{self, Hash};
pub use std::iter::{self, FromIterator};
pub use std::mem::{self, replace, swap, take};
pub use std::rc::{Rc, Weak};
pub use std::sync::{Arc, Weak as AWeak};

pub use arrayvec::{self, ArrayVec};
pub use bitflags::bitflags;
pub use derive_more::{self, Add, AddAssign, Constructor};
pub use fxhash::{self, FxHashMap, FxHashSet};
pub use itertools::{self, Either, Itertools};
pub use num::{
    self,
    integer::{gcd_lcm, sqrt},
};
pub use ord_by_key::ord_eq_by_key_selector as ord_by_key;
pub use parse_display::{self, Display, FromStr};
pub use rayon::{self, prelude::*};
pub use regex::{self, Regex};

pub use crate::helpers::array::ArrayExt as _;
pub use crate::helpers::grid::*;
pub use crate::helpers::eat::*;
