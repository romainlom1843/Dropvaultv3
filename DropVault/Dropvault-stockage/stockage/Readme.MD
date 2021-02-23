 
 
 Upload
 curl -v -H "Content-Type: application/json"  -X POST -d '{"filename": "file.txt", "content": "bar1", "username": "michel"}' 127.0.0.1:8084/files
 
 get files
 curl -X GET http://localhost:8084/files/michel

curl -v -X GET -H 'Authorization: Bearer '   -i 'http://127.0.0.1:8084/files/michel'

User
curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' 127.0.0.1:8080/signup

curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel",  "passwd": "mich2310"}' 127.0.0.1:8080/login
