# Signin and signup

User curl -v -H "Content-Type: application/json" -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' 127.0.0.1:8080/signup

curl -v -H "Content-Type: application/json" -X POST -d '{"username": "michel", "passwd": "mich2310"}' 127.0.0.1:8080/login
