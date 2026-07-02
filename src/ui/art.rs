pub const LOGO: &str = r#"
▄▄▄▄▄   ▄   ▄ ▄   ▄ ▄▄▄▄▄     ▄▄▄▄▄▄     ▄▄▄▄▄   ▄▄▄▄   ▄▄▄▄▄  ▄▄▄▄▄
█   █   █   █ █▀▄ █ █          █    █    █   █  █    █  █   █  █    █
█▄▄▄▀   █   █ █ █▄█ █▄▄▄      ▄█▄       █▄▄▄▀  █    █  █▄▄▄█  █    █
█  ▀▄   █   █ █   █ █          █  ▀▄    █  ▀▄   █    █  █   █  █    █
█   █   ▀▄▄▄▀ █   █ █▄▄▄▄     ▀    ▀    █   █   ▀▄▄▄▀  █   █  █▄▄▄▄█
"#;

pub const LOGO_SMALL: &str = r#"
 ✦ R U N E   &   R O A D ✦
"#;

pub const FERRIS: &str = r#"
    ▄   ▄
   (o)_(o)
  /  ___  \
 ( (     ) )
  \_\   /_/
    ▀▀▀▀▀
"#;

/// An open book, for when a Library shelf does the talking.
pub const BOOK: &str = r#"
   ______ ______
  /      Y      \
 |  ~~~~ | ~~~~  |
 |  ~~~  |  ~~~  |
 |  ~~~~ | ~~~~  |
  \______|______/
     ~ open ~
"#;

/// A little ASCII portrait for the dialogue box, keyed by speaker name.
pub fn portrait(name: &str) -> &'static str {
    match name {
        "Elder Rowan" => {
            r#"
    .-~~~-.
   / ~ ~ ~ \
  |  (. .)  |
  |    ◡    |
   \  ___  /
  .-`-...-`-.
 /  ~shawl~  \
"#
        }
        "Baker Poppy" => {
            r#"
    ___n___
   ( flour )
   |  ^ ^  |
   |  (o o)|
   |   ◡◡  |
    \_____/
    ~fresh~
"#
        }
        "Well-keeper Bram" => {
            r#"
     _____
    /  =  \
   |  o o  |
   |   L   |
   | \___/ |
    \_____/
   ~19.6m!~
"#
        }
        "Wren" => {
            r#"
     \|/
    (leaf)
   |  ^ ^ |
   | (• •)|
   |  \o/ |
    \____/
   ~zoom!~
"#
        }
        "Forager Maren" => {
            r#"
    _...._
   / hood \
  |  ( - -)|
  |    ~   |
   \ .::. /
    basket
   ~careful~
"#
        }
        "Shepherd Ambrose" => {
            r#"
     ____
    ( hat )
   |  - -  |
   |   o   |
   |  zZz  |
    \_____/
   ~5 more min~
"#
        }
        "Ferryman Wick" => {
            r#"
     _/\_
    ( cap )
   |  o_o  |
   |   -   |
   |  ~~~  |
    \_____/
   ~token?~
"#
        }
        "Fisher Juniper" => {
            r#"
     ___
    ( ~ )--,
   |  ^ ^ | |
   | (o o)| J
   |   ◡  |/
    \____/
   ~shhh~
"#
        }
        "Hermit Morrow" => {
            r#"
    .~~~~.
   ( beard )
   |  . .  |
   |  ___  |
   | (   ) |
    \~~~~~/
   ~the river~
"#
        }
        "Archivist Elm" => {
            r#"
    ______
   | specs |
   |  ⌐-⌐  |
   |   =   |
   |  ___  |
    \_____/
   ~stamped~
"#
        }
        "The Stone Golem" => {
            r#"
   ▄▄▄▄▄▄▄
   █ ▓▓▓ █
   █ ■ ■ █
   █  ▂  █
   ▀█▓▓▓█▀
   ~ADMISSIONS~
"#
        }
        "Sage Alderly" => {
            r#"
     _✦_
    / ~ \
   |  * *  |
   |   ◡   |
   |  ___  |
    \_____/
   ~welcome~
"#
        }
        "Signpost" => {
            r#"

    ┌─────┐
    │ ➤➤➤ │
    └──┬──┘
       │
    ~~~┴~~~
"#
        }
        _ => FERRIS,
    }
}

pub const SPARKLES: &str = r#"
   ✦     ✧    ✦
     ✧  ✦   ✧
   ✦   THE   ✦
  ✧   RUNE    ✧
   ✦  HOLDS  ✦
     ✧  ✦   ✧
   ✦     ✧    ✦
"#;
