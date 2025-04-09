pub struct Four {
    pub num: Option<u16>
}
pub struct Three {
    pub four: Option<Four>
}
pub struct Two {
  pub three: Option<Three> 
}
pub struct One {
   pub two: Option<Two>
    
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        Some(self.two?.three?.four?.num?)
    }
}
