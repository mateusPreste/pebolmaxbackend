#!/bin/bash

# Loop 5 times to revert migrations
for i in {1..6}
do
  echo "Running sqlx migrate revert (Attempt $i of 5)"
  sqlx migrate revert

  # Check if the command was successful
  if [ $? -ne 0 ]; then
    echo "Error encountered during revert attempt $i. Aborting."
    exit 1 # Exit the script with an error code
  fi
done

echo "Successfully reverted 5 migrations."
exit 0 # Exit the script successfully
