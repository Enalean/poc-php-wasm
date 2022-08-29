<?php
$ffi = FFI::load(__DIR__ . '/wasmtimewrapper.h');

if ($argc != 3) {
    print "Usage: " . $argv[0] . " <filename> <json>\n";
    return 1;
}
$json_in = file_get_contents($argv[2]);

$out = $ffi->compile_and_exec($argv[1], $json_in);
$json_out = json_decode($out);

print "<>PHP Output JSON Object: \n";
var_dump($json_out);
