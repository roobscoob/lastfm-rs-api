pub mod public;

pub trait Capability {}
pub enum ReadPublic {}
pub enum WriteUser {}
impl Capability for ReadPublic {}
impl Capability for WriteUser {}

pub trait Enables<C: Capability> {}
