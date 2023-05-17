crate::settings!(ChatSettings {
    /// Toggles global chat on or off.
    #[serde(with = "crate::serde::bool")]
    global_chat: bool = true,

    chat_streams: String = format!("s,r,a,w,y,sh,f,all"),

    /// The first welcome message visible in the chat panel. This will be displayed immediately after player login. you can use RGB colours to chance the colour of the welcome message. You can also use <LINE> to create a separate lines within your text. Use: <RGB:1,0,0> This message will show up red!
    server_welcome_message: String = format!("Welcome to Project Zomboid Multiplayer! <LINE> <LINE> To interact with the Chat panel: press Tab, T, or Enter. <LINE> <LINE> The Tab key will change the target stream of the message. <LINE> <LINE> Global Streams: /all <LINE> Local Streams: /say, /yell <LINE> Special Steams: /whisper, /safehouse, /faction. <LINE> <LINE> Press the Up arrow to cycle through your message history. Click the Gear icon to customize chat. <LINE> <LINE> Happy surviving!"),

    /// If checked, every time a player dies a global message will be displayed in the chat
    #[serde(with = "crate::serde::bool")]
    announce_death: bool = false
});
