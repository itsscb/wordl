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
    WORDS.choose(&mut rng).unwrap().to_string()
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

static WORDS: &[&str; 1929] = &[
    "Abbau", "Abend", "Abgas", "Abhub", "Abruf", "Absud", "Abtei", "Abweg", "Abzug", "Achse",
    "Acker", "Acryl", "Adept", "Adern", "Adler", "Affen", "After", "Agave", "Agent", "Agrar",
    "Ahorn", "Akkus", "Akten", "Aktie", "Aktor", "Alarm", "Alben", "Album", "Algen", "Alibi",
    "Alien", "Allee", "Almen", "Alpen", "Altar", "Amber", "Ampel", "Amsel", "Anbau", "Anden",
    "Angel", "Anime", "Anker", "Anmut", "Anode", "Anruf", "Anzug", "Aorta", "Apfel", "Apnoe",
    "April", "Arbon", "Arche", "Areal", "Arena", "Argon", "Armee", "Armut", "Aroma", "Arsen",
    "Asche", "Asiat", "Asien", "Assel", "Astro", "Atlas", "Atoll", "Atome", "Audio", "Audit",
    "Augen", "Autor", "Autos", "Axiom", "Azubi", "Babys", "Bagel", "Bambi", "Bande", "Banjo",
    "Barde", "Basar", "Basis", "Baske", "Bauch", "Bauer", "Bayer", "Bazar", "Becks", "Beere",
    "Beete", "Beile", "Beine", "Belag", "Beleg", "Beppo", "Beruf", "Besen", "Beton", "Bezug",
    "Bibel", "Biber", "Biene", "Biere", "Biest", "Biker", "Bilge", "Bimbo", "Bingo", "Binom",
    "Birke", "Birne", "Bison", "Bisse", "Blatt", "Blech", "Blick", "Blitz", "Block", "Blues",
    "Bluff", "Blume", "Bluse", "Board", "Boden", "Bohle", "Bohne", "Bojen", "Bombe", "Bongo",
    "Bonus", "Bonze", "Borax", "Borde", "Borke", "Borna", "Boson", "Botox", "Bowle", "Boxer",
    "Brand", "Brass", "Brief", "Brigg", "Brise", "Brite", "Brote", "Bruch", "Brust", "Buben",
    "Buden", "Buggy", "Bulle", "Burka", "Busch", "Busen", "Busse", "Bussi", "Butan", "Caddy",
    "Celle", "Cello", "Cents", "Ceran", "Chaos", "Chaot", "Chefs", "Chemo", "Chili", "China",
    "Chips", "Chlor", "Chrom", "Civil", "Clans", "Clips", "Clone", "Cloud", "Clown", "Clubs",
    "Coach", "Cobol", "Codes", "Comic", "Corso", "Couch", "Coups", "Cross", "Curry", "Cyber",
    "Dachs", "Damen", "Dampf", "Datei", "Daten", "Dates", "Dativ", "Datum", "Daube", "Dauer",
    "Daune", "Degen", "Deich", "Dekan", "Dekor", "Delir", "Demut", "Depot", "Derby", "Diebe",
    "Diele", "Dildo", "Diner", "Dingo", "Dings", "Dinos", "Diode", "Dirne", "Disco", "Disko",
    "Dispo", "Docht", "Dodos", "Dogma", "Dolch", "Donau", "Dorne", "Dosen", "Dosis", "Draht",
    "Drama", "Dreck", "Dress", "Drill", "Drink", "Droge", "Drops", "Drums", "Duden", "Duell",
    "Duett", "Dummy", "Dunst", "Durst", "Dusel", "Ebola", "Echos", "Echse", "Eifel", "Eifer",
    "Eimer", "Eisen", "Eiter", "Eklat", "Ekzem", "Elfen", "Elite", "Ellen", "Elsas", "Emoji",
    "Engel", "Enkel", "Enten", "Enzym", "Erbin", "Erbse", "Esche", "Eseln", "Essig", "Etage",
    "Etats", "Ether", "Ethik", "Ethos", "Etuis", "Eulen", "Euros", "Euter", "Event", "Exile",
    "Fabel", "Fachs", "Fahne", "Fakes", "Fakir", "Falke", "Fanta", "Farbe", "Farce", "Farne",
    "Fasan", "Faser", "Fauna", "Faust", "Fazit", "Feder", "Feger", "Fehde", "Feier", "Felge",
    "Felle", "Femen", "Femur", "Ferme", "Ferse", "Feten", "Fetus", "Feuer", "Fibel", "Fiber",
    "Figur", "Filet", "Finne", "Finte", "Firma", "Fisch", "Fjord", "Flaum", "Fleck", "Flips",
    "Flirt", "Flora", "Fluch", "Fluge", "Fluor", "Fluse", "Flush", "Fluss", "Flyer", "Fokus",
    "Folie", "Fonds", "Foren", "Forst", "Forts", "Forum", "Fotos", "Fouls", "Foyer", "Frack",
    "Freak", "Frist", "Front", "Frost", "Frust", "Fuchs", "Fuhre", "Funde", "Furie", "Furor",
    "Fusel", "Futon", "Futur", "Gabel", "Galle", "Gamer", "Games", "Garde", "Gasse", "Gassi",
    "Gatte", "Gaube", "Gaudi", "Gauen", "Geber", "Gebet", "Gecko", "Geier", "Geist", "Gelee",
    "Gemme", "Genie", "Genom", "Genre", "Geste", "Getto", "Gicht", "Gifts", "Gilde", "Glanz",
    "Gleis", "Glied", "Golem", "Gosse", "Gouda", "Gramm", "Graph", "Greis", "Grimm", "Grips",
    "Grube", "Gruft", "Grund", "Gummi", "Gunst", "Gyros", "Hader", "Hades", "Hafen", "Hafer",
    "Hagel", "Haiku", "Halde", "Handy", "Harem", "Harfe", "Hartz", "Hasch", "Hasen", "Haube",
    "Hauer", "Haupt", "Hebel", "Heber", "Hecht", "Heere", "Heide", "Hesse", "Hexer", "Hirse",
    "Hirte", "Hitze", "Hobby", "Hobel", "Hoden", "Honig", "Horde", "Hosen", "Hotel", "Hufen",
    "Humor", "Humus", "Hydra", "Hydro", "Ideen", "Idiom", "Idiot", "Idyll", "Ikone", "Imker",
    "Inbus", "Index", "Insel", "Intro", "Inuit", "Islam", "Jacke", "Japan", "Jeans", "Jesus",
    "Joint", "Joker", "Jubel", "Judas", "Jumbo", "Jungs", "Juror", "Juwel", "Kabel", "Kader",
    "Kakao", "Kakis", "Kamel", "Kamin", "Kampf", "Kanal", "Kanne", "Kanon", "Karat", "Karma",
    "Kasse", "Kater", "Katze", "Kebab", "Kekse", "Kelch", "Kelle", "Kelte", "Kerle", "Kerne",
    "Kerze", "Kieme", "Kilos", "Kimme", "Kiosk", "Kippa", "Kiste", "Kiwis", "Klima", "Klotz",
    "Kluft", "Knabe", "Knast", "Knauf", "Knick", "Knopf", "Koala", "Kobra", "Kodex", "Kohle",
    "Kojen", "Kokon", "Kokos", "Kolik", "Komet", "Komik", "Komma", "Kopie", "Koran", "Krach",
    "Krake", "Kranz", "Kraut", "Krebs", "Kreis", "Krill", "Krimi", "Kripo", "Krise", "Krone",
    "Kufen", "Kugel", "Kuhle", "Kunde", "Kunst", "Kuppe", "Kurse", "Kutte", "Labor", "Lachs",
    "Lager", "Laich", "Laien", "Laken", "Lamas", "Lampe", "Lanze", "Larve", "Laser", "Lasso",
    "Lasur", "Latex", "Latte", "Laube", "Lauch", "Laune", "Leder", "Leier", "Leine", "Lemur",
    "Lende", "Lepra", "Lesbe", "Leser", "Leute", "Level", "Lexus", "Liesl", "Lilie", "Limbo",
    "Limit", "Linie", "Lippe", "Liter", "Lobby", "Lodge", "Logik", "Lokus", "Lolli", "Lotto",
    "Lotus", "Luchs", "Luder", "Lumen", "Lunge", "Lunte", "Lurch", "Luxus", "Lyrik", "Macho",
    "Macke", "Maden", "Mafia", "Magen", "Magma", "Mainz", "Major", "Makel", "Maler", "Mambo",
    "Manta", "Mappe", "Marke", "Markt", "Maske", "Masse", "Media", "Meile", "Meise", "Memme",
    "Mensa", "Meter", "Metro", "Meute", "Miene", "Mikro", "Milbe", "Milch", "Miliz", "Minze",
    "Mixer", "Model", "Modem", "Modul", "Modus", "Mofas", "Mogul", "Molke", "Monat", "Moped",
    "Mopps", "Moral", "Motiv", "Motor", "Motte", "Motto", "Muffe", "Mulch", "Mulde", "Mumie",
    "Mumms", "Mumps", "Murks", "Musik", "Nabel", "Nacht", "Nadel", "Nagel", "Nager", "Namen",
    "Narbe", "Natur", "Nazis", "Nebel", "Neffe", "Nelke", "Nexus", "Niere", "Ninja", "Nisse",
    "Nixen", "Nomen", "Nonne", "Notar", "Notiz", "Novum", "Nudel", "Nugat", "Nylon", "Obhut",
    "Ochse", "Ohren", "Oktan", "Oktav", "Oldie", "Olymp", "Onkel", "Opiat", "Opium", "Optik",
    "Orbit", "Orden", "Order", "Organ", "Orgel", "Orgie", "Orion", "Orkan", "Ornat", "Osten",
    "Otter", "Oxide", "Ozean", "Pacht", "Paket", "Palme", "Pampa", "Pampe", "Panda", "Panik",
    "Panne", "Papst", "Paris", "Parka", "Party", "Pasta", "Paste", "Pater", "Pegel", "Penis",
    "Penny", "Pesto", "Pfahl", "Pfalz", "Pfand", "Pfeil", "Pferd", "Pflug", "Pfote", "Pfund",
    "Phase", "Pille", "Pilot", "Pilze", "Pimpf", "Pings", "Pinie", "Pinot", "Pinte", "Pinus",
    "Piotr", "Piper", "Pippi", "Pirat", "Pirna", "Pirol", "Pisas", "Piste", "Pixar", "Pixel",
    "Pizza", "Pjotr", "Plaid", "Plans", "Plato", "Platz", "Playa", "Plaza", "Plebs", "Plena",
    "Plins", "Plots", "Plugs", "Pluto", "Pneus", "Pocke", "Podex", "Poems", "Point", "Pokal",
    "Poker", "Poldi", "Polin", "Polio", "Polis", "Polka", "Polle", "Polly", "Polos", "Polyp",
    "Pomps", "Ponte", "Ponys", "Ponzi", "Pools", "Popel", "Popen", "Popos", "Poren", "Porno",
    "Porta", "Porti", "Porto", "Ports", "Posen", "Posex", "POSIX", "Posse", "Posts", "Potis",
    "Potte", "Potts", "Power", "Prada", "Prado", "Prags", "Praha", "Priel", "Priem", "Prien",
    "Prime", "Prinz", "Prior", "Prise", "PRISM", "Prius", "PRler", "Profi", "Prolo", "Promi",
    "Prosa", "Protz", "Proxy", "Prunk", "Psalm", "Pucks", "Pudel", "Puder", "Puffs", "Pulke",
    "Pulks", "Pulli", "Pulpa", "Pulpe", "Pults", "Pumas", "Pumps", "Punks", "Punkt", "Punze",
    "Puppe", "Purim", "Purin", "Pussy", "Puten", "Puter", "Putin", "Putte", "Putto", "Pylon",
    "Pyrit", "Qaida", "Qatar", "Quads", "Quais", "Quali", "Qualm", "Quant", "Quark", "Quart",
    "Quarz", "Quast", "Queen", "Quell", "Quest", "Queue", "Quick", "Quinn", "Quint", "Quirl",
    "Quito", "Quota", "Quote", "Rabat", "Rabbi", "Raben", "Rache", "Radar", "Radau", "Radek",
    "Rades", "Radio", "Radix", "Radom", "Radon", "Rahel", "Rahen", "Rahms", "Raine", "Rains",
    "Rakel", "Ralfs", "Ralle", "Ralph", "Rambo", "Ramon", "Rampe", "Ranch", "Rande", "Randy",
    "Ranft", "Range", "Rangs", "Ranze", "Raoul", "Rappe", "Raser", "Rasse", "Rasur", "Rates",
    "Ratte", "Raubs", "Raudi", "Rauke", "Raume", "Raums", "Raupe", "Ravel", "Raver", "Rayon",
    "Realo", "Reben", "Rebus", "Reede", "Reepe", "Reeps", "Regal", "Regel", "Regex", "Regie",
    "Rehau", "Rehen", "Reifs", "Reiki", "Reims", "Remix", "Remus", "Rente", "Rerum", "Resch",
    "Reset", "Reste", "Rests", "Reuse", "Reuss", "Reval", "Revue", "Rewes", "Rheda", "Rhein",
    "Rhema", "Rhode", "Rhone", "Riads", "Ricas", "Richy", "Ricke", "Ricky", "Ricos", "Riege",
    "Riehl", "Riesa", "Riese", "Riffe", "Riffs", "Rigas", "Rigor", "Rilke", "Rille", "Rinde",
    "Rinds", "Ringo", "Rioja", "Rippe", "Rispe", "Risse", "Ritas", "Riten", "Ritts", "Ritus",
    "River", "Roben", "Robin", "Robot", "Rocks", "Rocky", "Rodel", "Rodeo", "Rodin", "Rogen",
    "Roger", "Rohei", "Rohre", "Rohrs", "Rolex", "Rolfs", "Rolli", "Rollo", "Rolls", "Roman",
    "Romeo", "Romys", "Ronde", "Rondo", "Ronja", "Ronny", "Rosas", "Rosen", "Roses", "Rosie",
    "Rosis", "Rosse", "Rossi", "Rothe", "Rotor", "Rouen", "Rouge", "Route", "Rover", "Rowdy",
    "Royce", "Rubel", "Rubin", "Rubra", "Rucks", "Rudel", "Ruder", "Rudis", "Rudys", "Rufer",
    "Rufes", "Rufus", "Rugby", "Ruhla", "Ruhms", "Ruine", "Ruins", "Rumba", "Rumor", "Rumpf",
    "Runen", "Runge", "Rupie", "Ruppe", "Rushs", "Russe", "Ruten", "Ruths", "Rutte", "Ruwer",
    "Saale", "Saals", "Sache", "Sachs", "Sacks", "Safes", "Safts", "Sager", "Sahel", "Sahib",
    "Saint", "Saite", "Sakko", "Sakra", "Salam", "Salat", "Saldo", "Salem", "Sally", "Salon",
    "Salsa", "Salto", "Salut", "Salve", "Samba", "Samen", "Sammy", "Samoa", "Samos", "Samts",
    "Sanaa", "Sande", "Sands", "Sandy", "Santa", "Santo", "Sanyo", "Sarah", "Saras", "Sarde",
    "Sargs", "Sarin", "Saris", "Sasse", "Satan", "Satin", "Satyr", "Satze", "Sauce", "Sauls",
    "Saums", "Sauna", "Savoy", "Scala", "Scans", "Schaf", "Schah", "Scham", "Schar", "Schia",
    "Schis", "Schmu", "Schot", "Schub", "Schuh", "Schur", "Score", "Scott", "Scout", "SEATO",
    "Sedan", "Seele", "Segel", "Segen", "Segge", "Seher", "Seide", "Seiko", "Seils", "Seite",
    "Sekte", "Sekts", "Selen", "Selim", "Selma", "Semem", "Semit", "Senat", "Senfs", "Senne",
    "Sense", "Senta", "Seoul", "Sepia", "Sepps", "Serbe", "Seren", "Serge", "Serie", "Serin",
    "Serra", "Serum", "Sesam", "Setup", "Sexes", "Sexta", "Sexte", "Sexus", "Shake", "Sharp",
    "Shaws", "Shell", "Shift", "Shirt", "Shiva", "Shoah", "Shops", "Shows", "Siams", "Sibiu",
    "Sicht", "Sicke", "Sidon", "Siebs", "Siegs", "Siele", "Siels", "Siena", "Sigel", "Siggi",
    "Sigis", "Sigle", "Sikhs", "Silbe", "Silke", "Silos", "Silur", "Simon", "Simse", "Sinai",
    "Sinns", "Sinti", "Sinus", "Sioux", "Sippe", "SIPRI", "Sirup", "Sisal", "Sissi", "Sissy",
    "Sitar", "Sites", "Sitte", "Skala", "Skalp", "Skats", "Skier", "Skill", "Skins", "Skoda",
    "Skunk", "Skype", "Slang", "Slawe", "Slick", "Slips", "Slots", "Slums", "Smith", "Smogs",
    "Snack", "Snobs", "Soaps", "Socke", "Sodas", "Soden", "Sodom", "Soest", "Sofas", "Sofia",
    "Sofie", "Soges", "Sohle", "Sohne", "Sohns", "Sojus", "Solde", "Solds", "Solei", "Solen",
    "Solid", "Solon", "Solos", "Somme", "Sonar", "Sonde", "Songs", "Sonia", "Sonja", "Sonys",
    "Sopor", "Sorbe", "Sorte", "Souks", "Sound", "South", "Sozis", "Space", "Spalt", "Spans",
    "Spant", "SPARC", "Spatz", "Speck", "Speed", "Speer", "Spelz", "Spezi", "Spice", "Spike",
    "Spill", "Spina", "Spind", "Spins", "Spion", "Split", "Spock", "Spore", "Sporn", "Sport",
    "Spots", "Spott", "Spray", "Spree", "Spreu", "Sprit", "Spuks", "Spund", "Spurt", "SpVgg",
    "Squaw", "Staat", "Stabs", "Stack", "Stade", "Stadt", "Stage", "Stall", "Stamm", "Stare",
    "Stars", "Start", "Stasi", "Staub", "Staus", "Steak", "Steam", "Stege", "Stegs", "Stein",
    "Stele", "Steno", "Stenz", "Stern", "Sterz", "Stetl", "Steve", "Stews", "Steyr", "Stick",
    "Stiel", "Stier", "Stift", "Stile", "Stils", "Stino", "Stipp", "Stirn", "Stock", "Stoff",
    "Stola", "Stone", "Store", "Storm", "Story", "Streb", "Strip", "Stroh", "Strom", "Stube",
    "Stuck", "Studi", "Stuhl", "Stuka", "Stunk", "Stunt", "Stupa", "Stups", "Sturm", "Sturz",
    "Stuss", "Stute", "StVZO", "Suada", "Suade", "Sudan", "Sufis", "Suite", "Sujet", "Sulky",
    "Sulla", "Sumer", "Sumpf", "Suomi", "Suppe", "Supra", "Suren", "Susan", "Sushi", "Susie",
    "Susis", "Suzys", "Svens", "Sveta", "SWAPO", "Swift", "Swing", "Swiss", "Syene", "Syker",
    "Sykes", "Sylts", "Syrer", "Syrte", "Szene", "Tabak", "Tabea", "Tabor", "Tabus", "Tacho",
    "Tacos", "Tadel", "Tafel", "Tafts", "Tages", "Taiga", "Takel", "Takts", "Talar", "Taler",
    "Tales", "Talgs", "Talks", "Talon", "Talus", "Tamil", "Tands", "Tanga", "Tango", "Tangs",
    "Tanja", "Tanks", "Tanne", "Tante", "Tapas", "Tapes", "Tapet", "Tapir", "Tarif", "Tarot",
    "Tarps", "Tartu", "Taser", "Tasks", "Tasse", "Tasso", "Tatar", "Tatra", "Tatze", "Taues",
    "Taxen", "Taxis", "Taxon", "Taxus", "TByte", "Teams", "Teddy", "Teeei", "Teens", "Teeny",
    "Teers", "Tegel", "Teich", "Teige", "Teigs", "Teins", "Teint", "Telex", "Telko", "Tells",
    "Tempi", "Tempo", "Tenne", "Tenno", "Tenor", "Terme", "Terms", "Terra", "Terry", "Tesla",
    "Tessa", "Tests", "Tetum", "Teufe", "Texas", "Texel", "Texts", "Thais", "Thein", "Theke",
    "Thema", "Theos", "These", "Thiel", "Thilo", "Thing", "Thora", "Thorn", "Thors", "Thron",
    "Thuje", "Thule", "Thurn", "Tiara", "Tiber", "Tibet", "Ticks", "Tiden", "Tieck", "Tiefs",
    "Tiere", "Tiers", "Tietz", "Tiger", "Tilde", "Tilia", "Tills", "Tilly", "Timer", "Times",
    "Timex", "Timms", "Timmy", "Timor", "Timur", "Tinas", "Tinte", "Tipis", "Tipps", "Tiran",
    "Tirol", "Tisch", "Titan", "Titel", "Titer", "Titos", "Titte", "Titus", "Toast", "Tobak",
    "Tobis", "Toddy", "Toden", "Todes", "Tofus", "Togen", "Togos", "Tokio", "Tokyo", "Tommi",
    "Tommy", "Tomsk", "Tonen", "Toner", "Tones", "Tonga", "Tonic", "Tonis", "Tonne", "Tonus",
    "Tools", "Topas", "Topfs", "Topik", "Topoi", "Topos", "Toren", "Tores", "Torfe", "Torfs",
    "Torso", "Torte", "Torus", "Tosca", "Totos", "Touch", "Touri", "Tours", "Tower", "Toxin",
    "Trabi", "Track", "Tracy", "Trade", "Trafo", "Trail", "Trakl", "Trakt", "Trams", "Trane",
    "Trans", "Trapp", "Trara", "Trash", "Traum", "Traun", "Trebe", "Treck", "Treff", "Trema",
    "Trend", "Trias", "Trick", "Trier", "Trift", "Trike", "Trios", "Trips", "Troas", "Troer",
    "Troja", "Troll", "Tropf", "Tross", "Trost", "Trott", "Truck", "Trude", "Trudi", "Truhe",
    "Trumm", "Trump", "Trunk", "Trupp", "Trust", "Tuben", "Tubus", "Tuche", "Tuchs", "Tudor",
    "Tuffe", "Tuffs", "Tukan", "Tulpe", "Tumor", "Tuner", "Tunis", "Tunte", "Tupel", "Turbo",
    "Turin", "Turku", "Turme", "Turms", "Tusch", "Tusse", "Tussi", "Tutor", "Tutsi", "Tutus",
    "Twain", "Tweed", "Tweet", "Twens", "Twist", "Tyler", "Tylom", "Typen", "Typik", "Typus",
    "Tyros", "Tyrus", "Tyson", "TzBfG", "Ubier", "Uchta", "Udine", "UdSSR", "Uedem", "Ufers",
    "UFRGS", "Uhren", "Ulkus", "Ullas", "Ulmen", "Ulmer", "Umbau", "Umber", "Umbra", "Umweg",
    "Umzug", "Unart", "UNDCP", "Unfug", "Ungar", "UNHCR", "Union", "UNITA", "Units", "Unkel",
    "Unmut", "Unnas", "Unrat", "Unruh", "Untat", "Unzen", "Urahn", "Urals", "Urans", "UrhWG",
    "Uriel", "Urins", "Urnen", "Uroma", "Uropa", "Ursel", "Ursus", "Urtyp", "USAAF", "Usanz",
    "Uschi", "Usern", "Users", "Usutu", "Utahs", "Utans", "Uteri", "Uvula", "Vaduz", "Valin",
    "Vamps", "Varia", "Varix", "VARTA", "Varus", "Vasco", "Vasen", "Vater", "Vatis", "Vegas",
    "Veith", "Veits", "Velin", "Venen", "Venia", "Venlo", "Venns", "Venus", "Veras", "Verbs",
    "Verdi", "Verse", "Verve", "Vespa", "Vesta", "Vesuv", "Vetos", "Vichy", "Vicki", "Vicky",
    "Video", "Viech", "Viehs", "Viere", "Vigil", "Vikar", "Villa", "Ville", "Vince", "Vinci",
    "Vinyl", "Viola", "Viole", "Viper", "Viren", "Virgo", "Virus", "Visio", "Visit", "Visum",
    "Visus", "Viten", "Vitus", "Vlies", "Vogel", "Vogts", "Voigt", "Voile", "Volke", "Volks",
    "Volos", "Volta", "Volte", "Volvo", "Vopos", "Voten", "Votum", "Vulva", "VwVfG", "Waadt",
    "Waage", "Waben", "Wachs", "Waden", "Wadis", "Waffe", "Wagon", "Wahns", "Waise", "Walch",
    "Walde", "Waldi", "Waldo", "Walen", "Wales", "Walls", "Wally", "Wampe", "Wanda", "Wange",
    "Wanne", "Wanst", "Wanze", "Waran", "Warna", "Warze", "Watte", "Watts", "Wayne", "Weber",
    "WebEx", "Wedel", "Weeze", "Weges", "Wehrs", "Weibe", "Weida", "Weill", "Weins", "Welfe",
    "Welpe", "Welse", "Wendy", "Werks", "Werra", "Werst", "Werts", "Wesel", "Wesen", "Weser",
    "Wesir", "Wespe", "Wessi", "Weste", "Weyer", "Whigs", "White", "Wibke", "Wicke", "Wieck",
    "Wiehl", "Wiens", "Wiesn", "Wikis", "Wilds", "Wille", "Willi", "Willy", "Wilma", "Wilms",
    "Wilna", "Winds", "Wings", "Winks", "Wirte", "Wirts", "Witwe", "Witze", "Woche", "Wodka",
    "Wohle", "Wohls", "Wolff", "Wolfs", "Wolga", "Wolke", "Wonne", "Woods", "Woody", "World",
    "Worms", "Worte", "Worts", "Wotan", "Wrack", "Wraps", "Wruke", "Wucht", "Wuhan", "Wulff",
    "Wulst", "Wumme", "Wurfs", "Wurms", "Wurst", "Wusts", "Xaver", "Xenia", "Xenie", "Xenix",
    "Xenon", "Xerox", "Xetra", "Xhosa", "XHTML", "XLIII", "XLVII", "XVIII", "XXIII", "XXVII",
    "XXXII", "XXXIV", "XXXIX", "XXXVI", "Xylit", "Xylol", "Yacht", "Yahoo", "Yards", "Yetis",
    "Ylang", "Yogas", "Yogis", "Yorck", "Yorks", "Young", "Ypern", "Yucca", "Yukon", "Yusuf",
    "Zacke", "Zadek", "Zahns", "Zaire", "Zange", "Zappa", "Zaras", "Zaren", "Zarge", "Zarin",
    "Zaume", "Zaums", "Zauns", "Zebra", "Zebus", "Zecke", "Zeder", "Zehen", "Zeile", "Zeiss",
    "Zeitz", "Zelle", "Zelot", "Ziege", "Zitat", "Zitze", "Zivis", "Zonen", "Zubau", "Zucht",
    "Zunft", "Zunge", "Zuruf", "Zweck", "Zweig", "Zwerg", "Zwirn", "Zwist", "Zyste",
];
