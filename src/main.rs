use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use http::Method;
use rand::seq::SliceRandom;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::info;

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins; adjust as necessary
        .allow_methods(vec![Method::GET]) // Specify allowed methods
        .allow_headers(Any);

    let router = Router::new()
        .nest_service("/", ServeDir::new("frontend/dist"))
        .route("/word", get(word))
        .layer(middleware::from_fn(log_ip))
        .layer(cors);

    Ok(router.into())
}

async fn word() -> String {
    let mut rng = rand::thread_rng();
    WORDS.choose(&mut rng).map_or_else(String::new, |w| (*w).to_string())
}

async fn log_ip(req: Request<Body>, next: Next) -> Response {
    // let mut head = "REMOTE_ADDR";
    // let ip = req
    //     .headers()
    //     .get(head)
    //     .and_then(|hv| hv.to_str().ok())
    //     .or_else(|| {
    //         head = "HTTP_CLIENT_IP";
    //         req.headers().get(head).and_then(|hv| hv.to_str().ok())
    //     })
    //     .or_else(|| {
    //         head = "x-real-ip";
    //         req.headers().get(head).and_then(|hv| hv.to_str().ok())
    //     })
    //     .or_else(|| {
    //         head = "cf-connection-ip";
    //         req.headers().get(head).and_then(|hv| hv.to_str().ok())
    //     })
    //     .or_else(|| {
    //         head = "HTTP_X_FORWARDED_FOR";
    //         req.headers().get(head).and_then(|hv| hv.to_str().ok())
    //     })
    //     .or_else(|| {
    //         head = "HTTP_FORWARDED_FOR";
    //         req.headers().get(head).and_then(|hv| hv.to_str().ok())
    //     })
    //     .unwrap_or("Unknown");

    // if format!("{ip}") == "Unknown" {
    //     head = "none"
    // }

    // info!(PATH = req.uri().path().to_string(), IP = ip);
    info!(PATH = req.uri().path().to_string());

    next.run(req).await
}

