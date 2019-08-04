use super::*;

impl Materializer for Frag {
    fn merge(&mut self, other: &Frag) {
        if let Frag::Base(ref mut base) = self {
            base.apply(other);
        } else {
            panic!("expected base to be the first node");
        }
    }
}
