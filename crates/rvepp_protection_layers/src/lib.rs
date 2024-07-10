pub mod rtfm;

pub trait ProtectionLayer {
    fn initialize(&mut self);

    fn shutdown(&mut self);

    fn run(&mut self);
}