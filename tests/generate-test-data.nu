let data = xcrun simctl list --json | from json

let list = $data | get devices | transpose runtime-id device-info


let runtime_ids = $list | get runtime-id | flatten
open runtime-ids.json | append $runtime_ids | uniq | save -f runtime-ids.json


let names = $list | get device-info | flatten | get name
open names.json | append $names | uniq | save -f names.json

let availability_errors = $list | get device-info | flatten | get availabilityError
open availability_errors.json | append $availability_errors | uniq | save -f availability_errors.json