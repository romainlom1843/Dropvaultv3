 
 # PROCESSUS D UPLOAD 
 
 1) Déclare un objet dans la bdd de stockage :
 
 curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6InQyNGZJdWkiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE4NDkzMzM1fQ.a0imm1mYywVBoQfVk1JktNyXaufC7QhnvbVr6VehZd4' -X POST -d '{"filename": "toto.txt", "username": "michel", "sizing": "15", "ext": "image/"}' localhost:31000/files
 
2) Liste les fichiers

curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6InQyNGZJdWkiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE4NDkzMzM1fQ.a0imm1mYywVBoQfVk1JktNyXaufC7QhnvbVr6VehZd4' -X GET localhost:31000/files/michel

3) ajout le contenu du fichier créé

curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6InIwSTUyYXEiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE4NDkzODE4fQ.RXyW5iXpLxsReonr2owrq01UHYLqU3cn_zkHhvWYRNU' -X POST -d '{"content": "Blabla.", "id_c": 1 }' localhost:31000/upload

# vielle commande d'upload
 Upload
 curl -v -H "Content-Type: application/json" -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Im1vMjhPYXAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJyb21haW5sIiwiY29tcGFueSI6IkRyb3BWYXVsdCIsImV4cCI6MTYxNDc4Njg4OH0.jOKHlRAsvOeQkYh6LbSxT8Z1CahNJyRVWqQJs-hAj70' -X POST -d '{"filename": "file.txt", "username": "michel", "sizing": "15", "extension": "image/"}' 127.0.0.1:8084/files
 
 get files
 curl -X GET http://localhost:8084/files/michel

curl -v -X GET -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Ik12dzJybGEiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiJtaWNoZWwiLCJjb21wYW55IjoiRHJvcFZhdWx0IiwiZXhwIjoxNjE0MTk2Nzk5fQ.MxIHwqxgQ_5UQ4dOOT0wgjpbGJCJ-saWVJfMT-cqVbU'   -i 'http://127.0.0.1:8084/files/michel'

User
curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel", "email": "michel.drucker@gmail.fr", "passwd": "mich2310"}' 127.0.0.1:8080/signup

curl -v -H "Content-Type: application/json"  -X POST -d '{"username": "michel",  "passwd": "mich2310"}' 127.0.0.1:8080/login
