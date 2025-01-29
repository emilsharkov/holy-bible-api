output "bucket_name" {
  value = aws_s3_bucket.existing_bucket.bucket
}

output "region" {
  value = aws_s3_bucket.existing_bucket.region
}