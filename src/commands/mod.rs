mod deploy;
mod listen;

pub fn listen() {
    listen::run();
}
pub fn deploy() {
    deploy::run();
}
