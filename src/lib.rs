pub fn reply(message: &str) -> &str {
    fn is_yelling(msg: &str) -> bool {
        let has_letter = msg.chars().any(|c| c.is_alphabetic());
        has_letter && msg == msg.to_uppercase()
    }

    fn is_question(msg: &str) -> bool {
        msg.trim_end().ends_with("?")
    }

    fn is_silence(msg: &str) -> bool {
        msg.trim().is_empty()
    }

    match message {
        msg if is_silence(msg)=>"Fine. Be that way!",
        msg if is_yelling(msg) && is_question(msg)=>"Calm down, I know what I'm doing!",
        msg if is_yelling(msg)=>"Whoa, chill out!",
        msg if is_question(msg)=>"Sure.",
        _ => "Whatever."
    }
}
