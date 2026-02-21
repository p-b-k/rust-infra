########################################################################################################################
# Set up environment for using nushell
########################################################################################################################

$env.PATH = $env.PATH | append ([(pwd) target/debug] | str join /)

const cp_json_file = "cp.image.json"
const ui_json_file = "ui.image.json"
const w_json_file  = "w.image.json"

let servers = ( cat $"docker/($cp_json_file)" | from json
              | append ( cat $"docker/($ui_json_file)" | from json )
              | append ( cat $"docker/($w_json_file)"  | from json
                       | insert name w0
                       | update ports [ { "external" : 7300, "internal" : 8000 } ]
                       )
              | append ( cat $"docker/($w_json_file)"  | from json
                       | insert name w1
                       | update ports [ { "external" : 7301, "internal" : 8000 } ]
                       )
              )

# const core_server = cat 
# 
def "start cplane" [ ] {
  print "Starting Control Plane"

}

def "stop cplane" [ ] { 
  print "Stopping Control Plane"
}

def "build cplane" [ ] {
  cargo test
  cargo build -r

  podman build -t rusty-base  -f Dockerfile.base   ..
  podman build -t db-base     -f Dockerfile.mysql  ..
  podman build -t cplane-base -f Dockerfile.cplane ..
}

def "list images" [] {
  podman image ls --noheading | lines
}

# def "start sample db" [name] {
#   mkdir $"($env.HOME)/.db/($name)"

#   if true {
#     podman-compose -f docker/sampledb.yaml up -d
#   } else {
#     ( podman run --rm -d --name $name 
#       -e MYSQL_ROOT_PASSWORD=foobar
#       -p 3300:3306
#       -v $"($env.HOME)/.db/($name):/var/lib/mysql:Z"
#       db-base
#     )
#   }
# }

def "exec image" [name] {
  podman run -it $name bash -i
}

def "exec container" [name] {
  podman exec -it $name bash
}

def "pod ps" [...fields : string] {
  let f = if $fields == [] { [Names, Image] } else { $fields }

  podman ps --format json | from json
}
