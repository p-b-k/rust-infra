########################################################################################################################
# Set up environment for using nushell
########################################################################################################################

$env.PATH = $env.PATH | append ([(pwd) target/debug] | str join /)
$env.PROJ_ROOT = $env.PWD

$env.profiles.ui = "BaseBlue"
$env.profiles.cp = "BaseBlack"

# const core_server = cat 
# 
def "start cp" [
  --log = warn
] {
  $env.RUST_LOG = $log

  if $env.TERM == xterm-kitty {
    let r = (hyprctl dispatch focuswindow class:cplane_svr)
    if $r == ok {
      echo "CPlane Already Started"
    } else {
      kitty --class=cplane_svr -T "Control Plane Server" --detach cp-svr
    }
  } else {
    ( mate-terminal --working-directory $env.PROJ_ROOT --profile $env.profiles.cp
      -e target/debug/cp-svr
      -t "CP"
      --geometry 132x16+0-0
    )
  }
}

def "start ui" [
  --log: string = warn
] {
  $env.RUST_LOG = $log
  if $env.TERM == xterm-kitty {
    let r = (hyprctl dispatch focuswindow class:ui_svr) 
    if $r == ok {
      echo "UI Already Started"
    } else {
      kitty --class=ui_svr -T "Control Plane UI" --detach ui-svr
    }
  } else {
    ( mate-terminal --working-directory $env.PROJ_ROOT --profile $env.profiles.ui
      -e target/debug/ui-svr
      -t "UI"
      --geometry 132x16+0-360
    )
  }
}

def "build cplane" [ ] {
  cargo build
  cargo test
  [ infra cplane svr/ui-svr svr/cp-svr ] | each { |p| cargo install --path=$"rust/($p)" } 
  
  # cargo test
  # cargo build -r

  # podman build -t rusty-base  -f Dockerfile.base   ..
  # podman build -t db-base     -f Dockerfile.mysql  ..
  # podman build -t cplane-base -f Dockerfile.cplane ..
  # podman build -t ui-base     -f Dockerfile.ui ..
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

