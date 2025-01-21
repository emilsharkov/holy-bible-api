resource "aws_security_group" "all_traffic_sg" {
  name        = "allow-all-traffic-sg"
  description = "Security group that allows all inbound and outbound traffic"
  vpc_id      = aws_vpc.example.id

  ingress {
    description      = "Allow all inbound"
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

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
