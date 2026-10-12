use freenet_stdlib::prelude::*;

pub type DivergentGen = fn() -> Option<(State<'static>, State<'static>)>;
