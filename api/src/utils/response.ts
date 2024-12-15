export const successResponse = (data: any, message: string = 'Success') => {
    return { status: 'success', message, data };
  };
  
  export const errorResponse = (error: string) => {
    return { status: 'error', message: error };
  };
  