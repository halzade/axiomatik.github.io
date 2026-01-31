pub async fn extract_video_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    let bytes = field.bytes().await.ok()?.to_vec();
    validate_and_extract(&file_name, bytes, ALLOWED_EXTENSIONS_VIDEO)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_extract_video_data() {
        let allowed = ALLOWED_EXTENSIONS_VIDEO;
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.mp4", data.clone(), allowed), Some((data.clone(), "mp4".to_string())));
        assert_eq!(validate_and_extract("t.jpg", data.clone(), allowed), None);
    }
}