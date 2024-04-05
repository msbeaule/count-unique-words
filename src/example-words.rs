// common groupings of words, and their misspellings
pub fn get_synonyms() -> Vec<Vec<&'static str>> {
    let synonyms:Vec<Vec<&str>> = vec![
        vec!["house", "housing", "home", "rental", "rent", "condo", "apartment"],
        vec!["marina", "water", "waterfront", "boardwalk", "launch", "boat", "beach", "beaches", "sea", "seaside", "ocean"],
        vec!["walk", "walking", "walks", "walkable", "sidewalk", "path", "paths", "trail", "trails", "pedestrian", "pedestrians"],
        vec!["bike", "bikes", "biking", "cycle", "cycling", "cycles"],
        vec!["green", "air", "natural", "nature", "river", "forest", "mountain", "mountains"],
        vec!["park", "parks"],
        vec!["parking", "traffic"],
        vec!["transit", "bus", "busses"],
        vec!["retail", "store", "stores", "shop", "shops", "shopping", "grocery"],
        vec!["restaurant", "restaurants", "cafe", "cafes", "coffee"],
        vec!["senior", "seniors", "elder", "elderly", "retire", "retirement"],
        vec!["medical", "med", "nurse", "dentist", "doctor", "dr", "health", "healthcare"],
        vec!["festival", "festivals", "event", "events"],
        vec!["tourist", "tourists", "tour", "tourism", "visitor", "visitors"],
        vec!["hotel", "hotels", "motel", "motels", "accommodation", "accommodations"],
    ];
    return synonyms;
}
