/questions POST
```
curl -X POST 'localhost:7878/questions' -H 'Content-Type: application/json' -d '{"title": "Urgent question", "content": "Customer care phone is always busy"}'
```

/questions GET
```
curl --location 'localhost:7878/questions'
```

/question/:uid GET
```
curl --location 'localhost:7878/questions/b321bcc9-5a78-4daf-ac6a-bad182278f07'
```

/questions/:uid PUT
```
curl -X PUT 'localhost:7878/questions/90473158-5b63-11ed-9b6a-0242ac120002' -H 'Content-Type: application/json' -d '{"title": "Urgent question (CLOSED)", "content": "Customer care phone is always busy."}' 
```

/question/:uid DELETE
```
curl -X DELETE 'localhost:7878/questions/b321bcc9-5a78-4daf-ac6a-bad182278f07'
```

