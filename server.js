const express = require('express');
const app = express();
const { exec } = require('child_process');

// parse JSON body
app.use(express.json());

const [shellCommand] = process.argv.slice(2);

app.post('/', (req, res) => {
  const body = req.body;

  if (!shellCommand) {
    return res.status(400).json({
      success: false,
      error: 'shell command is missing',
    });
  }

  const command = `${shellCommand} ${JSON.stringify(body)}`;
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
