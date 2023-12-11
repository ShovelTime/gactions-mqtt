
import * as http from "http";
import body from 'body-parser';
import express, {Express, Request, Response} from "express";
import dotenv from 'dotenv';
import qs from 'qs';
import oauth2, {AuthorizationServer, OAuthClient} from "@jmondi/oauth2-server";

const client_id = ""; // get that from the google stuff.
const redirection_url = ""; //ditto.
const env = dotenv.config();

const oauthserver : oauth2.AuthorizationServer = new oauth2.AuthorizationServer();

oauth2.enableGrantType({
  grant: "authorization_code",
  userRepository,
  authCodeRepository,
});

const app = express();
app.settings('query parser',
  (str : string) => qs.parse(str, { /* custom options */ }))

app.use(body.urlencoded({ extended: true }));
 
app.use(body.json());


app.get('/oauth/token', function(req, res) {
  const q_client_id : string = req.query.client_id as string;
  const q_re_uri : string = req.query.redirect_uri as string;
  const requested_scope : string = req.query.scope as string;

  if(q_re_uri !== redirection_url || q_client_id !== client_id)
  {
    res.status(403).send("clientID or Redirection mismatch!");
  }
  


})

app.get('/', oauth2.authorise(), function (req, res) {
    res.send('Secret area');
  });

app.listen(process.env.port);


