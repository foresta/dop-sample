#!/bin/bash

SQLDEF_VERSION="v0.16.10"
PROJECT_ROOT=`git rev-parse --show-toplevel`

mkdir -p $PROJECT_ROOT/bin

$PROJECT_ROOT/bin/mysqldef --help &> /dev/null 
if [ $? -ne 0 ] ; then
  echo "[START] install mysqldef" 
else
  echo "[SKIP] mysqldef is already installed."
  exit
fi

OS=$(uname -s | tr '[A-Z]' '[a-z]')
PLATFORM=$(uname -m)

if [ $OS = "darwin" ] || [ $OS = "windows" ] ; then
    FILENAME=mysqldef_${OS}_${PLATFORM}.zip
    curl -L -O https://github.com/k0kubun/sqldef/releases/download/${SQLDEF_VERSION}/${FILENAME}
    unzip ${FILENAME}
    rm ${FILENAME}
    mv mysqldef $PROJECT_ROOT/bin/
else
    FILENAME=mysqldef_${OS}_${PLATFORM}.tar.gz
    curl -L -O https://github.com/k0kubun/sqldef/releases/download/${SQLDEF_VERSION}/${FILENAME}
    tar xf ${FILENAME}
    rm ${FILE_NAME}
    mv mysqldef ${PROJECT_ROOT}/bin/
fi

echo "[FINISH] install mysqldef."
