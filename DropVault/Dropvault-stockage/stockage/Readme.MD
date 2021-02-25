 
 
 Upload
 curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Iko0eUFzMlgiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJyb21haW5sbCIsImNvbXBhbnkiOiJEcm9wVmF1bHQiLCJleHAiOjE2MTQyNDE3MDV9.X2DbCFxTD-37S5wmUAXHgCNsiSTYiRIKMX27Bd4durs' -X POST -d '{"filename": "file.txt", "content": "bar1", "username": "michel"}' 127.0.0.1:8084/files
 
 get files
 curl -X GET http://localhost:8084/files/michel

curl -v -X GET -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Ik12dzJybGEiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE0MTk2Nzk5fQ.MxIHwqxgQ_5UQ4dOOT0wgjpbGJCJ-saWVJfMT-cqVbU'   -i 'http://127.0.0.1:8084/files/michel'

User
curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' 127.0.0.1:8080/signup

curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel",  "passwd": "mich2310"}' 127.0.0.1:8080/login
