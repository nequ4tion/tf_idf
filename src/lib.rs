mod tf;
mod idf;

/// Calculate the tf*idf vector for a given term
pub fn tf_idf_vector(term: &str, documents: &[String]) -> Vec<f32> {
    let mut tf_idf: Vec<f32> = Vec::new();
    // all words need to be lowercase so that term gets recognized, including term
    let term = term.to_lowercase();
    let lowercase_documents_vector = {
        let mut lowercase_vector: Vec<String> = Vec::new();
        for document in documents {
            lowercase_vector.push(document.to_lowercase());
        }
        lowercase_vector
    };

    // calculate the inverse document frequency
    let inverse_document_frequency = idf::idf(&term, &lowercase_documents_vector);
    
    // a vector in which the values for the term frequency will be stored in order
    let mut tf_vector: Vec<f32> = Vec::new();
    for document in lowercase_documents_vector.iter() {
        tf_vector.push(tf::tf(&term, document))
    }

    for tf_value in tf_vector {
        tf_idf.push(tf_value * inverse_document_frequency);
    }
    tf_idf
}

pub fn cosine_similarity(tf_idf_vector1: &Vec<f32>, tf_idf_vector2: &Vec<f32>) -> f32 {
    // first calculate the dot product of those two vectors
    let mut dot_product_result = 0.0f32;
    for (i, j) in tf_idf_vector1.iter().zip(tf_idf_vector2) {
        dot_product_result += i * j;
    }
    // now calculate the norm (length) of the vectors
    let mut tf_idf_vector1_norm: f32;
    let mut tf_idf_vector2_norm: f32;
    // first calculate the result of all numbers in the vector squared and added
    let mut vector1_result_of_squaring_and_adding = 0.0f32;
    let mut vector2_result_of_squaring_and_adding = 0.0f32;
    for (vector1n, vector2n) in tf_idf_vector1.iter().zip(tf_idf_vector2) {
        vector1_result_of_squaring_and_adding += vector1n.powi(2);
        vector2_result_of_squaring_and_adding += vector2n.powi(2);
    }
    // now get the square root of the result
    tf_idf_vector1_norm = vector1_result_of_squaring_and_adding.sqrt();
    tf_idf_vector2_norm = vector2_result_of_squaring_and_adding.sqrt();

    // now calculate the dot product / the product of the norms
    dot_product_result / (tf_idf_vector1_norm * tf_idf_vector2_norm)
}

pub fn cosine_similarity_from_terms(term1: &str, term2: &str, documents: &[String]) -> f32 {
    let vector1 = tf_idf_vector(term1, documents);
    let vector2 = tf_idf_vector(term2, documents);
    cosine_similarity(&vector1, &vector2)
}