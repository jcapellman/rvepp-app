mod rtfm;

trait ProtectionLayer {
    fn initialize(&self);

    fn shutdown(&self);

    fn run(&self);
}