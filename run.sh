#! bash

set -e

if ! hash docopts
then
echo "docopts must be available"
exit 1
fi

##? Usage:
##?   run.sh (-h|--help)
##?   run.sh (test|run) <year> [<day> [<puzzle_number>]] [--implem=<lang>]
##?
##? Options:
##?   --implem=<lang>  Implementation of the solution [default: rust].
##?
##? Examples:
##?   run.sh run 2021 12

usage=$(grep "^##?" "$0" | cut -c 5-)
args={}
eval "$(docopts -A args -h "$usage" : "$@")"

implem_rust() {
  local is_test=$1
  local year=$2
  local day=$3
  local number=$4

  cd "$(dirname "$0")/rust"

  if [[ $is_test == "true" ]]
  then
      local filter=""
      if [[ $year ]]; then filter=$filter"y$year"; fi 
      if [[ $day ]]; then filter=$filter"::day_$(printf "%02d" "$day")"; fi 
      cargo test "$filter"
  else
      cargo run "$year" "$day" "$number" --release
  fi
}


case ${args["--implem"]} in

  "rust")
  implem_rust ${args["test"]} ${args["<year>"]} ${args["<day>"]} ${args["<puzzle_number>"]}
  ;;

  *)
  echo "Unhandled implementation lang ${args["--implem"]}"
  exit 1
  ;;

esac
