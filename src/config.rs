use serenity::all::{Color, EmojiId, ReactionType};
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

pub static EVENT_EMOJI: LazyLock<HashMap<&str, ReactionType>> = LazyLock::new(|| {
    HashMap::from([
        (
            "333",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577416660516875),
                name: Some("3x3solved".to_string()),
            },
        ),
        (
            "222",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577416861810728),
                name: Some("2x2x2".to_string()),
            },
        ),
        (
            "444",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417109299231),
                name: Some("4x4x4".to_string()),
            },
        ),
        (
            "555",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417285468250),
                name: Some("5x5x5".to_string()),
            },
        ),
        (
            "666",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417012817981),
                name: Some("6x6x6".to_string()),
            },
        ),
        (
            "777",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577416857624597),
                name: Some("7x7x7".to_string()),
            },
        ),
        (
            "skewb",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417616785408),
                name: Some("skewb".to_string()),
            },
        ),
        (
            "minx",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577418262741072),
                name: Some("megaminx".to_string()),
            },
        ),
        (
            "sq1",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417306419221),
                name: Some("square-1".to_string()),
            },
        ),
        ("333oh", ReactionType::Unicode("🖐️".to_string())),
        (
            "333bf",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417293844540),
                name: Some("3bld".to_string()),
            },
        ),
        (
            "444bf",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417167999077),
                name: Some("4bld".to_string()),
            },
        ),
        (
            "555bf",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417021218877),
                name: Some("5bld".to_string()),
            },
        ),
        ("333fm", ReactionType::Unicode("✍️".to_string())),
        ("333mbf", ReactionType::Unicode("🧠".to_string())),
        (
            "clock",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417692282960),
                name: Some("clock".to_string()),
            },
        ),
        (
            "pyram",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(889577417943969862),
                name: Some("pyraminx".to_string()),
            },
        ),
    ])
});

pub static TAG_COLOR: LazyLock<HashMap<&str, Color>> = LazyLock::new(|| {
    HashMap::from([
        ("WR", Color::from_rgb(0xf4, 0x43, 0x36)),
        ("CR", Color::from_rgb(0xff, 0xeb, 0x3b)),
        ("NR", Color::from_rgb(0x00, 0xe6, 0x76)),
    ])
});
