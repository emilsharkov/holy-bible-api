output "db_instance_endpoint" {
  value = aws_db_instance.bible-db.endpoint
}

output "vpc_id" {
  value = aws_vpc.example.id
}