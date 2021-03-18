#!/bin/sh
sleep 5
ls -l
diesel setup
diesel migration run
