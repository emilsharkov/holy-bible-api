output "vpc_id" {
  description = "The ID of the newly created VPC."
  value       = aws_vpc.this.id
}

output "redis_security_group_id" {
  description = "The ID of the Redis security group."
  value       = aws_security_group.redis_sg.id
}

output "redis_endpoint" {
  description = "The endpoint address of the Redis cluster."
  value = aws_elasticache_cluster.redis.cache_nodes[0].address
}
