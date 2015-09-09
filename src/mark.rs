use std;

pub trait Marker {
    fn mark(&mut self, s: usize) -> &Marker;
    fn is_marked(&self, s: usize) -> bool;
}

pub struct TMarker {
    marks: Vec<u32>,
    marking: u32
}

impl Marker for TMarker {
    fn mark(&mut self, s: usize) -> &Marker {
        self.marks[s] = self.marking;
        self
    }
    fn is_marked(&self, s: usize) -> bool {
        self.marks[s] == self.marking
    }
}
impl TMarker {
    pub fn new(n: usize) -> TMarker {
        TMarker { marks: vec![0; n], marking: 1 }
    }
    pub fn reset(&mut self) -> &TMarker {
        if self.marking < std::u32::MAX {
            self.marking += 1;
        } else {
            for i in &mut self.marks {
                *i = 0;
            }
        }
        self
    }
}
