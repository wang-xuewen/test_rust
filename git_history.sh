#!/bin/bash

git log -n 5 --pretty=format:"%h - %an, %ad : %s" origin/main
