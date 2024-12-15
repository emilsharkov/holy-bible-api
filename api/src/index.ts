import app from './app';
import connectDB from './config/db';
import { config } from './config/env';

const PORT = config.PORT;

connectDB();

app.listen(PORT, () => {
  console.log(`Server running on http://localhos t:${PORT}`);
});
