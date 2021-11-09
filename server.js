const express = require('express');
const app = express();
const { exec } = require('child_process');
const fs = require('fs');

// parse JSON body
app.use(express.json());

const [shellCommand] = process.argv.slice(2);

app.get('/', (req, res) => {
  res.json({
    status: 'ok',
    time: Date.now(),
  });
});

app.post('/', (req, res) => {
  const body = req.body;
  const now = Date.now();

  if (!shellCommand) {
    return res.status(400).json({
      success: false,
      error: 'shell command is missing',
    });
  }

  fs.writeFileSync(`/tmp/${now}.json`, JSON.stringify(body));

  const command = `cat /tmp/${now}.json | ${shellCommand}`;
  console.log('command', command);

  exec(command, (error, stdout, stderr) => {
    if (error) {
      console.log(`error: ${error.message}`);
      return res.status(400).json({
        success: false,
        error: error.message,
      });
    }

    if (stderr) {
      console.log(`stderr: ${stderr}`);
      return res.status(400).json({
        success: false,
        stderr,
      });
    }

    console.log(`stdout: ${stdout}`);
    res.json({
      success: true,
      stdout,
    });
  });
});

const port = process.env.PORT || 8080;

app.listen(port, () => {
  console.log(`runner: listening on port ${port}`);
});
