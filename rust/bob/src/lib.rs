pub fn reply(msg: &str) -> &str {
    return match msg {
               "" => "Fine. Be that way!",
               msg if msg.ends_with("?") => "Sure.",
               msg if msg == msg.to_uppercase() => "Whoa, chill out!",
               _ => "Whatever.",
           };
}
