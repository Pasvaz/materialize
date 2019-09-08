// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

use dataflow_types::Timestamp;
use std::cell::RefCell;
use std::rc::Rc;
use timely::dataflow::operators::Capability;

mod kafka;
mod local;
mod util;

pub use kafka::kafka;
pub use local::local;

pub type SharedCapability = Rc<RefCell<Capability<Timestamp>>>;