group "default" {
  targets = ["app", "chart"]
}

target "app" {
  context = "."
  dockerfile = "Dockerfile.ess"
  target = "runtime_image"
  platforms = ["linux/amd64"]
}

target "chart" {
  context = "."
  dockerfile = "Dockerfile.ess"
  target = "chart_archive"
  platforms = ["linux/amd64"]
  output = ["type=local,dest=out/chart"]
}

