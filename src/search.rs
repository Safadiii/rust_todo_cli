use std::{clone, result};

use crate::{category::Category, task::Task};
use color_eyre::config;
use strsim::jaro_winkler;


pub enum MatchField {
    Tag(String),
    Title,
    Description
}
pub struct SearchResult<'a> {
    pub category_title: &'a str,
    pub task: &'a Task,
    pub score: f64,
    pub matched_on: Vec<MatchField>
}


fn score_field(field: &str, query: &str) -> f64 {
    let q = query.to_lowercase();
    let text = field.to_lowercase();

    let full_score = jaro_winkler(&text, &q);

    let word_score = text.split_whitespace().map(|word| jaro_winkler(word, &q)).fold(0.0_f64, f64::max);

    full_score.max(word_score)
}


/*

Config for Search Function -> Seemed like a nice touch 

*/

pub struct  SearchConfig {
    pub title_threshold: f64,
    pub tag_threshold: f64,
    pub description_threshold: f64,
    pub title_weight: f64,
    pub tag_weight: f64,
    pub description_weight: f64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            title_threshold: 0.85,
            tag_threshold: 0.8,
            description_threshold: 0.72,
            title_weight: 3.0,
            tag_weight: 2.0,
            description_weight: 1.0
        }
    }
}

pub fn search_fuzzy<'a>(categories: &'a [Category], query: &str, config: &SearchConfig) -> Vec<SearchResult<'a>> {
    let mut results: Vec<SearchResult<'a>> = Vec::new();

    for cat in categories {
        for task in &cat.taskslist.tasks {
            let mut weighted_score = 0.0_f64;
            let mut matched_on = Vec::new();

            let title_score = score_field(&task.title, query);
            if title_score >= config.title_threshold {
                weighted_score = weighted_score.max(title_score * config.title_weight);
                matched_on.push(MatchField::Title);
            }

            for tag in &task.tags {
                let tag_score = score_field(&tag, query);
                if tag_score >= config.tag_threshold {
                    weighted_score = weighted_score.max(tag_score * config.tag_weight);
                    matched_on.push(MatchField::Tag(tag.clone()));
                }
            }

            if !task.description.is_empty() {
                let desc_score = score_field(&task.description, query);
                if desc_score >= config.description_threshold {
                    weighted_score = weighted_score.max(desc_score * config.description_weight);
                    matched_on.push(MatchField::Description);
                }
            }

            if !matched_on.is_empty() {
                results.push(SearchResult { category_title: &cat.title, task, score: weighted_score, matched_on });
            }
        }
    }

    results.sort_by(|a,  b| b.score.partial_cmp(&a.score).unwrap());

    results
}