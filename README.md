# lunisolar-api

This is a wrapper around https://github.com/magiclen/chinese-lunisolar-calendar to provide solar to lunisolar date conversions.

This Rust code is compiled to WebAssembly and is running on Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Usage

### Getting the lunar month and day for a solar date

```
$ curl 'https://create-chinese-calendar-events.ackerleytng.workers.dev/solar-to-lunar/2000/1/1'
{"month":11,"day":25}
$
```

### Getting the solar year, month and day for a lunar date

```
$ curl https://create-chinese-calendar-events.ackerleytng.workers.dev/lunar-to-solar/2000/1/1
[{"year":2000,"month":2,"day":5}]
$
```

And when the lunar date is in a leap month, you get two solar dates!

```
$ curl https://create-chinese-calendar-events.ackerleytng.workers.dev/lunar-to-solar/2020/4/1
[{"year":2020,"month":4,"day":23},{"year":2020,"month":5,"day":23}]
```

## How I'm using this

I'm calling this from a Scenario at [make.com](https://www.make.com) which would

+ Run yearly on 1st January
+ Convert a list of lunar dates important to me
+ Create calendar events on my personal Google calendar

## Developing

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev --verbose

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

Docs are here: https://docs.rs/worker
