use cookie::CookieJar;

pub fn cookie_jar_to_string(cookie_jar: &CookieJar) -> String {
    cookie_jar.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}
