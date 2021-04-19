# SIGNIN AND SIGNUP

User curl -v -H "Content-Type: application/json" -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' localhost:31003/signup

curl -v -H "Content-Type: application/json" -X POST -d '{"username": "michel", "passwd": "mich2310"}' localhost:31003/login
