const express = require('express');
const router = express.Router();
const path = require('path');

const staticRoutes = [
    "/",
    "/home",
    "/hello-world",
    "/dashboard",
    "/login",
    "/signup",
    "/logout",
    "/page-not-found",
];

router.use(staticRoutes, express.static(path.join(__dirname, 'dist')));

module.exports = router;
