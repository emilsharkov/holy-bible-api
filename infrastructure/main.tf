terraform {
  required_version = ">= 1.0.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = "us-east-1"  # Change to your preferred region
}

# ---------------------------
# VPC & Networking Resources
# ---------------------------

resource "aws_vpc" "example" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_support   = true
  enable_dns_hostnames = true

  tags = {
    Name = "example-vpc"
  }
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.example.id

  tags = {
    Name = "example-igw"
  }
}

# First public subnet in us-east-1a
resource "aws_subnet" "public_subnet" {
  vpc_id                  = aws_vpc.example.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true
  availability_zone       = "us-east-1a"

  tags = {
    Name = "example-public-subnet"
  }
}

# Second public subnet in us-east-1b (added)
resource "aws_subnet" "public_subnet_2" {
  vpc_id                  = aws_vpc.example.id
  cidr_block              = "10.0.2.0/24"
  map_public_ip_on_launch = true
  availability_zone       = "us-east-1b"

  tags = {
    Name = "example-public-subnet-2"
  }
}

resource "aws_route_table" "public_rt" {
  vpc_id = aws_vpc.example.id

  tags = {
    Name = "example-public-rt"
  }
}

resource "aws_route" "public_route" {
  route_table_id         = aws_route_table.public_rt.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.igw.id
}

resource "aws_route_table_association" "public_rta" {
  route_table_id = aws_route_table.public_rt.id
  subnet_id      = aws_subnet.public_subnet.id
}

# ---------------------------
# Security Group
# ---------------------------

resource "aws_security_group" "all_traffic_sg" {
  name        = "allow-all-traffic-sg"
  description = "Security group that allows all inbound and outbound traffic"
  vpc_id      = aws_vpc.example.id

  # Allow all inbound
  ingress {
    description      = "Allow all inbound"
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  # Allow all outbound
  egress {
    description      = "Allow all outbound"
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  tags = {
    Name = "allow-all-traffic-sg"
  }
}

# ---------------------------
# RDS Subnet Group
# ---------------------------
# Updated to include two subnets in different AZs
resource "aws_db_subnet_group" "example" {
  name       = "example-db-subnet-group"
  # Include both subnets for AZ coverage
  subnet_ids = [
    aws_subnet.public_subnet.id,
    aws_subnet.public_subnet_2.id
  ]

  tags = {
    Name = "example-db-subnet-group"
  }
}

# ---------------------------
# RDS Instance
# ---------------------------

resource "aws_db_instance" "bible-db" {
  allocated_storage      = 20
  engine                 = "postgres"
  engine_version         = "15.4"
  instance_class         = "db.t4g.micro"
  username               = "postgres"
  password               = "bibledbpassword"
  parameter_group_name   = "default.postgres15"
  publicly_accessible    = true
  vpc_security_group_ids = [aws_security_group.all_traffic_sg.id]
  db_subnet_group_name   = aws_db_subnet_group.example.name
  skip_final_snapshot    = true

  tags = {
    Name = "bible-db"
  }
}