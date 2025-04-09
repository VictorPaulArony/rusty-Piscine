pub struct Four {
    num: Option<u16>
}
pub struct Three {
    four: Option<Four>
}
pub struct Two {
   three: Option<Three> 
}
pub struct One {
    two: Option<Two>
    
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        Some(self.two?.three?.four?.num?)
    }
}
