
// Standard
#include <math.h>

// Matlab
#include "mex.h"

// Local
//#include "example_rustlib.h"
extern void nn_smooth(double* array, int length, int nn);

/**
*! Near-neighbor smoothing
**/
#define IS_REAL_2D_FULL_DOUBLE(P) (!mxIsComplex(P) && mxGetNumberOfDimensions(P) == 2 && !mxIsSparse(P) && mxIsDouble(P))

#define IS_REAL_SCALAR(P) (IS_REAL_2D_FULL_DOUBLE(P) && mxGetNumberOfElements(P) == 1)

#define IS_REAL_2D_FULL_INT(P) (!mxIsComplex(P) && mxGetNumberOfDimensions(P) == 2 && !mxIsSparse(P) && mxIsInt32(P))

#define IS_REAL_INT_SCALAR(P) (IS_REAL_2D_FULL_INT(P) && mxGetNumberOfElements(P) == 1)

void mexFunction(int nout, mxArray *p_out[], int nargs, const mxArray *p_args[]) {
    /* Macros for the ouput and input arguments */
    #define B_OUT p_out[0]
    #define A_IN p_args[0]
    #define NN_IN p_args[1]

    // Check the number of arguments
	if(nargs < 2 || 2 < nargs) {
		mexErrMsgTxt("Wrong number of input arguments.");
    } else if(nout > 1) {
		mexErrMsgTxt("Too many output arguments.");
    }
    
    // Check A
	if(!IS_REAL_2D_FULL_DOUBLE(A_IN)) {
		mexErrMsgTxt("A must be a real 2D full double array.");
    }
    
    int nn;
    // If P was specified, check that it is a real double scalar
	if(!IS_REAL_SCALAR(NN_IN)) {
		mexErrMsgTxt("P must be a real double scalar.");
	} else {
        // Get p
		nn = (int) mxGetScalar(NN_IN);
    }
    
    // Get the dimensions of A
    // rows?
	size_t N_rows = mxGetM(A_IN);
    // columns?
	size_t N_cols = mxGetN(A_IN);
    // Get the pointer to the data of A
	double* A = mxGetPr(A_IN);
    // Create the output matrix
	B_OUT = mxDuplicateArray(A_IN);
    // Get the pointer to the data of B
	double* B = mxGetPr(B_OUT);
    // Compute a matrix with normalized columns
	for(size_t col = 0; col < N_cols; col++) {
        nn_smooth(B + col * N_rows, N_rows, nn);
	}
	return;
}