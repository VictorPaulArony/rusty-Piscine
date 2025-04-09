
struct Message {
    content: String,
}
impl Message{
    pub fn new(content:String) -> Self {
        Message{content}
    }
    pub fn send_ms(&self) -> Option<&str>{
        if self.content.is_empty() || self.content.contains("stupid"){
            None
        }else {
            Some(&self.content)
        }
    }
}



pub fn check_ms(message: &str) -> Result<&str, &str> {
    let sms = Message::new(message.to_string());
   match sms.send_ms() {
    Some(_) => Ok(message),
    None => Err("ERROR: illegal"),
   }
}

