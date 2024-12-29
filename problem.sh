#! bash

function get_input() {
  local -r year=$(printf "%d" "$1");
  local -r day=$(printf "%d" "$2");

  local session_cookie
  session_cookie=$(cat .session)

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


