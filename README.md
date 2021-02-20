# Rspotify's future architecture

Here's a draft for [Rspotify](https://github.com/ramsayleung/rspotify)'s
possible future architecture, compared to the current one, as of 2021/02/21. It
separates the `Spotify` client into multiple ones, depending on what
authentication method is being used for the initialization.

See [rspotify/#173](https://github.com/ramsayleung/rspotify/issues/173) to join
the discussion.

## Advantages

* Type safety to separate between the authentication methods (i.e. you can't
  call an user-authenticated endpoint if the authentication process is the basic
  one). With this the `InvalidAuth` variant may be removed.
* Improved flexibility as much as possible for future clients and authentication
  methods.
* The code is more nicely distributed. The `client.rs` file is currently 2200
  lines long. Endpoints and methods for different authentication methods are
  also mixed up in the same files.
* Allows to get rid of the builder pattern for the clients, as they can be
  initialized with custom methods instead of with a single client. The builder
  pattern could actually be removed completely as it's not really necessary for
  `Token`, `Credentials`, etc (to be considered).

## Disadvantages

* Requires a `prelude` module to have quick access to the traits in order to use
  the endpoints (see the `BaseEndpoints` and `OAuthEndpoints` traits).

  This might not be that big of a deal because we can take advantage of it and
  also include the basic parts of the library in there (the client structs, auth
  structs like `Token` or `Credentials`, and similars) for easy access. It's
  also a very common pattern in Rust.
* The `Token` unwrap issue in the client is still unsolved. This means that if
  the user attempts to call an endpoint before authenticating for the first time
  (`client.token == None`), an `unwrap` will cause a panic.

  This is not a big deal because it's a developer error and easy to discover
  before deployment. If the dev read the docs this most likely won't happen.

  Anyhow, if this could be solved with type safety at compile time it would be
  awesome. This could work by having an authenticator object that returns the
  actual client:

  * `Authenticator::new()`
  * `Authenticator::prompt_for_user_token() -> OAuthClient`
  * `OAuthClient::endpoint()`

  This is currently quite hard to implement because the authenticator may be
  required multiple times, like in the case of an automatically refreshing
  token. The method that returns the client (`prompt_for_user_token` for
  example) is required to end the lifetime of `Authenticator` because the HTTP
  client has to be moved to the new client, so it can't be reused.

  To use a single HTTP client throughout the entire execution of the program,
  it would have to be passed from the Spotify client to the Authenticator and
  back continuously, which doesn't seem trivial.

## Possible improvements

* Converting between the different clients?
* Macros to implement `BaseClient`, `BaseEndpoints`, `OAuthClient` and
  `OAuthClient`?
* When this is final, a diagram of Rspotify's architecture should be made for
  new contributors.
