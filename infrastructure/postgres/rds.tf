resource "aws_db_subnet_group" "example" {
  name       = "example-db-subnet-group"
  subnet_ids = [
    aws_subnet.public_subnet.id,
    aws_subnet.public_subnet_2.id
  ]

  tags = {
    Name = "example-db-subnet-group"
  }
}

resource "aws_db_instance" "bible-db" {
  allocated_storage      = 20
  engine                 = "postgres"
  engine_version         = "15.7"
  instance_class         = "db.t4g.micro"
  username               = var.db_username
  password               = var.db_password
  parameter_group_name   = "default.postgres15"
  publicly_accessible    = true
  vpc_security_group_ids = [aws_security_group.all_traffic_sg.id]
  db_subnet_group_name   = aws_db_subnet_group.example.name
  skip_final_snapshot    = true
  apply_immediately      = true

  tags = {
    Name = "bible-db"
  }
}