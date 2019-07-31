
#[inline]
fn is_term_in_document(term: &str, document: &str) -> bool {
    for i in document.split_whitespace() {
        if i == term {
            return true;
        }
    }
    false
}


/// Inverse document frequency: 1 + log(N/n) where N is the number of documents
/// and n is the the number of documents the term t has appeared in.
pub fn idf(term: &str, documents: &[String]) -> f32 {
    // let N be the number of documents
    let big_n = documents.len();
    // let n be the number of documents term has appeared in
    let mut n = 0;

    for i in documents {
        if is_term_in_document(term, &i) {
            n += 1;
        }
    }
    if big_n > 0 && n > 0 {
        1.0f32 + (big_n as f32 / n as f32).log(std::f32::consts::E)
    } else {
        1.0f32
    }
}

