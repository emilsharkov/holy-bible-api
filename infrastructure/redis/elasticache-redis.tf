resource "aws_elasticache_replication_group" "my_redis_cluster" {
  engine                        = "redis"
  engine_version                = "7.0"
  transit_encryption_enabled    = true
  auth_token                    = var.auth_token
  automatic_failover_enabled    = false
  replication_group_id          = "my-redis-cluster"
  node_type                     = "cache.t4g.micro"
  parameter_group_name          = "default.redis7"
  port                          = 6379
  subnet_group_name             = "${aws_elasticache_subnet_group.redis_subnet_group.name}"
  security_group_ids            = ["${aws_security_group.all_traffic_sg.id}"]
  description                   = "free tier redis elasticache cluster"
}