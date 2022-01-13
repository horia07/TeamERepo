#!/bin/sh

/phoronix-test-suite/phoronix-test-suite install fs-mark
/phoronix-test-suite/phoronix-test-suite install postmark
#/phoronix-test-suite/phoronix-test-suite install dbench
/phoronix-test-suite/phoronix-test-suite install ior 
/phoronix-test-suite/phoronix-test-suite batch-benchmark ext4-test-suite

cd /var/lib/phoronix-test-suite/test-results
bash /usr/src/app/rename.sh
/phoronix-test-suite/phoronix-test-suite result-file-to-pdf 001
rm -r 0*
cp /root/* /var/lib/phoronix-test-suite/test-results/




