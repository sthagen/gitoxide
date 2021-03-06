use crate::pack;
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when reading the pack or creating a temporary file")
            from()
            source(err)
        }
        PackEntryDecode(err: pack::data::iter::Error) {
            display("A pack entry could not be extracted")
            from()
            source(err)
        }
        Unsupported(kind: pack::index::Kind) {
            display("Indices of type {} cannot be written, only {} are supported", *kind as usize, pack::index::Kind::default() as usize)
        }
        IteratorInvariantNoRefDelta {
            display("Ref delta objects are not supported as there is no way to look them up. Resolve them beforehand.")
        }
        IteratorInvariantTrailer {
            display("The iterator failed to set a trailing hash over all prior pack entries in the last provided entry")
        }
        IteratorInvariantBasesPresent {
            display("Did not encounter a single base")
        }
        IteratorInvariantTooManyObjects(num_objects: usize) {
            display("Only u32::MAX objects can be stored in a pack, found {}", num_objects)
        }
        IteratorInvariantBaseOffset(pack_offset: u64, distance: u64) {
            display("{} is not a valid offset for pack offset {}", distance, pack_offset)
        }
        Tree(err: pack::tree::Error) {
            display("An invariant regarding the delta tree did not hold")
            source(err)
            from()
        }
        TreeTraversal(err: pack::tree::traverse::Error) {
            display("Tree traversal failed")
            source(err)
            from()
        }
    }
}
