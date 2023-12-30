#![allow(dead_code)]

pub use std::array;
pub use std::borrow::Cow;
pub use std::cell::{Cell, RefCell};
pub use std::cmp::{self, max, min, Ordering, Reverse};
pub use std::collections::{
    hash_map::Entry, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
pub use std::convert::{TryFrom, TryInto};
pub use std::hash::{self, Hash};
pub use std::iter::{self, FromIterator};
pub use std::mem::{self, forget, replace, swap, take};
pub use std::ops::ControlFlow;
pub use std::rc::{Rc, Weak};
pub use std::sync::{Arc, Weak as AWeak};

pub use ::arrayvec::{self, ArrayVec};
pub use ::bitflags::bitflags;
pub use ::bitvec::{self, array::BitArray, bitarr, bitbox, bitvec, boxed::BitBox, vec::BitVec};
pub use ::derive_more::{self, Add, AddAssign, Constructor};
pub use ::indexmap::{self, IndexMap, IndexSet};
pub use ::itertools::{self, chain as ichain, iproduct, izip, Either, Itertools};
pub use ::num::{
    self,
    integer::{gcd_lcm, sqrt},
};
pub use ::once_cell::{
    self,
    sync::{Lazy, OnceCell},
};
pub use ::ord_by_key::ord_eq_by_key_selector as ord_by_key;
pub use ::parking_lot::{self, Mutex, RwLock};
pub use ::parse_display::{self, Display, FromStr};
pub use ::rayon::{self, prelude::*};
pub use ::regex::{self, Regex};
pub use ::rustc_hash::{self, FxHashMap, FxHashSet};

pub use crate::helpers::array::{ArrayExt as _, ArrayFromIterExt as _};
pub use crate::helpers::bfs::*;
pub use crate::helpers::eat::*;
pub use crate::helpers::grid::*;
pub use crate::helpers::grid3d::*;
pub use crate::helpers::iter::IteratorExt as _;
pub use crate::helpers::ocr::*;
pub use crate::helpers::par::ParFindChunkedExt as _;
pub use crate::helpers::slice::SliceExt as _;

pub type FxIndexMap<K, V> = IndexMap<K, V, hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub type FxIndexSet<T> = IndexSet<T, hash::BuildHasherDefault<rustc_hash::FxHasher>>;
