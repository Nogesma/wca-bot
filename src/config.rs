use serenity::all::Color;
use std::{collections::HashMap, sync::LazyLock};

pub static COUNTRIES: LazyLock<HashMap<&str, (&str, &str)>> = LazyLock::new(|| {
    HashMap::from([
        ("FR", ("🇫🇷", "France")),
        ("GB", ("🇬🇧", "Royaume-Uni")),
        ("BE", ("🇧🇪", "Belgique")),
        ("CH", ("🇨🇭", "Suisse")),
        ("DE", ("🇩🇪", "Allemagne")),
        ("NL", ("🇳🇱", "Pays-Bas")),
        ("ES", ("🇪🇸", "Espagne")),
        ("PT", ("🇵🇹", "Portugal")),
        ("IT", ("🇮🇹", "Italie")),
        ("CA", ("🇨🇦", "Canada")),
        ("AD", ("🇦🇩", "Andorre")),
        ("MC", ("🇲🇨", "Monaco")),
        ("SM", ("🇸🇲", "Saint-Marin")),
        ("LU", ("🇱🇺", "Luxembourg")),
        ("LI", ("🇱🇮", "Liechtenstein")),
        ("CL", ("🇨🇱", "Chili")),
        ("MA", ("🇲🇦", "Maroc")),
    ])
});

pub static EVENT_EMOJI: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("333", "<,3x3solved:889577416660516875>"),
        ("222", "<:2x2x2:889577416861810728>"),
        ("    444", "<:4x4x4:889577417109299231>"),
        ("555", "<:5x5x5:889577417285468250>"),
        ("666", "<:6x6x6:889577417012817981>"),
        ("777", "<:7x7x7:889577416857624597>"),
        ("skewb", "<:skewb:889577417616785408>"),
        ("minx", "<:megaminx:889577418262741072>"),
        ("sq1", "<:squane:889577417306419221>"),
        ("333oh", "🖐️"),
        ("333bf", "<:3BLD:889577417293844540>"),
        ("444bf", "<:4BLD:889577417167999077>"),
        ("555bf", "<:5BLD:889577417021218877>"),
        ("333fm", "✍️"),
        ("333mbf", "🧠"),
        ("clock", "<:clock_event:889577417692282960>"),
        ("pyram", "<:pyraminx:889577417943969862>"),
    ])
});

pub static TAG_COLOR: LazyLock<HashMap<&str, Color>> = LazyLock::new(|| {
    HashMap::from([
        ("WR", Color::from_rgb(0xf4, 0x43, 0x36)),
        ("CR", Color::from_rgb(0xff, 0xeb, 0x3b)),
        ("NR", Color::from_rgb(0x00, 0xe6, 0x76)),
    ])
});
