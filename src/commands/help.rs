use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let message: &str = "
👋 The ultimate hacking bot
Brought to you by the folks over at https://discord.com/invite/zYTM3rZM4T

```
$ares aGVsbG8=
```

Decodes the text aGVsbG8= with Ares, the next generation of Ciphey <http://github.com/bee-san/Ares>

```
$ares https://pastebin.com/raw/05umSkne
```

If your file is too large to copy & paste you can upload it to pastebin, get the raw link and Ares will decode it.

```
$what 192.168.0.1
```

Run Lemmeknow / PyWhat on the input to identify it <https://github.com/swanandx/lemmeknow>

```
$ping
```

PONG!

```
$ciphey aGVsbG8=
```

Runs Ciphey on the given text.

```
$sth 5f4dcc3b5aa765d61d8327deb882cf99
```

Runs Search-That-Hash on the hash.
    ";

    msg.reply(ctx, message).await?;
    Ok(())
}
