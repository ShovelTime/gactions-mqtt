
import * as http from "http";
import body from 'body-parser';
import express, {Express, } from "express";
import dotenv from 'dotenv';

const oauth = require('node-oauth2-server');



const env = dotenv.config();

const app = express();

app.use(body.urlencoded({ extended: true }));
 
app.use(body.json());

const oauth2 = oauth({
    model: {},
    grants: ['password'], // needs more than that
    debug: true
});

app.all('/oauth/token', oauth2.grant())

app.get('/', oauth2.authorise(), function (req, res) {
    res.send('Secret area');
  });
   
app.use(oauth2.errorHandler());

app.listen(process.env.port);


