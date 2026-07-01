#import "@preview/zebra:0.1.0": qrcode

// us-business-card is 3.5in x 2in; flipped gives a 2in-wide portrait card.
#let h-margin = 0.2cm
#let card-width = 2in
#let inner-width = card-width - 2 * h-margin

#set page(
  paper: "us-business-card",
  flipped: true,
  margin: (top: 0.4cm, bottom: 0.2cm, x: h-margin),
)
#set text(font: "Atkinson Hyperlegible Next")
#show raw: set text(font: "Berkeley Mono")

#let ssid = sys.inputs.ssid
#let password = sys.inputs.password
#let authtype = sys.inputs.at("authtype", default: "WPA2")
#let location = sys.inputs.at("location", default: "")

// Escape the chars the WiFi QR grammar treats as delimiters so SSIDs/passwords
// containing them still scan correctly.
#let esc(s) = s.replace("\\", "\\\\").replace(";", "\\;").replace(",", "\\,").replace(":", "\\:")
#let payload = "WIFI:S:" + esc(ssid) + ";T:" + authtype + ";P:" + esc(password) + ";;"

// Shrink a line to fit within `width`, leaving it untouched when it already fits,
// so an arbitrarily long SSID or password never overflows the card.
#let fit(width, body) = context {
  // `width` may carry an em term (the icon column is sized in em), so resolve it
  // against the ambient text size before dividing by the measured absolute width.
  let avail = width.to-absolute()
  let natural = measure(body).width
  let factor = calc.min(1.0, avail / natural)
  scale(body, x: factor * 100%, y: factor * 100%, origin: left + horizon, reflow: true)
}

#set align(center)
#v(1fr)

#if location != "" {
  fit(inner-width, text(size: 15pt, weight: "bold")[#location])
  v(0.3cm)
}

#qrcode(payload, width: 4.2cm)
#v(0.2cm)

#set text(size: 15pt)
#let icon-size = 1em
#let gutter = 6pt
#let text-width = inner-width - icon-size - gutter
#grid(
  columns: (icon-size, auto),
  column-gutter: gutter,
  row-gutter: 6pt,
  align: left + horizon,
  image("wifi.svg", height: icon-size), fit(text-width, raw(ssid)),
  image("key.svg", height: icon-size), fit(text-width, raw(password)),
)

#v(1fr)
