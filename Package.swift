// swift-tools-version: 6.0

import PackageDescription

let package = Package(
  name: "clickandmeow",
  platforms: [
    .iOS(.v12),
    .macOS(.v10_15),
  ],
  products: [
    .library(
      name: "ClickAndMeow",
      targets: ["ClickAndMeow"])
  ],
  targets: [
    .binaryTarget(
      name: "clickandmeowFFI",
      url:
        "https://github.com/LiterateInk/ClickAndMeow/releases/download/1.0.0/clickandmeowFFI.xcframework.zip",
      checksum: "1909a64aae9a31baf6b9b249d46d656928ac42374215fbc7849f802b27e375db"),
    .target(
      name: "ClickAndMeow",
      dependencies: [.target(name: "clickandmeowFFI")],
      path: "swift"),
  ]
)
