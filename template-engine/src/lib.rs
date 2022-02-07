use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognised,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionData {
    pub expression: String,
    pub var_map: Vec<String>,
    pub generated_html: String,
}

// HTML-related function
pub fn generate_html_template_var(
    content: &mut ExpressionData,
    context: HashMap<String, String>
) -> String {

    content.generated_html = content.expression.clone();

    println!("content_var: {:?}", content.var_map);

    for term in &content.var_map {
        if check_symbol_string(&term, "{{") &&
            check_symbol_string(&term, "}}") {
                let (_, i) = get_index_for_symbol(&term, "{{");
                let (_, j) = get_index_for_symbol(&term, "}}");

                let variable = term[i+"{{".len()..j].to_string();
                let value = context.get(&variable).unwrap();

                content.generated_html = content.generated_html.replace(term, value);
            }
    }

    content.generated_html.clone()
}


// Parser-related functions
pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str, symbol1: &str, symbol2: &str) -> bool {
    let index_1 = input.find(symbol1);
    let index_2 = input.find(symbol2);

    let precedency = index_1 < index_2;

    input.contains(symbol1) && input.contains(symbol2) && precedency
}

pub fn get_index_for_symbol(input: &str, symbol: &str) -> (bool, usize) {
    // char_indices returns an iterator over the char's of a string
    let exists;
    let mut index = 0;

    if let Some(pos) = input.find(symbol) {
        exists = true;
        index = pos;
    } else {
        exists = false;
    }

    (exists, index)
}

pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let expression_terms = input_line.split_whitespace();
    let mut template_var_map = vec![];

    for term in expression_terms {
        if check_symbol_string(term, "{{") &&
            check_symbol_string(term, "}}") {
            template_var_map.push(term.to_string());
        } 
    }

    ExpressionData {
        expression: input_line.to_string(),
        var_map: template_var_map,
        generated_html: "".to_string()
    }
}

pub fn get_content_type(input_line: &str) -> ContentType {
    // {% if %}
    // {% for %}
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");

    // {% for txn in transactions %}
    let is_for_tag = 
        (check_symbol_string(&input_line, "for") &&
        check_symbol_string(&input_line, "in")) ||
        check_symbol_string(&input_line, "endfor");
   
    // {% if user_logged_in %}
    let is_if_tag = 
        check_symbol_string(&input_line, "if") ||
        check_symbol_string(&input_line, "endif");

    // Date: {{txn.date}}
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");

    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognised;
    }

    return return_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1> Hello World </h1>";

        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test() {
        let content = ExpressionData {
            expression: "Hi {{name}} ,welcome".to_string(),
            var_map: vec!["{{name}}".to_string()],
            generated_html: "".to_string()
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}} ,welcome") );
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag), 
            get_content_type("{% for name in names %}, welcome"));
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag), 
            get_content_type("{% if name == 'Bob' %}")); 
    }

}