#!/bin/bash

echo "Running sqlx migrate run..."
sqlx migrate run

# Check if the command was successful
if [ $? -ne 0 ]; then
  echo "Error encountered during migration run. Aborting."
  exit 1 # Exit the script with an error code
fi

echo "Successfully applied pending migrations."
exit 0 # Exit the script successfully
