const express = require('express')
const app = express()
const path = require('path');
const morgan = require('morgan')
const config = require('config')
const { createProxyMiddleware } = require('http-proxy-middleware')

const port = config.get('app.port') || process.env.APP_PORT || 3000
const apiBackendHost = config.get('api.backend.host') || process.env.API_BACKEND_HOST
const apiBackendPort = config.get('api.backend.port') || process.env.API_BACKEND_PORT
const apiBackendPath = config.get('api.backend.path') || process.env.API_BACKEND_PATH
const api = `http://${apiBackendHost}:${apiBackendPort}/${apiBackendPath}`

app.use(morgan('dev'))

// Proxy backend api
app.use('/api', createProxyMiddleware({
    target: `${api}`,
    changeOrigin: true,
    pathRewrite: {
        [`^/api`]: '',
    }
}))

// Serve static web
app.use('/', express.static(path.join(__dirname, 'dist')))
app.get('*', function(req, res) {
    res.sendFile(path.join(__dirname, 'dist/index.html'))
})

app.listen(port, () => {
    console.log(`Server listening on port ${port}`)
})