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
