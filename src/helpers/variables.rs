use crate::types::variable::IVariable;
use std::collections::HashMap;

fn construct_by_groups(variables: Vec<&IVariable>) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let mut grouped = HashMap::<String, Vec<&IVariable>>::new();

    for var in variables {
        grouped.entry(var.project.id.to_string()).or_insert_with(Vec::new).push(var);
    }

    for var in grouped.values() {
        lines.push(format!("\n# From {}", var[0].project.name));

        let var_lines = var
            .iter()
            .map(|v| format!("{}={}\n", v.name, v.value.clone().unwrap_or("".to_string())))
            .collect::<Vec<String>>();

        lines.push("\n".to_string());

        lines.extend(var_lines);
    }

    lines
}

pub fn construct_env_file(variables: Vec<IVariable>) -> String {
    let secret_variables =
        (&variables).into_iter().filter(|var| var.is_secret).collect::<Vec<&IVariable>>();

    let non_secret_variables =
        (&variables).into_iter().filter(|var| !var.is_secret).collect::<Vec<&IVariable>>();

    let mut lines: Vec<String> = Vec::new();

    lines.extend(construct_by_groups(non_secret_variables));

    lines.push("\n####################".to_string());
    lines.push("\n# Secret Variables #".to_string());
    lines.push("\n####################\n\n".to_string());

    lines.extend(construct_by_groups(secret_variables));

    lines.concat()
}