static WORDS: &[&str; 990] = &[
    "Abbau", "Abend", "Abgas", "Abhub", "Abruf", "Absud", "Abtei", "Abweg", "Abzug", "Achse",
    "Acker", "Acryl", "Adept", "Adern", "Adler", "Affen", "After", "Agave", "Agent", "Agrar",
    "Ahorn", "Akten", "Aktie", "Aktor", "Alarm", "Alben", "Album", "Algen", "Alibi", "Alien",
    "Allee", "Almen", "Alpen", "Altar", "Amber", "Ampel", "Amsel", "Anbau", "Anden", "Angel",
    "Anime", "Anker", "Anmut", "Anode", "Anruf", "Anzug", "Aorta", "Apfel", "Apnoe", "April",
    "Arbon", "Arche", "Areal", "Arena", "Argon", "Armee", "Armut", "Aroma", "Arsen", "Asche",
    "Asiat", "Asien", "Assel", "Astro", "Atlas", "Atoll", "Atome", "Audio", "Audit", "Augen",
    "Autor", "Axiom", "Azubi", "Bagel", "Bambi", "Bande", "Banjo", "Barde", "Basar", "Basis",
    "Baske", "Bauch", "Bauer", "Bayer", "Bazar", "Beere", "Beete", "Beile", "Beine", "Belag",
    "Beleg", "Beppo", "Beruf", "Besen", "Beton", "Bezug", "Bibel", "Biber", "Biene", "Biere",
    "Biest", "Biker", "Bilge", "Bimbo", "Bingo", "Binom", "Birke", "Birne", "Bison", "Bisse",
    "Blatt", "Blech", "Blick", "Blitz", "Block", "Blues", "Bluff", "Blume", "Bluse", "Board",
    "Boden", "Bohle", "Bohne", "Bojen", "Bombe", "Bongo", "Bonus", "Bonze", "Borax", "Borde",
    "Borke", "Borna", "Boson", "Botox", "Bowle", "Boxer", "Brand", "Brass", "Brief", "Brigg",
    "Brise", "Brite", "Brote", "Bruch", "Brust", "Buben", "Buden", "Buggy", "Bulle", "Burka",
    "Busch", "Busen", "Busse", "Bussi", "Butan", "Caddy", "Celle", "Cello", "Ceran", "Chaos",
    "Chaot", "Chemo", "Chili", "China", "Chlor", "Chrom", "Civil", "Clone", "Cloud", "Clown",
    "Coach", "Cobol", "Comic", "Corso", "Couch", "Curry", "Cyber", "Dachs", "Damen", "Dampf",
    "Datei", "Daten", "Dativ", "Datum", "Daube", "Dauer", "Daune", "Degen", "Deich", "Dekan",
    "Dekor", "Delir", "Demut", "Depot", "Derby", "Diebe", "Diele", "Dildo", "Diner", "Dingo",
    "Diode", "Dirne", "Disco", "Disko", "Dispo", "Docht", "Dogma", "Dolch", "Donau", "Dorne",
    "Dosen", "Dosis", "Draht", "Drama", "Dreck", "Dress", "Drill", "Drink", "Droge", "Duden",
    "Duell", "Duett", "Dummy", "Dunst", "Durst", "Dusel", "Ebola", "Echse", "Eifel", "Eifer",
    "Eimer", "Eisen", "Eiter", "Eklat", "Ekzem", "Elfen", "Elite", "Ellen", "Elsas", "Emoji",
    "Engel", "Enkel", "Enten", "Enzym", "Erbin", "Erbse", "Esche", "Eseln", "Essig", "Etage",
    "Etats", "Ether", "Ethik", "Ethos", "Eulen", "Euter", "Event", "Exile", "Fabel", "Fahne",
    "Fakir", "Falke", "Fanta", "Farbe", "Farce", "Farne", "Fasan", "Faser", "Fauna", "Faust",
    "Fazit", "Feder", "Feger", "Fehde", "Feier", "Felge", "Felle", "Femen", "Femur", "Ferme",
    "Ferse", "Feten", "Fetus", "Feuer", "Fibel", "Fiber", "Figur", "Filet", "Finne", "Finte",
    "Firma", "Fisch", "Fjord", "Flaum", "Fleck", "Flirt", "Flora", "Fluch", "Fluge", "Fluor",
    "Fluse", "Flush", "Fluss", "Flyer", "Fokus", "Folie", "Foren", "Forst", "Forum", "Foyer",
    "Frack", "Freak", "Frist", "Front", "Frost", "Frust", "Fuchs", "Fuhre", "Funde", "Furie",
    "Furor", "Fusel", "Futon", "Futur", "Gabel", "Galle", "Gamer", "Garde", "Gasse", "Gassi",
    "Gatte", "Gaube", "Gaudi", "Gauen", "Geber", "Gebet", "Gecko", "Geier", "Geist", "Gelee",
    "Gemme", "Genie", "Genom", "Genre", "Geste", "Getto", "Gicht", "Gilde", "Glanz", "Gleis",
    "Glied", "Golem", "Gosse", "Gouda", "Gramm", "Graph", "Greis", "Grimm", "Grips", "Grube",
    "Gruft", "Grund", "Gummi", "Gunst", "Gyros", "Hader", "Hades", "Hafen", "Hafer", "Hagel",
    "Haiku", "Halde", "Handy", "Harem", "Harfe", "Hartz", "Hasch", "Hasen", "Haube", "Hauer",
    "Haupt", "Hebel", "Heber", "Hecht", "Heere", "Heide", "Hesse", "Hexer", "Hirse", "Hirte",
    "Hitze", "Hobby", "Hobel", "Hoden", "Honig", "Horde", "Hosen", "Hotel", "Hufen", "Humor",
    "Humus", "Hydra", "Hydro", "Ideen", "Idiom", "Idiot", "Idyll", "Ikone", "Imker", "Inbus",
    "Index", "Insel", "Intro", "Inuit", "Islam", "Jacke", "Japan", "Jeans", "Jesus", "Joint",
    "Joker", "Jubel", "Judas", "Jumbo", "Juror", "Juwel", "Kabel", "Kader", "Kakao", "Kamel",
    "Kamin", "Kampf", "Kanal", "Kanne", "Kanon", "Karat", "Karma", "Kasse", "Kater", "Katze",
    "Kebab", "Kekse", "Kelch", "Kelle", "Kelte", "Kerle", "Kerne", "Kerze", "Kieme", "Kimme",
    "Kiosk", "Kippa", "Kiste", "Klima", "Klotz", "Kluft", "Knabe", "Knast", "Knauf", "Knick",
    "Knopf", "Koala", "Kobra", "Kodex", "Kohle", "Kojen", "Kokon", "Kokos", "Kolik", "Komet",
    "Komik", "Komma", "Kopie", "Koran", "Krach", "Krake", "Kranz", "Kraut", "Krebs", "Kreis",
    "Krill", "Krimi", "Kripo", "Krise", "Krone", "Kufen", "Kugel", "Kuhle", "Kunde", "Kunst",
    "Kuppe", "Kurse", "Kutte", "Labor", "Lachs", "Lager", "Laich", "Laien", "Laken", "Lampe",
    "Lanze", "Larve", "Laser", "Lasso", "Lasur", "Latex", "Latte", "Laube", "Lauch", "Laune",
    "Leder", "Leier", "Leine", "Lemur", "Lende", "Lepra", "Lesbe", "Leser", "Leute", "Level",
    "Lexus", "Liesl", "Lilie", "Limbo", "Limit", "Linie", "Lippe", "Liter", "Lobby", "Lodge",
    "Logik", "Lokus", "Lolli", "Lotto", "Lotus", "Luchs", "Luder", "Lumen", "Lunge", "Lunte",
    "Lurch", "Luxus", "Lyrik", "Macho", "Macke", "Maden", "Mafia", "Magen", "Magma", "Mainz",
    "Major", "Makel", "Maler", "Mambo", "Manta", "Mappe", "Marke", "Markt", "Maske", "Masse",
    "Media", "Meile", "Meise", "Memme", "Mensa", "Meter", "Metro", "Meute", "Miene", "Mikro",
    "Milbe", "Milch", "Miliz", "Minze", "Mixer", "Model", "Modem", "Modul", "Modus", "Mogul",
    "Molke", "Monat", "Moped", "Mopps", "Moral", "Motiv", "Motor", "Motte", "Motto", "Muffe",
    "Mulch", "Mulde", "Mumie", "Mumms", "Mumps", "Murks", "Musik", "Nabel", "Nacht", "Nadel",
    "Nagel", "Nager", "Namen", "Narbe", "Natur", "Nebel", "Neffe", "Nelke", "Nexus", "Niere",
    "Ninja", "Nisse", "Nixen", "Nomen", "Nonne", "Notar", "Notiz", "Novum", "Nudel", "Nugat",
    "Nylon", "Obhut", "Ochse", "Ohren", "Oktan", "Oktav", "Oldie", "Olymp", "Onkel", "Opiat",
    "Opium", "Optik", "Orbit", "Orden", "Order", "Organ", "Orgel", "Orgie", "Orion", "Orkan",
    "Ornat", "Osten", "Otter", "Oxide", "Ozean", "Pacht", "Paket", "Palme", "Pampa", "Pampe",
    "Panda", "Panik", "Panne", "Papst", "Paris", "Parka", "Party", "Pasta", "Paste", "Pater",
    "Pegel", "Penis", "Penny", "Pesto", "Pfahl", "Pfalz", "Pfand", "Pfeil", "Pferd", "Pflug",
    "Pfote", "Pfund", "Phase", "Pille", "Pilot", "Pilze", "Pippi", "Pirat", "Piste", "Pixel",
    "Pizza", "Platz", "Pluto", "Pocke", "Pokal", "Poker", "Polio", "Polka", "Polle", "Polyp",
    "Popel", "Poren", "Porno", "Porto", "Posse", "Prinz", "Prise", "Profi", "Promi", "Prosa",
    "Protz", "Proxy", "Prunk", "Psalm", "Pudel", "Puder", "Pulli", "Punkt", "Puppe", "Pussy",
    "Pylon", "Pyrit", "Qualm", "Quark", "Quarz", "Quell", "Quirl", "Quote", "Rabbi", "Rache",
    "Radar", "Radau", "Radio", "Radon", "Rampe", "Ranch", "Raser", "Rasse", "Rasur", "Ratte",
    "Raudi", "Rauke", "Raupe", "Regal", "Regel", "Regie", "Remix", "Rente", "Reste", "Revue",
    "Rille", "Rinde", "Rippe", "Rispe", "Ritus", "Rodeo", "Rollo", "Roman", "Rosen", "Rotor",
    "Route", "Rover", "Rowdy", "Rubin", "Rudel", "Ruder", "Rugby", "Ruine", "Rumpf", "Runen",
    "Sache", "Sakko", "Salat", "Saldo", "Salon", "Salsa", "Salto", "Salut", "Salve", "Samba",
    "Samen", "Satan", "Satin", "Satyr", "Sauce", "Sauna", "Scala", "Schaf", "Schar", "Schub",
    "Schuh", "Schur", "Seele", "Segel", "Segen", "Seher", "Seide", "Seite", "Sekte", "Senat",
    "Sense", "Sepia", "Serie", "Serum", "Sesam", "Shirt", "Sicht", "Siele", "Silbe", "Sinus",
    "Sippe", "Sirup", "Sitte", "Skala", "Slang", "Snack", "Socke", "Sohle", "Sohne", "Sonar",
    "Sonde", "Sorte", "Spalt", "Spatz", "Speck", "Speer", "Spion", "Spore", "Sport", "Spott",
    "Spray", "Spreu", "Sprit", "Spurt", "Staat", "Stadt", "Stall", "Stamm", "Start", "Staub",
    "Steak", "Stein", "Stern", "Stiel", "Stier", "Stift", "Stirn", "Stock", "Stoff", "Strom",
    "Stube", "Stuhl", "Sturm", "Sturz", "Stuss", "Stute", "Sumpf", "Suppe", "Szene", "Tabak",
    "Tacho", "Tadel", "Tafel", "Tanga", "Tango", "Tanne", "Tante", "Tapir", "Tarif", "Tarot",
    "Taser", "Tasse", "Tatze", "Tegel", "Teich", "Tempo", "Tenor", "Theke", "Thema", "These",
    "Tiger", "Tilde", "Tinte", "Tirol", "Tisch", "Titan", "Titel", "Titte", "Toast", "Tobak",
    "Toner", "Tonic", "Tonne", "Toxin", "Trafo", "Traum", "Treff", "Trend", "Trick", "Troll",
    "Tropf", "Trost", "Truck", "Truhe", "Trupp", "Tubus", "Tukan", "Tulpe", "Tumor", "Tuner",
    "Tupel", "Turbo", "Tutor", "Twist", "Uhren", "Umbau", "Umweg", "Umzug", "Unart", "Unfug",
    "Union", "Unmut", "Unrat", "Unruh", "Untat", "Vasen", "Vater", "Venen", "Venus", "Verse",
    "Video", "Viech", "Villa", "Vinyl", "Viper", "Viren", "Virus", "Visum", "Vogel", "Votum",
    "Vulva", "Waage", "Waben", "Wachs", "Waden", "Waffe", "Wagon", "Waise", "Wampe", "Wange",
    "Wanne", "Wanst", "Wanze", "Warze", "Watte", "Weber", "Wedel", "Welpe", "Wesen", "Wespe",
    "Weste", "Wille", "Witwe", "Witze", "Woche", "Wohle", "Wolke", "Wonne", "Wrack", "Wucht",
    "Wulst", "Wurst", "Xenon", "Xerox", "Yacht", "Zacke", "Zange", "Zarge", "Zebra", "Zecke",
    "Zeder", "Zehen", "Zeile", "Zelle", "Zelot", "Ziege", "Zitat", "Zitze", "Zonen", "Zubau",
    "Zucht", "Zunft", "Zunge", "Zuruf", "Zweck", "Zweig", "Zwerg", "Zwirn", "Zwist", "Zyste",
];
