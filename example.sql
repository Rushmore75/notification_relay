
SELECT message.id
FROM read
INNER JOIN message ON read.message = message.id AND read.account = 1;


--@block
SELECT message.id
FROM message 
INNER JOIN read ON message.id != read.message;