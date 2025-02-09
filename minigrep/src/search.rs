pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
    let lower_cased_query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&lower_cased_query))
}
