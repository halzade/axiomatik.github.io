use std::fs;

fn replace_weather_in_content(content: &str, weather_string: &str) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";

    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(weather_string);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

#[test]
fn test_weather_replacement_in_file() {
    let test_file = "test_index_weather.html";
    let initial_content = "<html><!-- WEATHER -->OLD<!-- /WEATHER --></html>";
    fs::write(test_file, initial_content).unwrap();

    let weather_string = "23°C | Praha";
    let content = fs::read_to_string(test_file).unwrap();
    let new_content = replace_weather_in_content(&content, weather_string);

    if content != new_content {
        fs::write(test_file, new_content).unwrap();
    }

    let updated_content = fs::read_to_string(test_file).unwrap();
    assert_eq!(
        updated_content,
        "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>"
    );

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_no_weather_replacement_if_same_in_file() {
    let test_file = "test_index_weather_no_change.html";
    let initial_content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
    fs::write(test_file, initial_content).unwrap();

    let weather_string = "23°C | Praha";
    let content = fs::read_to_string(test_file).unwrap();
    let new_content = replace_weather_in_content(&content, weather_string);

    let mut updated = false;
    if content != new_content {
        fs::write(test_file, new_content).unwrap();
        updated = true;
    }

    assert!(!updated);
    let updated_content = fs::read_to_string(test_file).unwrap();
    assert_eq!(updated_content, initial_content);

    fs::remove_file(test_file).unwrap();
}
