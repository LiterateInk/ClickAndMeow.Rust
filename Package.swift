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
        "https://github.com/LiterateInk/ClickAndMeow/releases/download/0.0.0/clickandmeowFFI.xcframework.zip",
      checksum: "placeholder-checksum-until-first-release"),
    .target(
      name: "ClickAndMeow",
      dependencies: [.target(name: "clickandmeowFFI")],
      path: "swift"),
  ]
)
