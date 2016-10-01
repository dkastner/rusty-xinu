#!/bin/bash 

CMD=$1

PROJECT_DIR=/home/vagrant/xinu

vagrant scp . default:$PROJECT_DIR

function clean {
  echo "CLEANING"
  vagrant ssh default -c "cd $PROJECT_DIR; make -d clean"
}

if [ "$CMD" == "clean" ]; then
  clean
else
  clean
  echo "BUILDING"
  vagrant ssh default -c "cd $PROJECT_DIR; make -d all iso"
  vagrant scp default:$PROJECT_DIR/build/os.iso .
fi
