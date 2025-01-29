output "vpc_id" {
  description = "The ID of the newly created VPC."
  value       = aws_vpc.example.id
}

output "redis_security_group_id" {
  description = "The ID of the Redis security group."
  value       = aws_security_group.all_traffic_sg.id
}

output "redis_endpoint" {
  description = "The endpoint address of the Redis cluster."
  value       = aws_elasticache_replication_group.my_redis_cluster.primary_endpoint_address
}