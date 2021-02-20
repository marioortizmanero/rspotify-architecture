# Rspotify's future architecture

Here's a draft for [Rspotify](https://github.com/ramsayleung/rspotify)'s
possible future architecture, compared to the current one, as of 2021/02/21.

See [rspotify/#173](https://github.com/ramsayleung/rspotify/issues/173) to learn
more about the discussion of the architecture.

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

## Possible improvements

* Converting between the different clients?
* Macros to implement `BaseClient`, `BaseEndpoints`, `OAuthClient` and
  `OAuthClient`?
