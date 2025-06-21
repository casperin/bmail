
## Mitid Test user

Created from: https://pp.mitid.dk/test-tool/frontend/#/create-identity

When creating, make sure to pick IAN: High (no, I don't know why)

* Elton9303 / mandengikentur


## Signicat

* signicat@mail.gormcasper.dk
* Account ID: ???

* https://developer.signicat.com/identity-methods/mitid/integration-guide/oidc-mitid/
* https://developer.signicat.com/identity-methods/mitid/integration-guide/auth-api-mitid/
* https://dashboard.signicat.com/?scopeId=a-spge-U5AoeOLh9dXRpheviKe7
* https://jazzreader.sandbox.signicat.com/auth/open/.well-known/openid-configuration
  * https://jazzreader.sandbox.signicat.com/auth/open/connect/authorize

```js
{
  client_id: "sandbox-substantial-narwal-354",
  response_type: "code",
  scope: "openid profile",
  prompt: "login",
  acr_values: "idp:mitid",
  redirect_uri: "https://jazzreader.dk/signicat",
}
```

* https://jazzreader.dk/signicat?code=ABB4C4FFA12976AEC40C959E638CDC864AA5BDFA31DD883CE6C314FE2256D258-1&scope=openid%20profile&session_state=WU4fENepp7H2MwwOnpcGOsk6bo8g-2ot5krX676i-EQ.94143480261D82FEC877E467E0F1EBF8&iss=https%3A%2F%2Fjazzreader.sandbox.signicat.com%2Fauth%2Fopen


### API Client

Secret name: identify mitid
Client ID: sandbox-gaudy-collar-987
Client secret: ughD3KR47yRJENmyaxmOcqYwD1xOa2vGsJRFM624wbJh4Lv2
