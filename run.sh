#! bash

set -e

function get_input() {
  local -r year=$(printf "%d" "$1");
  local -r day=$(printf "%d" "$2");

  local -r scriptdir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
  local session_cookie
  session_cookie=$(cat "$scriptdir"/.session)

  local output
  output=$(curl "https://adventofcode.com/$year/day/$day/input"\
           -H "Cookie: $session_cookie")

  echo "$output"
}

function get_input_with_cache() {
  local -r year=$(printf "%d" "$1");
  local -r day=$(printf "%d" "$2");

  local -r scriptdir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
  mkdir -p "$scriptdir"/.inputs
  local -r filename="$scriptdir"/.inputs/${year}_${day}.txt

  if [[ -f $filename ]]; then
    cat "$filename"
  else
    get_input "$year" "$day" > "$filename"
    cat "$filename"
  fi
}

if ! hash docopts
then
echo "docopts must be available"
exit 1
fi

##? Usage:
##?   run.sh (-h|--help)
##?   run.sh test <year> [<day> [<puzzle_number>]] [--implem=<lang>]
##?   run.sh run <year> <day> [<puzzle_number>] [--implem=<lang>]
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

  if [[ $is_test == "true" ]]
  then
      local filter=""
      if [[ $year ]]; then filter=$filter"y$year"; fi 
      if [[ $day ]]; then filter=$filter"::day_$(printf "%02d" "$day")"; fi 
      cd "$(dirname "$0")/rust"
      cargo test "$filter"
  else
      local input
      input=$(get_input_with_cache "$year" "$day")
      cd "$(dirname "$0")/rust"
      echo "$input" | cargo run --release -- "$year" "$day" "$number"
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
