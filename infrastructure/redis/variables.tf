variable "aws_region" {
  type        = string
  description = "AWS region where resources will be created."
  default     = "us-east-1"
}

variable "auth_token" {
  type        = string
  description = "Auth token for Redis cluster."
}