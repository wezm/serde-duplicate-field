extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    title: String,
    photo_count: u32,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exif {
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
    iso: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    _self: String,
    html: String,
    photos: Option<String>,
    likes: Option<String>,
    portfolio: Option<String>,
    download: Option<String>,
    download_location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileImage {
    small: String,
    medium: String,
    large: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    updated_at: Option<String>,
    twitter_username: Option<String>,
    portfolio_url: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    total_likes: u32,
    total_photos: u32,
    total_collections: u32,
    profile_image: ProfileImage,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Position {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    city: String,
    country: String,
    position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct Collection {
    id: u32,
    title: String,
    published_at: String,
    updated_at: String,
    curated: bool,
    cover_photo: CoverPhoto,
    user: User,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoverPhoto {
    id: String,
    width: u16,
    height: u16,
    color: String,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    user: User,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnsplashFoto {
    id: String,
    created_at: String,
    updated_at: String,
    width: u16,
    height: u16,
    color: String,
    downloads: u16,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    exif: Exif,
    location: Option<Location>,
    current_user_collections: Vec<Collection>,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
    user: User,
    slug: Option<String>,
}

fn main() {
    let body = include_str!("foto.json");
    let foto: UnsplashFoto = match serde_json::from_str(body) {
        Ok(foto) => foto,
        Err(err) => panic!("JSON parse error {:?} body:\n{}", err, body),
    };

    println!("{:?}", foto);
}
