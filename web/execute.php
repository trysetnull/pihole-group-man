<!DOCTYPE html>
<html lang="en">
<head>
  <title>pihole-group-man</title>
  <style>
    pre {
      font-size: 26pt;
      font-weight: bold;
    }
  </style>
</head>
<body>

<?php
if ($_SERVER["REQUEST_METHOD"] == "POST") {
  echo "<pre>";
  if (isset($_POST['action']) && $_POST['action'] == 'on') {
    $output = shell_exec('pihole-group-man -vvv remove 2>&1');
    echo "$output";
    if (str_contains($output, 'SUCCESS')) {
      $output = shell_exec('sudo pihole restartdns reload-lists 2>&1');
      echo "$output";
    }
  } elseif (isset($_POST['action']) && $_POST['action'] == 'off') {
    $output = shell_exec('pihole-group-man -vvv append 2>&1');
    echo "$output";
    if (str_contains($output, 'SUCCESS')) {
      $output = shell_exec('sudo pihole restartdns reload-lists 2>&1');
      echo "$output";
    }
  }
  echo "</pre>";
}
?>

</body>
</html>
