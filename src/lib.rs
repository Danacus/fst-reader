// Copyright 2023 The Regents of the University of California
// Copyright 2024 Cornell University
// released under BSD 3-Clause License
// author: Kevin Laeufer <laeufer@cornell.edu>

mod fastlz;
mod io;
mod reader;
mod types;

pub use io::ReaderError;
pub use reader::{
    is_fst_file, FstFilter, FstHeader, FstReader, FstSignalValue, InputVariant, ReadHierarchy,
};
pub use types::{
    FstHierarchyEntry, FstScopeType, FstSignalHandle, FstVarDirection, FstVarType, FstVhdlDataType,
    FstVhdlVarType,
};
