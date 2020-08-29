#!/bin/bash

# This is a bad way to do this but whatever
echo "$(date): $(ssh -i $1 -o StrictHostKeyChecking=no ec2-user@ctf.mrnice.dev 'docker ps | grep mgb | wc -l') players"
