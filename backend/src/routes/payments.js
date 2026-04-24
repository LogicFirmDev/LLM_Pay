const express = require('express');
const router = express.Router();
const paymentsController = require('../controllers/paymentsController');

router.post('/submit', paymentsController.submitPayment);
router.get('/status', paymentsController.paymentStatus);

module.exports = router;
