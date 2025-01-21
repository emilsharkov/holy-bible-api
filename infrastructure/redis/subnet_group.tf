resource "aws_elasticache_subnet_group" "redis_subnet_group" {
  name        = "redis-subnet-group"
  description = "Subnet group for Redis"
  subnet_ids  = [
    aws_subnet.public_subnet.id,
    aws_subnet.public_subnet_2.id,
  ]

  tags = {
    Name = "redis-subnet-group"
  }
}
