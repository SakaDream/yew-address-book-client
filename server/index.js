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

const routes = require('./routes');

const secret = (process.env.APP_COOKIE_SECRET) ? process.env.APP_COOKIE_SECRET : (config.has('app.cookie.secret')) ? config.get('app.cookie.secret') : require("crypto").randomBytes(16).toString("hex");
const port = (process.env.APP_PORT) ? process.env.APP_PORT : (config.has('app.port')) ? config.get('app.port') : 3000;
const maxAge = (process.env.APP_MAX_AGE) ? process.env.APP_MAX_AGE : (config.has('app.max-age')) ? config.get('app.max-age') : 60 * 60 * 24 * 7;

const apiBackendHost = (process.env.API_BACKEND_HOST) ? process.env.API_BACKEND_HOST : (config.has('api.backend.host')) ? config.get('api.backend.host') : '';
const apiBackendPort = (process.env.API_BACKEND_PORT) ? process.env.API_BACKEND_PORT : (config.has('api.backend.port')) ? config.get('api.backend.port') : '';
const apiBackendPath = (process.env.API_BACKEND_PATH) ? process.env.API_BACKEND_PATH : (config.has('api.backend.path')) ? config.get('api.backend.path') : '';

const api = `http://${apiBackendHost}:${apiBackendPort}/${apiBackendPath}`;

const dbHost = (process.env.DB_HOST) ? process.env.DB_HOST : (config.has('db.host')) ? config.get('db.host') : '';
const dbPort = (process.env.DB_PORT) ? process.env.DB_PORT : (config.has('db.port')) ? config.get('db.port') : '';
const dbName = (process.env.DB_NAME) ? process.env.DB_NAME : (config.has('db.name')) ? config.get('db.name') : '';
const dbUsername = (process.env.DB_USERNAME) ? process.env.DB_USERNAME : (config.has('db.username')) ? config.get('db.username') : '';
const dbPassword = (process.env.DB_PASSWORD) ? process.env.DB_PASSWORD : (config.has('db.password')) ? config.get('db.password') : '';

const pgPool = new pg.Pool({
    host: dbHost,
    port: dbPort,
    database: dbName,
    user: dbUsername,
    password: dbPassword,
});

// Proxy backend api
const options = {
    target: `${api}`,
    changeOrigin: true,
    pathRewrite: {
        [`^/api`]: '',
    },
    onProxyReq(proxyReq, req) {
        let tokenType = 'bearer';
        let token = '';
        if (req.session) {
            if (req.session['token_type']) {
                tokenType = req.session['token_type'];
            }
            if (req.session.token) {
                token = req.session.token;
            }
        }
        proxyReq.setHeader('Authorization', `${tokenType} ${token}`);
    }
  };
const apiProxy = createProxyMiddleware('/api', options);
app.use(apiProxy);

app.use(morgan('dev'));
app.use(bodyParser.json({ type: 'application/json' }));
app.use(session({
    store: new (require('connect-pg-simple')(session))({
        pool: pgPool,
        createTableIfMissing: true,
    }),
    secret: secret,
    resave: false,
    cookie: {
        httpOnly: true,
        maxAge: maxAge
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
    console.log(`Server listening on port ${port}`);
    console.log(`Environment: ${process.env.NODE_ENV}`);
});