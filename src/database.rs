use crate::request::Message;

pub trait InfoLogger {
    fn save_message(message: &Message);

    fn save_all(messages: &Vec<Message>) {
        messages.iter().for_each(Self::save_message);
    }

    fn load() -> Box<Self>;
}
