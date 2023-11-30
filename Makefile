all:
	sbatch jobs/bprMaxUniform.sh
	sbatch jobs/bprSumUniform.sh
	sbatch jobs/bprCovUniform.sh
	sbatch jobs/bprMaxNetwork.sh
	sbatch jobs/bprSumNetwork.sh
	sbatch jobs/bprCovNetwork.sh

test:
	sbatch jobs/bprTest.sh