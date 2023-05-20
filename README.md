# shell-runner

An executable HTTP server that lets you run a shell command

## Usage

Example using `cat` as the command.

```bash
$ shell-runner cat
```

This will start a server on port 8080. You can specify a port with the env varible `PORT`.

## Endpoints

### `GET /`

This is simply a health check endpoint.
Returns a 200 response.

### `POST /`

This is the main endpoint. It can take any string as input and will pass it to the command as stdin.