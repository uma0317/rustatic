use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    App::new("clapex")
        .version("0.1.0")                       // バージョン情報
        .author("myname <myname@mail.com>")     // 作者情報
        .about("Clap Example CLI")              // このアプリについて
        .arg(Arg::with_name("pa")               // 位置引数を定義
        .help("sample positional argument")     // ヘルプメッセージ
        .required(true)                         // この引数は必須であることを定義
        )
        .arg(Arg::with_name("flg")              // フラグを定義
            .help("sample flag")                // ヘルプメッセージ
            .short("f")                         // ショートコマンド
            .long("flag")                       // ロングコマンド
        )
        .arg(Arg::with_name("opt")              // オプションを定義
            .help("sample option")              // ヘルプメッセージ
            .short("o")                         // ショートコマンド
            .long("opt")                        // ロングコマンド
            .takes_value(true)                  // 値を持つことを定義
        )
        .subcommand(SubCommand::with_name("sub")// サブコマンドを定義
            .about("sample subcommand")         // このサブコマンドについて
            .arg(Arg::with_name("subflg")       // フラグを定義
                .help("sample flag by sub")     // ヘルプメッセージ
                .short("f")                     // ショートコマンド
                .long("flag")                   // ロングコマンド
            )
        )
        .subcommand(SubCommand::with_name("add")
            .about("add folder to list")
            .arg(Arg::with_name("Target Project path")
                .help("Target Project path [\".\" = current path]")
                .required(true)
                .takes_value(true) 
            )
            .arg(Arg::with_name("New Boilerplate name")
                .help("New Boilerplate name")
                .takes_value(true) 
            )
            .arg(Arg::with_name("force")
                .help("Overwrite if duplicates")
                .short("f")
                .long("force")
            )
        )
}