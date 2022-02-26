export const useReducer = (reducer, initialState = {}) => {
    let state = initialState;

    const dispatch = (action) => {
        state = reducer(state, action);
    }

    return [state, dispatch];
}
