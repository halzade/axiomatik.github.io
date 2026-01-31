pub async fn extract_image_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    let bytes = field.bytes().await.ok()?.to_vec();
    validate_and_extract(&file_name, bytes, ALLOWED_EXTENSIONS_IMAGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_data() {
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.jpg", data.clone()), Some((data.clone(), "jpg".to_string())));
        assert_eq!(validate_and_extract("t.gif", data.clone()), None);
    }
}