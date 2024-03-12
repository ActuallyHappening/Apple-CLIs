let data = xcrun simctl list --json | from json

let list = $data | get devices | transpose runtime-id device-info

let runtime_ids = $list | get runtime-id | flatten
save_data_json runtime-ids $runtime_ids

let device_info = $list | get device-info | flatten

save_data_json names ($device_info | get name)

let availability_errors = $device_info | get availabilityError? --ignore-errors | filter { |x| $x != null } | uniq
save_data_json "availability-errors" $availability_errors

def save_data_json [name: string, data: any] {
	let $file_name = $"($name).json"
	open $file_name | append $data | filter { |x| $x != null } | uniq | save -f $file_name
}