const express = require('express');
const session = require('express-session');
const pg = require('pg');
const app = express();
const path = require('path');
const morgan = require('morgan');
const config = require('config');
const { createProxyMiddleware } = require('http-proxy-middleware');
const bodyParser = require('body-parser');
const { default: axios } = require('axios');
const { resolve } = require('path');
const { rejects } = require('assert');

const routes = require('./routes');

const secret = (config.has('app.cookie.secret')) ? config.get('app.cookie.secret') : process.env.APP_COOKIE_SECRET || require("crypto").randomBytes(16).toString("hex");
const port = (config.has('app.port')) ? config.get('app.port') : process.env.APP_PORT || 3000;
const maxAge = (config.has('app.max-age')) ? config.get('app.max-age') : process.env.APP_MAX_AGE || 60 * 60 * 24 * 7;

const apiBackendHost = (config.has('api.backend.host')) ? config.get('api.backend.host') : process.env.API_BACKEND_HOST;
const apiBackendPort = (config.has('api.backend.port')) ? config.get('api.backend.port') : process.env.API_BACKEND_PORT;
const apiBackendPath = (config.has('api.backend.path')) ? config.get('api.backend.path') : process.env.API_BACKEND_PATH;

const api = `http://${apiBackendHost}:${apiBackendPort}/${apiBackendPath}`;

const dbHost = (config.has('db.host')) ? config.get('db.host') : process.env.DB_HOST;
const dbPort = (config.has('db.port')) ? config.get('db.port') : process.env.DB_PORT;
const dbName = (config.has('db.name')) ? config.get('db.name') : process.env.DB_NAME;
const dbUsername = (config.has('db.username')) ? config.get('db.username') : process.env.DB_USERNAME;
const dbPassword = (config.has('db.password')) ? config.get('db.password') : process.env.DB_PASSWORD;

const pgPool = new pg.Pool({
    host: dbHost,
    port: dbPort,
    database: dbName,
    user: dbUsername,
    password: dbPassword,
});

app.use(morgan('dev'));
app.use(bodyParser.json({ type: 'application/json' }));
app.use(session({
    store: new (require('connect-pg-simple')(session))({
        pool: pgPool,
    }),
    secret: secret,
    resave: false,
    cookie: {
        httpOnly: true,
        maxAge: maxAge
    }
}));

// Proxy backend api
app.use('/api', createProxyMiddleware({
    target: `${api}`,
    changeOrigin: true,
    pathRewrite: {
        [`^/api`]: '',
    },
    onProxyReq(proxyReq, req, res) {
        proxyReq.setHeader('Authorization', `${req.session['token_type'] || ''} ${req.session.token || ''}`);
    }
}));

// Serve static web
app.use('/', routes);
app.get('*', function(req, res) {
    res.sendFile(path.join(__dirname, 'dist/index.html'))
});


app.post('/server/login', function (req, res, next) {
    axios.post(`${api}/auth/login`, req.body)
        .then(function (result) {
            req.session['token_type'] = result.data.data['token_type'];
            req.session.token = result.data.data.token;
            res.status(result.status).send(result.data);
        })
        .catch(function (err) {
            if (err.response) {
                res.status(err.response.status).send(err.response.data);
            } else {
                next(err);
            }
        });
});

app.get('/server/logout', function (req, res, next) {
    req.session.destroy();
    next();
});

app.listen(port, () => {
    console.log(`Server listening on port ${port}`)
});