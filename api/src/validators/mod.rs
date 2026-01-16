use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid_not_nil(uuid: &Uuid) -> Result<(), ValidationError> {
    if uuid.is_nil() {
        return Err(ValidationError::new("uuid_cannot_be_nil"));
    }
    Ok(())
}

pub fn validate_uuid_vec(vec: &Vec<Uuid>) -> Result<(), ValidationError> {
    for uuid in vec {
        if uuid.is_nil() {
            return Err(ValidationError::new("tags_contain_nil_uuid"));
        }
    }
    Ok(())
}
