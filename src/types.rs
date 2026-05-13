use iced::Element;

pub trait Component {
    type Message;
    
    fn new() -> Self;
    fn update(&mut self, msg: Self::Message) -> ();
    fn view(&self) -> Element<'_, Self::Message>;
}
