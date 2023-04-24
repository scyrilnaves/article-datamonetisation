Security Config:

Inherits WebSecurityConfigurerAdapter
================
Add CORS Filter
Disable CSRF
Session Management = Stateless
Authorize Requests = /login
exception handling= disable
Configure Boot as Oauth2ResourceServer ---> 
BearerTokenAuthenticationFilter = Filter Responsible For Authenticating Requests with JWTs in Header
 || 
JwtAuthenticationProvider = Bearer Token Authentication Filter delegates the actual authentication of JWTs to this class via Authentication Provider
 ||
JWTDecoder = Component used by JWT Authentication Provider to decode and validate receive JWTs.
             Validation includes signature validation and JWT Expiry validations

Main Class AuthController for getting the JWT   