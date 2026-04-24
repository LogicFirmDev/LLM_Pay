const express = require('express');
const paymentRoutes = require('./routes/payments');

const app = express();
app.use(express.json());
app.use('/api/payments', paymentRoutes);

app.get('/', (req, res) => {
  res.send('AI-Powered Payment Agent Backend');
});

const PORT = process.env.PORT || 4000;
app.listen(PORT, () => {
  console.log(`Backend running on port ${PORT}`);
});
