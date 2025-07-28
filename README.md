# DODS
API de traduccion de ingles a español!
## ¿Cómo funciona?
En la url `http://dominio.com/api/<palabra>` donde cambias `<palabra>` por la palabra en ingles, y te devuelve la palabra traducida al español.
## POST
Para las request post, usa el cliente que quieras y tenes que incluir esto en el json:
```json
{
  "EN": "<palabra en ingles>",
  "ES": "<palabra en español>"
}
```
## Logica:
Para la logica de el programa, usé un archivo json con cada palabra y su traduccion en una estructura
