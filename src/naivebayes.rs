use std::collections::{HashMap, HashSet};

// Naive bayes
// Implemented from https://www.youtube.com/watch?v=O2L2Uv9pdDA
fn main() {
    let good_messages = vec![
        "Dear Dear Dear Dear Dear Dear Dear Dear",
        "Friend Friend Friend Friend Friend",
        "Lunch Lunch Lunch",
        "Money",
        "",
        "",
        "",
        "",
    ];
    let spam_messages = vec![
        "Dear Dear",
        "Friend",
        "", // Lunch
        "Money Money Money Money",
    ];
    let training_data: Vec<&str> = vec![&good_messages[..], &spam_messages[..]].concat();

    let training_data_length = training_data.len();
    let good_prior_probability = good_messages.len() as f32 / training_data_length as f32;
    let spam_prior_probability = spam_messages.len() as f32 / training_data_length as f32;

    let alpha = 1;

    let (good_word_count, good_word_likelihoods) = word_likelihoods(good_messages.clone());
    let (spam_word_count, spam_word_likelihoods) = word_likelihoods(spam_messages.clone());
    println!("good_word_count: {}, good_word_likelihoods: {:?}", good_word_count, good_word_likelihoods);
    println!("spam_word_count: {}, spam_word_likelihoods: {:?}", spam_word_count, spam_word_likelihoods);

    let test_messages = vec![
        // "Dear Friend",
        "Lunch Money Money Money Money"
    ];
    for test_message in test_messages {
        let test_message_words = message_to_words(&test_message);
        let test_good_probability = good_prior_probability * word_likelihoods_product(alpha, &test_message_words, good_word_count, &good_word_likelihoods);
        let test_spam_probability = spam_prior_probability * word_likelihoods_product(alpha, &test_message_words, spam_word_count, &spam_word_likelihoods);
        println!(
            "result={}, good={}, spam={}, msg={}",
            if test_good_probability > test_spam_probability { "good" } else { "spam" },
            test_good_probability,
            test_spam_probability,
            test_message,
        );
    }
}

fn message_to_words(message: &str) -> Vec<&str> {
    message.split(" ").filter(|s| s.len() > 0).collect()
}

fn word_likelihoods(messages: Vec<&str>) -> (i32, HashMap<&str, f32>) {
    let mut word_counts: HashMap<&str, i32> = HashMap::new();
    let mut total_word_count = 0;
    for message in messages {
        for word in message_to_words(message) {
            total_word_count += 1;
            if !word_counts.contains_key(word) {
                word_counts.insert(word, 0);
            }
            word_counts.insert(word, word_counts.get(word).unwrap() + 1);
        }
    }

    let mut likelihoods: HashMap<&str, f32> = HashMap::new();
    for (word, count) in word_counts {
        likelihoods.insert(word, count as f32 / total_word_count as f32);
    }
    (total_word_count, likelihoods)
}

fn word_likelihoods_product(alpha: i32, words: &Vec<&str>, count: i32, likelihoods: &HashMap<&str, f32>) -> f32 {
    let distinct_words_count = likelihoods.len() as i32;
    let words_set: HashSet<&&str> = HashSet::from_iter(words.into_iter());
    let mut unknown_words_count = 0;
    for word in words_set {
        if !likelihoods.contains_key(word) {
            unknown_words_count += 1;
        }
    }
    let num_alpha = distinct_words_count + unknown_words_count;
    let denominator = count as f32 + (num_alpha as f32 * alpha as f32);

    let mut running_probability = 1.0;
    for word in words {
        // Zero conditional probability problem
        // running_probability *= likelihoods.get(word).unwrap_or(&0.0);
        let word_probability = likelihoods.get(word).unwrap_or(&0.0);
        let numerator = (word_probability * count as f32) + alpha as f32;
        let fixed_word_probability = numerator / denominator;
        // println!("p({} | .) = {}", word, fixed_word_probability);
        running_probability *= fixed_word_probability;
    }
    running_probability
}
