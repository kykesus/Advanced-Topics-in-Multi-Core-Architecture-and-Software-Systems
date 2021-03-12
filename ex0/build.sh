docker run -it -v /${PWD}/artifacts:/parsort/artifacts parsort:1.0.0
cp /${PWD}/artifacts/debug/parsort parsort
rm -rf /${PWD}/artifacts