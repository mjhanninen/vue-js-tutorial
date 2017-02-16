# Vue.js tutorial

This is an "etude" I wrote to study and understand the basics of [Vue.js][1].
It follows heavily [this tutorial][2] on Scotch.io.

[1]: https://vuejs.org
    "Vue.js -- The Progressive JavaScript Framework"

[2]: https://scotch.io/tutorials/build-an-app-with-vue-js-a-lightweight-alternative-to-angularjs
    "Build an App with Vue.js: A Lightweight Alternative to AngularJS"

## Front-end

To start the Webpack development server with hot reloading:

```.sh
$ npm start
```

And then head to [http://localhost:8080/](http::localhost:8080/).

## Back-end

There is a very simple back-end written in Rust for testing purposes.  You can
boot it up as follows:

```.sh
$ cd backend/
$ cargo run
```

You probably need a pretty recent (i.e. nightly) Rust to build the back-end.
