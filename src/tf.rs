/* Copyright (c) 2019 Nequ4tion */
/* See LICENSE for licensing information */

/// Term frequency: number of times t appeared in a document / number of terms in the document
pub fn tf(term: &str, document: &str) -> f32 {
    let mut number_of_occurrences = 0;
    let mut number_of_terms = 0;
    for i in document.split_whitespace() {
        if i == term {
            number_of_occurrences += 1;
        }
        number_of_terms += 1;
    }
    number_of_occurrences as f32 / number_of_terms as f32
}
