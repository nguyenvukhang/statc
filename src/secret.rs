pub const SECRET: &str = "
Qb jungrire gur uryy lbh jnag.
Whfg erzrzore gb xrrc qbvat chyyhcf.
  - Xunat 2022
";

pub fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}
