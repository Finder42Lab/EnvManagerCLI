use crate::types::variable::IVariable;
use std::{env, fs};
use owo_colors::OwoColorize;
use crate::helpers::variables::construct_env_file;

async fn get_project_variables(project_id: &str) -> Vec<IVariable> {
    let client = reqwest::Client::new();

    let host = env::var("EMN_HOST").unwrap_or_else(|_| "http://localhost:8420".to_string());

    client
        .get(&format!("{}/api/v1/variables/?project_id={}", host, project_id))
        .send()
        .await
        .unwrap()
        .json::<Vec<IVariable>>()
        .await
        .unwrap()
}

pub async fn switch_env(project_id: &str) {
    // Здесь будет логика переключения между проектами
    println!("{}", "Загружаю переменные".yellow());
    let variables = get_project_variables(project_id).await;

    println!("{}", "Генерирую содержимое .env-файла".yellow());

    let env_content = construct_env_file(variables);

    if let Ok(file) = fs::exists(".env") {
        if file {
            println!("{}", "Делаю копию сущестыующего .env-файла".yellow());

            fs::rename(".env", ".env.backup").unwrap();
        }
    }

    println!("{}", "Создаю новый .env-файл".yellow());

    match fs::write(".env", env_content) {
        Ok(_) =>     println!("{}", "Файл .env успешно создан".green()),
        Err(e) =>     println!("{} {}", "Ошибка при создании файла .env:".red(), e),
    };

}
