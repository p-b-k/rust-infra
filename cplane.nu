########################################################################################################################
# Set up environment for using nushell
########################################################################################################################

$env.PATH = $env.PATH | append ([(pwd) target/debug] | str join /)

# const core_server = cat 
# 
def "start cplane" [ ] {
  print "Starting Control Plane"
  ./cplane start
}

def "stop cplane" [ ] { 
  print "Stopping Control Plane"
  ./cplane stop
}

def "build cplane" [ ] {
  cargo test
  cargo build -r

  podman build -t rusty-base  -f Dockerfile.base   ..
  podman build -t db-base     -f Dockerfile.mysql  ..
  podman build -t cplane-base -f Dockerfile.cplane ..
  podman build -t ui-base     -f Dockerfile.ui ..
}

def "list images" [--all] {
  # podman image ls --noheading | lines
  if $all {
    podman image ls -a --format json | from json
  } else {
    podman image ls --format json | from json
  }
}

def "list containers" [--all] {
  # podman image ls --noheading | lines
  if $all {
    podman container ls -a --format json | from json
  } else {
    podman container ls --format json | from json
  }
}

def "exec image" [name] {
  podman run -it $name bash -i
}

def "exec container" [name] {
  podman exec -it $name bash
}

def "pod ps" [--fields = [ "Names", "Image" ]] {

  podman ps --format json | from json
}

const docker_config_file = "play/play.yaml"

def "pod start" [ service = "all"; --config = $docker_config_file ] {
  print $"starting pods, using config ($config)"
  if $service == "all" {
    podman-compose -f $config up -d
  } else {
    podman-compose -f $config start $service
  }
}

def "pod stop" [ service = "all"; --config = $docker_config_file ] {
  print $"stopping pods, using config ($config)"
  if $service == "all" {
    podman-compose -f $config down
  } else {
    podman-compose -f $config stop $service
  }
}

