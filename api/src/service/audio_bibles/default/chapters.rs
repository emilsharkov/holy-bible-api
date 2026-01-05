use std::error::Error;

use crate::{db::s3::S3Client, models::sql::audio_bibles};
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use sqlx::PgPool;
use tracing;

pub async fn get_audio_bible_chapters(
    db_client: &PgPool,
    audio_bible_id: i32,
    book_num: i32,
) -> Result<i64, Box<dyn Error>> {
    let rows: Vec<audio_bibles::Count> = sqlx::query_as(
        "SELECT COUNT(distinct chapter) 
        FROM audio_bible_chapters 
        WHERE audio_bible_id = $1
        AND book = $2",
    )
    .bind(audio_bible_id)
    .bind(book_num)
    .fetch_all(db_client)
    .await?;

    let num_chapters = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
        .count;

    Ok(num_chapters)
}

pub async fn get_audio_chapter_file_key(
    db_client: &PgPool,
    audio_bible_id: i32,
    book_num: i32,
    chapter_num: i32,
) -> Result<String, Box<dyn Error>> {
    let rows: Vec<audio_bibles::AudioBible> = sqlx::query_as(
        "SELECT audio_bible_id, language, version FROM audio_bibles WHERE audio_bible_id = $1",
    )
    .bind(audio_bible_id)
    .fetch_all(db_client)
    .await?;

    let audio_bible_language = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
        .language;

    let file_key = format!(
        "languages/{}/{}/{}.mp3",
        audio_bible_language, book_num, chapter_num
    );

    Ok(file_key)
}

pub async fn get_audio_chapter_from_s3(
    db_client: &PgPool,
    s3_client: &S3Client,
    audio_bible_id: i32,
    book_num: i32,
    chapter_num: i32,
) -> Result<aws_sdk_s3::operation::get_object::GetObjectOutput, Box<dyn Error>> {
    let S3Client {
        client,
        audio_bibles_bucket,
    } = s3_client;

    let file_key =
        get_audio_chapter_file_key(db_client, audio_bible_id, book_num, chapter_num).await?;

    let object_output = client
        .get_object()
        .bucket(audio_bibles_bucket)
        .key(file_key)
        .send()
        .await
        .map_err(|err| {
            match &err {
                SdkError::ServiceError(service_err) => {
                    tracing::error!(
                        "S3 Service Error: {:?}, Message: {:?}",
                        service_err.err().code(),
                        service_err.err().message()
                    );
                }
                SdkError::TimeoutError(_) => {
                    tracing::error!("S3 request timed out.");
                }
                SdkError::DispatchFailure(dispatch_err) => {
                    tracing::error!("Failed to send request: {:?}", dispatch_err);
                }
                SdkError::ResponseError(response_err) => {
                    tracing::error!("Invalid response from S3: {:?}", response_err);
                }
                _ => {
                    tracing::error!("Unhandled S3 error: {:?}", err);
                }
            }
            Box::<dyn Error>::from("Resource does not exist")
        })?;

    Ok(object_output)
}

